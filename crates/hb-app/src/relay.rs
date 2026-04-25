//! HTTP client for communicating with Hoardbook relays.
//!
//! Phase 1: queries all known relays in parallel; first successful response wins.
//! Direct connections (iroh) are deferred to Phase 2.

use anyhow::{anyhow, Context, Result};
use hb_core::{ChatMessage, SignedEnvelope};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::store::CachedPeer;

/// Hardcoded bootstrap relay URLs shipped in the binary.
const BOOTSTRAP_RELAYS: &[&str] = &[
    // TODO: add a community relay URL once one is hosted.
];

// ---------------------------------------------------------------------------
// Wire types — mirrors the relay's JSON responses
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct PeerResponse {
    profile: Option<SignedEnvelope>,
    #[serde(default)]
    collections: Vec<SignedEnvelope>,
    online: bool,
    node_addr: Option<String>,
}

#[derive(Debug, Serialize)]
struct PublishRequest<'a> {
    #[serde(rename = "type")]
    doc_type: &'a str,
    document: &'a SignedEnvelope,
}

// ---------------------------------------------------------------------------
// RelayClient
// ---------------------------------------------------------------------------

pub struct RelayClient {
    http: Client,
    /// Relay URLs to try, in priority order (bootstrap seeds first).
    relay_urls: std::sync::RwLock<Vec<String>>,
}

impl RelayClient {
    pub fn new(extra_relays: Vec<String>) -> Self {
        let mut relay_urls: Vec<String> = BOOTSTRAP_RELAYS
            .iter()
            .map(|s| s.to_string())
            .collect();
        relay_urls.extend(extra_relays);

        Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("failed to build HTTP client"),
            relay_urls: std::sync::RwLock::new(relay_urls),
        }
    }

    pub fn set_relay_urls(&self, urls: Vec<String>) {
        *self.relay_urls.write().unwrap() = urls;
    }

    /// Publish a signed envelope to all known relays.
    /// Succeeds if at least one relay accepts it.
    pub async fn publish(&self, doc_type: &str, envelope: &SignedEnvelope) -> Result<()> {
        let body = PublishRequest { doc_type, document: envelope };
        let mut last_err = anyhow!("no relays configured");
        let relay_urls = self.relay_urls.read().unwrap().clone();

        for url in &relay_urls {
            let endpoint = format!("{url}/v1/publish");
            match self.http.post(&endpoint).json(&body).send().await {
                Ok(resp) if resp.status().is_success() => return Ok(()),
                Ok(resp) => {
                    last_err = anyhow!(
                        "relay {} returned {}: {}",
                        url,
                        resp.status(),
                        resp.text().await.unwrap_or_default()
                    );
                }
                Err(e) => {
                    last_err = anyhow!("relay {} unreachable: {e}", url);
                    tracing::warn!("relay {url} unreachable: {e}");
                }
            }
        }

        Err(last_err)
    }

    /// Fetch a peer's cached profile and collections from the relay.
    /// Queries all known relays in parallel and returns the first success.
    pub async fn fetch_peer(&self, hb_id: &str) -> Result<CachedPeer> {
        use tokio::task::JoinSet;

        let mut set: JoinSet<Result<PeerResponse>> = JoinSet::new();
        let relay_urls = self.relay_urls.read().unwrap().clone();

        for url in &relay_urls {
            let endpoint = format!("{url}/v1/peer/{hb_id}");
            let client = self.http.clone();
            set.spawn(async move {
                let resp = client
                    .get(&endpoint)
                    .send()
                    .await
                    .context("relay unreachable")?;

                if !resp.status().is_success() {
                    return Err(anyhow!("relay returned {}", resp.status()));
                }

                resp.json::<PeerResponse>().await.context("invalid relay response")
            });
        }

        let mut last_err = anyhow!("no relays configured");
        while let Some(result) = set.join_next().await {
            match result {
                Ok(Ok(peer_resp)) => {
                    let profile = peer_resp.profile
                        .as_ref()
                        .and_then(|e| e.parse_payload().ok());
                    let collections = peer_resp.collections
                        .iter()
                        .filter_map(|e| e.parse_payload().ok())
                        .collect();
                    return Ok(CachedPeer {
                        hb_id: hb_id.to_string(),
                        profile,
                        collections,
                        online: peer_resp.online,
                        node_addr: peer_resp.node_addr,
                        last_fetched: chrono::Utc::now(),
                    });
                }
                Ok(Err(e)) => last_err = e,
                Err(e) => last_err = anyhow!("task error: {e}"),
            }
        }

        Err(last_err)
    }

    /// Fetch messages from all relays addressed to `my_pubkey`.
    /// Merges results from all relays, deduplicating by (from_key, sent_at).
    /// Each envelope's signature is verified before inclusion.
    pub async fn fetch_messages(
        &self,
        my_pubkey: &str,
    ) -> Result<Vec<(String, ChatMessage)>> {
        #[derive(Deserialize)]
        struct MessagesResponse {
            messages: Vec<SignedEnvelope>,
        }

        let relay_urls = self.relay_urls.read().unwrap().clone();
        let mut all_messages: Vec<(String, ChatMessage)> = Vec::new();
        let mut seen: std::collections::HashSet<(String, String)> = Default::default();

        for url in &relay_urls {
            let endpoint = format!("{url}/v1/messages/{my_pubkey}");
            match self.http.get(&endpoint).send().await {
                Ok(resp) if resp.status().is_success() => {
                    if let Ok(body) = resp.json::<MessagesResponse>().await {
                        for envelope in body.messages {
                            // Verify signature before trusting any content.
                            if envelope.verify().is_err() {
                                tracing::warn!("dropped message with invalid signature");
                                continue;
                            }
                            if let Ok(msg) = envelope.parse_payload::<ChatMessage>() {
                                let key = (
                                    envelope.public_key.clone(),
                                    msg.sent_at.to_rfc3339(),
                                );
                                if seen.insert(key) {
                                    all_messages.push((envelope.public_key, msg));
                                }
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => tracing::debug!("relay {url} messages fetch failed: {e}"),
            }
        }

        all_messages.sort_by_key(|(_, msg)| msg.sent_at);
        Ok(all_messages)
    }

    /// Send a heartbeat to all relays to signal liveness.
    /// Called every 5 minutes by the background task in lib.rs.
    pub async fn send_heartbeat(
        &self,
        keypair: &hb_core::HoardbookKeypair,
        node_addr: Option<String>,
    ) -> Result<()> {
        use hb_core::types::HeartbeatBody;

        let signed_at = chrono::Utc::now().to_rfc3339();
        let body = HeartbeatBody {
            node_addr: node_addr.clone(),
            public_key: keypair.hb_id(),
            signed_at: signed_at.clone(),
        };
        let body_value = serde_json::to_value(&body)?;
        let signature = keypair.sign(&body_value);

        #[derive(Serialize)]
        struct HeartbeatRequest {
            public_key: String,
            signed_at: String,
            node_addr: Option<String>,
            signature: String,
        }

        let req = HeartbeatRequest {
            public_key: keypair.hb_id(),
            signed_at,
            node_addr,
            signature,
        };

        let relay_urls = self.relay_urls.read().unwrap().clone();
        for url in &relay_urls {
            let endpoint = format!("{url}/v1/heartbeat");
            if let Err(e) = self.http.post(&endpoint).json(&req).send().await {
                tracing::debug!("heartbeat to {url} failed: {e}");
            }
        }

        Ok(())
    }
}
