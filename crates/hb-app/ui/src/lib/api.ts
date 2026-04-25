import { invoke } from '@tauri-apps/api/core';
import type {
	CachedPeer,
	Collection,
	IdentityInfo,
	Profile,
	ReceivedMessage,
	ScanOptions
} from './types.js';

// ── Identity ─────────────────────────────────────────────────────────────────

export const generateKeypair = () => invoke<IdentityInfo>('generate_keypair');

export const getIdentity = () => invoke<IdentityInfo | null>('get_identity');

export const getHbId = () => invoke<string>('get_hb_id');

export const validateHbId = (hb_id: string) => invoke<boolean>('validate_hb_id', { hb_id });

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

// ── Chat ──────────────────────────────────────────────────────────────────────

export const sendMessage = (to: string, content: string) =>
	invoke<ReceivedMessage>('send_message', { to, content });

export const getMessages = () => invoke<ReceivedMessage[]>('get_messages');
