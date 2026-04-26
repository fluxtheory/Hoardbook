#![forbid(unsafe_code)]

mod commands;
mod error;
mod relay;
mod store;

use std::sync::Arc;
use hb_core::HoardbookKeypair;
use relay::RelayClient;
use store::DataStore;
use tauri::Manager;
use tokio::sync::RwLock;

/// Managed state types — Arc-wrapped so they can be cloned into background tasks.
pub type SharedIdentity = Arc<RwLock<Option<HoardbookKeypair>>>;
pub type SharedRelay = Arc<RelayClient>;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("could not resolve app data dir");

            std::fs::create_dir_all(&data_dir).expect("could not create app data dir");

            let identity: SharedIdentity = Arc::new(RwLock::new(None));

            // Load saved relay URLs from settings, if any.
            let store_tmp = DataStore::new(data_dir.clone());
            let saved_relays = store_tmp
                .load_settings()
                .ok()
                .flatten()
                .map(|s| s.relay_urls)
                .unwrap_or_default();
            let relay: SharedRelay = Arc::new(RelayClient::new(saved_relays));

            app.manage(DataStore::new(data_dir));
            app.manage(Arc::clone(&identity));
            app.manage(Arc::clone(&relay));

            // Background heartbeat every 5 minutes.
            tauri::async_runtime::spawn(async move {
                let mut interval =
                    tokio::time::interval(tokio::time::Duration::from_secs(300));
                loop {
                    interval.tick().await;
                    let guard = identity.read().await;
                    if let Some(ref kp) = *guard {
                        if let Err(e) = relay.send_heartbeat(kp, None).await {
                            tracing::debug!("heartbeat failed: {e}");
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::identity::generate_keypair,
            commands::identity::get_identity,
            commands::identity::get_hb_id,
            commands::identity::validate_hb_id,
            commands::identity::export_keypair,
            commands::identity::wipe_data,
            commands::profile::save_profile,
            commands::profile::get_profile,
            commands::profile::publish_profile,
            commands::profile::unpublish_profile,
            commands::collection::scan_directory,
            commands::collection::get_collections,
            commands::collection::publish_collection,
            commands::browse::paste_key,
            commands::browse::follow,
            commands::browse::get_contacts,
            commands::browse::refresh_contact,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::chat::send_message,
            commands::chat::get_messages,
            commands::sharing::get_share_settings,
            commands::sharing::save_share_settings,
            commands::sharing::request_download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Hoardbook");
}
