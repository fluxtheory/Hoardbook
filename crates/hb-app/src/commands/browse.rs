use hb_core::hb_id_decode;
use tauri::State;

use crate::{
    error::{CmdResult, cmd_err},
    store::{CachedPeer, DataStore},
    SharedRelay,
};

#[tauri::command]
pub async fn paste_key(
    hb_id: String,
    relay: State<'_, SharedRelay>,
) -> CmdResult<CachedPeer> {
    hb_id_decode(&hb_id).map_err(cmd_err)?;
    relay.fetch_peer(&hb_id).await.map_err(cmd_err)
}

#[tauri::command]
pub async fn follow(
    hb_id: String,
    relay: State<'_, SharedRelay>,
    store: State<'_, DataStore>,
) -> CmdResult<()> {
    hb_id_decode(&hb_id).map_err(cmd_err)?;
    let peer = relay.fetch_peer(&hb_id).await.map_err(cmd_err)?;
    let hash = CachedPeer::pubkey_hash(&hb_id);
    store.save_contact(&hash, &peer).map_err(cmd_err)
}

#[tauri::command]
pub async fn get_contacts(store: State<'_, DataStore>) -> CmdResult<Vec<CachedPeer>> {
    store.list_contacts().map_err(cmd_err)
}

#[tauri::command]
pub async fn unfollow_contact(
    hb_id: String,
    store: State<'_, DataStore>,
) -> CmdResult<()> {
    hb_id_decode(&hb_id).map_err(cmd_err)?;
    let hash = CachedPeer::pubkey_hash(&hb_id);
    store.delete_contact(&hash).map_err(cmd_err)
}

#[tauri::command]
pub async fn refresh_contact(
    hb_id: String,
    relay: State<'_, SharedRelay>,
    store: State<'_, DataStore>,
) -> CmdResult<CachedPeer> {
    hb_id_decode(&hb_id).map_err(cmd_err)?;
    let peer = relay.fetch_peer(&hb_id).await.map_err(cmd_err)?;
    let hash = CachedPeer::pubkey_hash(&hb_id);
    store.save_contact(&hash, &peer).map_err(cmd_err)?;
    Ok(peer)
}
