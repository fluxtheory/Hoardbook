# Hoardbook

A P2P social phonebook for datahoarders. Share what you have, discover what others are sitting on, and connect with people who actually understand why you have twelve copies of a 4K encode of a 1954 film.

> **Status: early development / pre-alpha.** Core plumbing is working but there are no downloadable releases yet.

---

## What it does

Hoardbook gives datahoarders a way to publish a directory of their collections and find other hoarders nearby (in interest, not geography). You generate a keypair, publish a signed snapshot of your collection, and share your Hoardbook ID with people you want to connect with. They add you as a contact and can browse your collection, see when you're online, and send you a message.

No accounts. No servers you have to trust. No IP address exposure by default.

### Features

- **Collections** — publish a directory tree of your hoard with metadata: item counts, estimated size, tags, format info. Visitors see a two-pane file browser.
- **Contacts** — add peers by Hoardbook ID. See their online status (● Online / ◐ Stale / ○ Offline).
- **Chat** — direct messages routed through the relay, with optional E2E encryption (X25519 + ChaCha20-Poly1305).
- **Keypair identity** — Ed25519 keypairs. Your ID is a ~52-character string (`hb1_…`). No email, no username, no phone number.
- **Key succession** — if you rotate your key, a signed succession document is published so your contacts' apps migrate silently.

---

## Privacy model

Relay-first is the **default**. Your IP address is never exposed to peers unless you explicitly opt in to direct connections, at which point the app shows a warning modal explaining what that means.

The relay is a neutral pipe — it stores signed documents and routes messages but cannot read E2E-encrypted content and has no ability to forge documents (all envelopes are Ed25519-signed).

Your data lives in local signed JSON files. There is no central database that owns your identity.

---

## Architecture

This is a Cargo workspace with three crates:

```
hb-core/    — shared types, Ed25519 crypto, signed envelope format
hb-relay/   — HTTP relay server (axum + sqlx + SQLite)
hb-app/     — Tauri desktop app (Rust backend + SvelteKit UI)
```

### Relay HTTP API

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/v1/publish` | Publish a signed profile, collection, succession, or message |
| `POST` | `/v1/heartbeat` | Update online status (optionally advertise iroh node addr) |
| `GET` | `/v1/peer/:pubkey` | Fetch a peer's profile, collections, and online status |
| `GET` | `/v1/messages/:pubkey` | Fetch messages addressed to a pubkey |
| `GET` | `/v1/health` | Relay health and peer count |

All documents are `SignedEnvelope` — a JCS-canonicalized JSON payload with an Ed25519 signature. The relay verifies every signature before storing anything.

---

## Building from source

**Prerequisites:** Rust (stable), Node.js 18+, and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform.

```sh
# Clone
git clone https://github.com/fluxtheory/hoardbook
cd hoardbook

# Build the relay
cargo build --release -p hb-relay

# Run the relay (defaults to :3000)
./target/release/hb-relay

# Build the desktop app
cd crates/hb-app/ui && npm install && npm run build && cd ../../..
cargo tauri build
```

The relay can be self-hosted. A Docker image will be published alongside the first release.

---

## Platform support

- **Windows** — primary target
- **Linux** — supported
- **macOS** — Phase 2

---

## Roadmap

- [ ] Onboarding wizard (keygen → profile → first collection)
- [ ] Collection snapshot export (plain text + markdown checklist)
- [ ] Docker image for relay self-hosting
- [ ] Relay peering / discovery
- [ ] macOS support
- [ ] Qurator integration

---

## Self-hosting a relay

The relay is intentionally simple to self-host. It needs a writable directory for the SQLite database and an exposed port. A `docker-compose.yml` and self-hosting spec will ship with the first release.

Relay operators set their own terms of service. The relay code enforces rate limits and size caps but takes no position on content.

---

## License

MIT
