use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    error::{CmdResult, cmd_err},
    store::DataStore,
    SharedRelay,
};

fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub relay_urls: Vec<String>,
    #[serde(default = "default_true")]
    pub allow_dms: bool,
}

#[tauri::command]
pub async fn get_settings(store: State<'_, DataStore>) -> CmdResult<Settings> {
    Ok(store.load_settings().map_err(cmd_err)?.unwrap_or_default())
}

#[tauri::command]
pub async fn save_settings(
    settings: Settings,
    store: State<'_, DataStore>,
    relay: State<'_, SharedRelay>,
) -> CmdResult<()> {
    store.save_settings(&settings).map_err(cmd_err)?;
    relay.set_relay_urls(settings.relay_urls);
    Ok(())
}
