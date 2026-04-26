import { invoke as _invoke } from '@tauri-apps/api/core';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	if (!isTauri) return Promise.reject(new Error(`Tauri not available (cmd: ${cmd})`));
	return _invoke<T>(cmd, args);
}
import type {
	CachedPeer,
	Collection,
	IdentityInfo,
	Profile,
	ReceivedMessage,
	ScanOptions,
	ShareSettings,
} from './types.js';

// ── Identity ─────────────────────────────────────────────────────────────────

export const generateKeypair = () => invoke<IdentityInfo>('generate_keypair');

export const getIdentity = () => invoke<IdentityInfo | null>('get_identity');

export const getHbId = () => invoke<string>('get_hb_id');

export const validateHbId = (hb_id: string) => invoke<boolean>('validate_hb_id', { hb_id });

export const exportKeypair = () => invoke<string>('export_keypair');

export const wipeData = () => invoke<void>('wipe_data');

// ── Profile ───────────────────────────────────────────────────────────────────

export const saveProfile = (profile: Profile) => invoke<void>('save_profile', { profile });

export const getProfile = () => invoke<Profile | null>('get_profile');

export const publishProfile = () => invoke<void>('publish_profile');

export const unpublishProfile = () => invoke<void>('unpublish_profile');

// ── Collections ───────────────────────────────────────────────────────────────

export const scanDirectory = (opts: ScanOptions) =>
	invoke<Collection>('scan_directory', { opts });

export const getCollections = () => invoke<Collection[]>('get_collections');

export const publishCollection = (slug: string) =>
	invoke<void>('publish_collection', { slug });

// ── Settings ──────────────────────────────────────────────────────────────────

export interface Settings {
	relay_urls: string[];
}

export const getSettings = () => invoke<Settings>('get_settings');

export const saveSettings = (settings: Settings) => invoke<void>('save_settings', { settings });

// ── Browse / Contacts ─────────────────────────────────────────────────────────

export const pasteKey = (hb_id: string) => invoke<CachedPeer>('paste_key', { hb_id });

export const follow = (hb_id: string) => invoke<void>('follow', { hb_id });

export const getContacts = () => invoke<CachedPeer[]>('get_contacts');

export const refreshContact = (hb_id: string) => invoke<CachedPeer>('refresh_contact', { hb_id });

// ── Sharing ───────────────────────────────────────────────────────────────────

export const getShareSettings = (slug: string) =>
	invoke<ShareSettings>('get_share_settings', { slug });

export const saveShareSettings = (slug: string, settings: ShareSettings) =>
	invoke<void>('save_share_settings', { slug, settings });

export const requestDownload = (peer_hb_id: string, slug: string, path: string) =>
	invoke<void>('request_download', { peer_hb_id, slug, path });

// ── Chat ──────────────────────────────────────────────────────────────────────

export const sendMessage = (to: string, content: string) =>
	invoke<ReceivedMessage>('send_message', { to, content });

export const getMessages = () => invoke<ReceivedMessage[]>('get_messages');
