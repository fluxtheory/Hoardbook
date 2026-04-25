use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

const EXPIRY_DAYS: i64 = 30;
const SECS_PER_DAY: i64 = 86_400;

// ---------------------------------------------------------------------------
// Connect & migrate
// ---------------------------------------------------------------------------

pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    // Ensure the database file is created if it doesn't exist.
    let opts = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await?;
    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            pubkey      TEXT    NOT NULL,
            doc_type    TEXT    NOT NULL CHECK(doc_type IN ('profile', 'succession')),
            envelope    TEXT    NOT NULL,
            stored_at   INTEGER NOT NULL,
            expires_at  INTEGER NOT NULL,
            PRIMARY KEY (pubkey, doc_type)
        );

        CREATE TABLE IF NOT EXISTS collections (
            pubkey      TEXT    NOT NULL,
            slug        TEXT    NOT NULL,
            envelope    TEXT    NOT NULL,
            stored_at   INTEGER NOT NULL,
            expires_at  INTEGER NOT NULL,
            PRIMARY KEY (pubkey, slug)
        );

        CREATE TABLE IF NOT EXISTS heartbeats (
            pubkey      TEXT    PRIMARY KEY,
            last_seen   INTEGER NOT NULL,
            node_addr   TEXT
        );

        CREATE TABLE IF NOT EXISTS messages (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            from_key    TEXT    NOT NULL,
            to_key      TEXT    NOT NULL,
            envelope    TEXT    NOT NULL,
            sent_at     TEXT    NOT NULL,
            stored_at   INTEGER NOT NULL,
            expires_at  INTEGER NOT NULL,
            UNIQUE(from_key, sent_at)
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_messages_to ON messages(to_key, sent_at DESC)",
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn now_secs() -> i64 {
    chrono::Utc::now().timestamp()
}

fn expiry_secs() -> i64 {
    now_secs() + EXPIRY_DAYS * SECS_PER_DAY
}

// ---------------------------------------------------------------------------
// Documents (profile, succession)
// ---------------------------------------------------------------------------

pub async fn upsert_document(
    pool: &SqlitePool,
    pubkey: &str,
    doc_type: &str,
    envelope_json: &str,
) -> Result<()> {
    let now = now_secs();
    let expires = expiry_secs();

    sqlx::query(
        r#"
        INSERT INTO documents (pubkey, doc_type, envelope, stored_at, expires_at)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(pubkey, doc_type) DO UPDATE SET
            envelope   = excluded.envelope,
            stored_at  = excluded.stored_at,
            expires_at = excluded.expires_at
        "#,
    )
    .bind(pubkey)
    .bind(doc_type)
    .bind(envelope_json)
    .bind(now)
    .bind(expires)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_document(
    pool: &SqlitePool,
    pubkey: &str,
    doc_type: &str,
) -> Result<Option<String>> {
    let now = now_secs();
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT envelope FROM documents WHERE pubkey = ? AND doc_type = ? AND expires_at > ?",
    )
    .bind(pubkey)
    .bind(doc_type)
    .bind(now)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(env,)| env))
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

pub async fn upsert_collection(
    pool: &SqlitePool,
    pubkey: &str,
    slug: &str,
    envelope_json: &str,
) -> Result<()> {
    let now = now_secs();
    let expires = expiry_secs();

    sqlx::query(
        r#"
        INSERT INTO collections (pubkey, slug, envelope, stored_at, expires_at)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(pubkey, slug) DO UPDATE SET
            envelope   = excluded.envelope,
            stored_at  = excluded.stored_at,
            expires_at = excluded.expires_at
        "#,
    )
    .bind(pubkey)
    .bind(slug)
    .bind(envelope_json)
    .bind(now)
    .bind(expires)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_collections(pool: &SqlitePool, pubkey: &str) -> Result<Vec<String>> {
    let now = now_secs();
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT envelope FROM collections WHERE pubkey = ? AND expires_at > ? ORDER BY slug",
    )
    .bind(pubkey)
    .bind(now)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(env,)| env).collect())
}

// ---------------------------------------------------------------------------
// Heartbeats
// ---------------------------------------------------------------------------

pub async fn upsert_heartbeat(
    pool: &SqlitePool,
    pubkey: &str,
    node_addr: Option<&str>,
) -> Result<()> {
    let now = now_secs();

    sqlx::query(
        r#"
        INSERT INTO heartbeats (pubkey, last_seen, node_addr)
        VALUES (?, ?, ?)
        ON CONFLICT(pubkey) DO UPDATE SET
            last_seen = excluded.last_seen,
            node_addr = excluded.node_addr
        "#,
    )
    .bind(pubkey)
    .bind(now)
    .bind(node_addr)
    .execute(pool)
    .await?;

    Ok(())
}

/// Returns `(last_seen_unix, node_addr)` or `None` if no heartbeat exists.
pub async fn get_heartbeat(
    pool: &SqlitePool,
    pubkey: &str,
) -> Result<Option<(i64, Option<String>)>> {
    let row: Option<(i64, Option<String>)> =
        sqlx::query_as("SELECT last_seen, node_addr FROM heartbeats WHERE pubkey = ?")
            .bind(pubkey)
            .fetch_optional(pool)
            .await?;

    Ok(row)
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

pub const MAX_MESSAGES_PER_RECIPIENT: i64 = 500;

pub async fn insert_message(
    pool: &SqlitePool,
    from_key: &str,
    to_key: &str,
    sent_at: &str,
    envelope_json: &str,
) -> Result<()> {
    let now = now_secs();
    let expires = expiry_secs();

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO messages (from_key, to_key, envelope, sent_at, stored_at, expires_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(from_key)
    .bind(to_key)
    .bind(envelope_json)
    .bind(sent_at)
    .bind(now)
    .bind(expires)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn count_messages_for(pool: &SqlitePool, to_key: &str) -> Result<i64> {
    let now = now_secs();
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE to_key = ? AND expires_at > ?",
    )
    .bind(to_key)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Returns the 100 most recent non-expired messages for `to_key`, oldest first.
pub async fn get_messages_for(pool: &SqlitePool, to_key: &str) -> Result<Vec<String>> {
    let now = now_secs();
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT envelope FROM (
            SELECT envelope, sent_at FROM messages
            WHERE to_key = ? AND expires_at > ?
            ORDER BY sent_at DESC
            LIMIT 100
        ) ORDER BY sent_at ASC
        "#,
    )
    .bind(to_key)
    .bind(now)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(env,)| env).collect())
}

// ---------------------------------------------------------------------------
// Expiry
// ---------------------------------------------------------------------------

/// Delete all expired documents, collections, and messages. Run hourly.
pub async fn expire_documents(pool: &SqlitePool) -> Result<()> {
    let now = now_secs();

    let docs = sqlx::query("DELETE FROM documents WHERE expires_at <= ?")
        .bind(now)
        .execute(pool)
        .await?
        .rows_affected();

    let colls = sqlx::query("DELETE FROM collections WHERE expires_at <= ?")
        .bind(now)
        .execute(pool)
        .await?
        .rows_affected();

    let msgs = sqlx::query("DELETE FROM messages WHERE expires_at <= ?")
        .bind(now)
        .execute(pool)
        .await?
        .rows_affected();

    if docs + colls + msgs > 0 {
        tracing::info!("expired {docs} documents, {colls} collections, {msgs} messages");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

pub async fn count_stored(pool: &SqlitePool) -> Result<i64> {
    let (doc_count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM documents").fetch_one(pool).await?;
    let (coll_count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM collections").fetch_one(pool).await?;
    Ok(doc_count + coll_count)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    async fn in_memory_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        migrate(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn insert_message_deduplicates_same_sender_and_timestamp() {
        // Two POSTs with identical (from_key, sent_at) — e.g. client retrying after a timeout —
        // must result in exactly one stored message, not two.
        let pool = in_memory_pool().await;
        insert_message(&pool, "alice", "bob", "2026-04-22T12:00:00Z", "envelope_v1")
            .await
            .unwrap();
        insert_message(&pool, "alice", "bob", "2026-04-22T12:00:00Z", "envelope_v2")
            .await
            .unwrap(); // duplicate — must be silently ignored (INSERT OR IGNORE)

        let msgs = get_messages_for(&pool, "bob").await.unwrap();
        assert_eq!(msgs.len(), 1, "duplicate (from, sent_at) must be silently dropped");
        assert_eq!(msgs[0], "envelope_v1", "first write wins");
    }

    #[tokio::test]
    async fn get_messages_returns_chronological_order() {
        // Messages must come back oldest-first so the chat UI renders naturally.
        let pool = in_memory_pool().await;
        insert_message(&pool, "s", "me", "2026-04-22T12:00:02Z", "third").await.unwrap();
        insert_message(&pool, "s", "me", "2026-04-22T12:00:00Z", "first").await.unwrap();
        insert_message(&pool, "s", "me", "2026-04-22T12:00:01Z", "second").await.unwrap();

        let msgs = get_messages_for(&pool, "me").await.unwrap();
        assert_eq!(msgs, ["first", "second", "third"]);
    }

    #[tokio::test]
    async fn get_messages_caps_at_100_most_recent() {
        // 150 messages stored; must return the 100 newest, still in chronological order.
        let pool = in_memory_pool().await;
        for i in 0u32..150 {
            let sent_at = format!(
                "2026-04-22T{:02}:{:02}:{:02}Z",
                i / 3600,
                (i / 60) % 60,
                i % 60
            );
            insert_message(&pool, "s", "r", &sent_at, &format!("env{i}"))
                .await
                .unwrap();
        }
        let msgs = get_messages_for(&pool, "r").await.unwrap();
        assert_eq!(msgs.len(), 100, "must cap at 100");
        // The 100 returned must be the 50 newest (env50…env149), oldest-first.
        assert_eq!(msgs[0], "env50", "should start from the 51st message (0-indexed)");
        assert_eq!(msgs[99], "env149", "should end with the newest");
    }

    #[tokio::test]
    async fn count_messages_for_reflects_actual_count() {
        let pool = in_memory_pool().await;
        assert_eq!(count_messages_for(&pool, "bob").await.unwrap(), 0);

        insert_message(&pool, "alice", "bob", "2026-04-22T12:00:00Z", "e1")
            .await
            .unwrap();
        insert_message(&pool, "carol", "bob", "2026-04-22T12:00:01Z", "e2")
            .await
            .unwrap();

        assert_eq!(
            count_messages_for(&pool, "bob").await.unwrap(),
            2,
            "count must include messages from all senders"
        );
        assert_eq!(
            count_messages_for(&pool, "alice").await.unwrap(),
            0,
            "count must be per-recipient"
        );
    }

    #[tokio::test]
    async fn mailbox_cap_constant_matches_handler_expectation() {
        // The handler rejects at >= MAX_MESSAGES_PER_RECIPIENT.
        // This test documents the invariant so a change to either constant is caught.
        assert_eq!(
            MAX_MESSAGES_PER_RECIPIENT,
            500,
            "changing this cap requires updating the relay docs and client UX"
        );
    }
}
