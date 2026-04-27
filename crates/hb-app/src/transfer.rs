//! iroh-based P2P file transfer.
//!
//! Protocol (`/hoardbook/xfer/1`):
//!   Client → Server  [u32-LE request-len] [JSON XferRequest]
//!   Server → Client  [u8 status: 0=ok 1=error]
//!     ok    → [u64-LE file-size] [file bytes]
//!     error → [u32-LE msg-len]  [UTF-8 error message]

use std::path::Path;

use anyhow::{Context, Result, anyhow};
use iroh::{Endpoint, EndpointAddr};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::store::DataStore;

pub const XFER_ALPN: &[u8] = b"/hoardbook/xfer/1";

#[derive(Serialize, Deserialize)]
struct XferRequest {
    slug: String,
    path: String,
}

// ---------------------------------------------------------------------------
// Server — runs as a background task on the local iroh endpoint
// ---------------------------------------------------------------------------

pub async fn run_server(endpoint: Endpoint, store: DataStore) {
    loop {
        let incoming = match endpoint.accept().await {
            Some(inc) => inc,
            None => {
                tracing::debug!("iroh endpoint closed — transfer server exiting");
                break;
            }
        };

        let store_clone = store.clone();
        tokio::spawn(async move {
            let accepting = match incoming.accept() {
                Ok(a) => a,
                Err(e) => {
                    tracing::debug!("iroh incoming reject: {e}");
                    return;
                }
            };

            let conn = match accepting.await {
                Ok(c) => c,
                Err(e) => {
                    tracing::debug!("iroh handshake error: {e}");
                    return;
                }
            };

            if let Err(e) = handle_connection(conn, store_clone).await {
                tracing::warn!("transfer session error: {e}");
            }
        });
    }
}

async fn handle_connection(
    conn: iroh::endpoint::Connection,
    store: DataStore,
) -> Result<()> {
    let (mut send, mut recv) = conn.accept_bi().await.context("accept_bi")?;

    // Read request
    let req_len = recv.read_u32_le().await.context("read req len")?;
    if req_len > 64 * 1024 {
        return Err(anyhow!("request too large"));
    }
    let mut req_bytes = vec![0u8; req_len as usize];
    recv.read_exact(&mut req_bytes).await.context("read req")?;
    let req: XferRequest = serde_json::from_slice(&req_bytes).context("parse request")?;

    // Load share settings
    let settings = store
        .load_share_settings(&req.slug)
        .context("load share settings")?
        .unwrap_or_default();

    if !settings.enabled {
        return send_error(&mut send, "Sharing is disabled for this collection").await;
    }

    if !is_allowed_path(&req.path, &settings.allowed_paths) {
        return send_error(&mut send, "File is not in the allowed download paths").await;
    }

    let root = match settings.root_path {
        Some(p) => p,
        None => return send_error(&mut send, "Collection root path not configured on sharer's end").await,
    };

    // Build and validate path (prevent traversal)
    let rel = Path::new(&req.path);
    if rel.is_absolute()
        || rel.components().any(|c| c == std::path::Component::ParentDir)
    {
        return send_error(&mut send, "Invalid file path").await;
    }
    let file_path = Path::new(&root).join(rel);

    if !file_path.is_file() {
        return send_error(&mut send, "File not found").await;
    }

    // Stream file
    let file = tokio::fs::File::open(&file_path).await.context("open file")?;
    let file_size = file.metadata().await.context("metadata")?.len();

    send.write_u8(0).await.context("write ok status")?;
    send.write_u64_le(file_size).await.context("write file size")?;

    let mut reader = tokio::io::BufReader::new(file);
    tokio::io::copy(&mut reader, &mut send).await.context("stream file")?;
    send.shutdown().await.context("shutdown send")?;

    Ok(())
}

fn is_allowed_path(path: &str, allowed: &[String]) -> bool {
    allowed.is_empty() || allowed.iter().any(|prefix| path.starts_with(prefix.as_str()))
}

async fn send_error(
    send: &mut iroh::endpoint::SendStream,
    msg: &str,
) -> Result<()> {
    let bytes = msg.as_bytes();
    send.write_u8(1).await.context("write error status")?;
    send.write_u32_le(bytes.len() as u32).await.context("write error len")?;
    send.write_all(bytes).await.context("write error msg")?;
    send.shutdown().await.context("shutdown")?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Client — called from the request_download command
// ---------------------------------------------------------------------------

/// Connect to a peer and download a single file.
/// Returns the number of bytes written.
pub async fn download_file(
    endpoint: &Endpoint,
    peer_addr_json: &str,
    slug: &str,
    path: &str,
    save_path: &str,
) -> Result<u64> {
    let peer_addr: EndpointAddr =
        serde_json::from_str(peer_addr_json).context("parse peer EndpointAddr")?;

    let conn = endpoint
        .connect(peer_addr, XFER_ALPN)
        .await
        .context("connect to peer")?;

    let (mut send, mut recv) = conn.open_bi().await.context("open_bi")?;

    // Send request
    let req = XferRequest { slug: slug.to_string(), path: path.to_string() };
    let req_bytes = serde_json::to_vec(&req).context("serialize request")?;
    send.write_u32_le(req_bytes.len() as u32).await.context("write req len")?;
    send.write_all(&req_bytes).await.context("write req")?;
    send.shutdown().await.context("shutdown send")?;

    // Read response
    let status = recv.read_u8().await.context("read status")?;

    if status != 0 {
        let err_len = recv.read_u32_le().await.context("read err len")?;
        let mut err_bytes = vec![0u8; err_len as usize];
        recv.read_exact(&mut err_bytes).await.context("read err")?;
        return Err(anyhow!(String::from_utf8_lossy(&err_bytes).into_owned()));
    }

    let file_size = recv.read_u64_le().await.context("read file size")?;

    // Create parent dirs and output file
    if let Some(parent) = Path::new(save_path).parent() {
        tokio::fs::create_dir_all(parent).await.context("create dirs")?;
    }
    let out = tokio::fs::File::create(save_path).await.context("create output file")?;
    let mut writer = tokio::io::BufWriter::new(out);

    let mut limited = (&mut recv).take(file_size);
    let written = tokio::io::copy(&mut limited, &mut writer).await.context("stream download")?;
    writer.flush().await.context("flush")?;

    conn.close(0u32.into(), b"");
    Ok(written)
}
