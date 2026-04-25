use chrono::Utc;
use hb_core::{DocType, SignedEnvelope, types::ChatMessage};
use serde::Serialize;
use tauri::State;

use crate::{
    error::{CmdResult, cmd_err},
    SharedIdentity, SharedRelay,
};

/// A decoded, sender-attributed chat message returned to the frontend.
/// Content is always plaintext — decryption happens here before returning.
#[derive(Debug, Clone, Serialize)]
pub struct ReceivedMessage {
    pub from: String,
    pub to: String,
    pub content: String,
    pub sent_at: String, // ISO 8601
    pub encrypted: bool,
}

/// Encrypt and send a chat message to `to` via all configured relays.
/// Returns the sent message so the frontend can append it immediately.
#[tauri::command]
pub async fn send_message(
    to: String,
    content: String,
    identity: State<'_, SharedIdentity>,
    relay: State<'_, SharedRelay>,
) -> CmdResult<ReceivedMessage> {
    // Validate recipient and decode their public key for encryption.
    let recipient_pubkey = hb_core::hb_id_decode(&to)
        .map_err(|_| "Invalid recipient ID".to_string())?;

    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        return Err("Message cannot be empty".into());
    }
    if trimmed.len() > 4096 {
        return Err(format!(
            "Message too long ({} chars, max 4096)",
            trimmed.len()
        ));
    }

    let guard = identity.read().await;
    let kp = guard
        .as_ref()
        .ok_or("No identity loaded. Generate a keypair first.")?;

    let encrypted_content = kp
        .encrypt_for(&recipient_pubkey, &trimmed)
        .map_err(cmd_err)?;

    let sent_at = Utc::now();
    let msg = ChatMessage {
        to: to.clone(),
        content: encrypted_content,
        encrypted: true,
        sent_at,
    };

    let envelope = SignedEnvelope::create(kp, DocType::Message, &msg).map_err(cmd_err)?;
    let from = kp.hb_id();

    relay.publish("message", &envelope).await.map_err(cmd_err)?;

    Ok(ReceivedMessage {
        from,
        to,
        content: trimmed, // return plaintext to the frontend
        sent_at: sent_at.to_rfc3339(),
        encrypted: true,
    })
}

/// Fetch and decrypt messages from all relays addressed to the current user's inbox.
/// Messages with invalid or undecryptable content are returned with a placeholder.
#[tauri::command]
pub async fn get_messages(
    identity: State<'_, SharedIdentity>,
    relay: State<'_, SharedRelay>,
) -> CmdResult<Vec<ReceivedMessage>> {
    let guard = identity.read().await;
    let kp = guard.as_ref().ok_or("No identity loaded.")?;
    let my_pubkey = kp.hb_id();

    let raw = relay.fetch_messages(&my_pubkey).await.map_err(cmd_err)?;

    let messages = raw
        .into_iter()
        .map(|(from, msg)| {
            let content = if msg.encrypted {
                // Decode the sender's hb_id and decrypt. If anything fails,
                // surface a placeholder so the conversation still renders.
                match hb_core::hb_id_decode(&from) {
                    Ok(sender_pubkey) => kp
                        .decrypt_from(&sender_pubkey, &msg.content)
                        .unwrap_or_else(|_| "[decryption failed]".to_string()),
                    Err(_) => "[unknown sender key]".to_string(),
                }
            } else {
                msg.content
            };

            ReceivedMessage {
                from,
                to: msg.to,
                content,
                sent_at: msg.sent_at.to_rfc3339(),
                encrypted: msg.encrypted,
            }
        })
        .collect();

    Ok(messages)
}
