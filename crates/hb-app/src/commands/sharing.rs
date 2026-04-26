use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    error::{CmdResult, cmd_err},
    store::DataStore,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShareSettings {
    pub enabled: bool,
    /// Paths (relative to collection root) that are downloadable. Empty = all files.
    pub allowed_paths: Vec<String>,
    /// Speed cap in KB/s. None = unlimited.
    pub speed_cap_kbps: Option<u32>,
    /// Max simultaneous downloads allowed. None = unlimited.
    pub download_limit: Option<u32>,
    /// If true, only peers you follow can download.
    pub require_follow: bool,
}

#[tauri::command]
pub async fn get_share_settings(
    slug: String,
    store: State<'_, DataStore>,
) -> CmdResult<ShareSettings> {
    Ok(store.load_share_settings(&slug).map_err(cmd_err)?.unwrap_or_default())
}

#[tauri::command]
pub async fn save_share_settings(
    slug: String,
    settings: ShareSettings,
    store: State<'_, DataStore>,
) -> CmdResult<()> {
    store.save_share_settings(&slug, &settings).map_err(cmd_err)
}

/// Request a file download from a peer's collection.
/// Stub: actual transfer requires iroh P2P transport (Phase 2).
#[tauri::command]
pub async fn request_download(
    _peer_hb_id: String,
    _slug: String,
    _path: String,
) -> CmdResult<()> {
    Err("Direct file transfer requires a live P2P connection (iroh). This will be available once the peer is reachable.".into())
}
