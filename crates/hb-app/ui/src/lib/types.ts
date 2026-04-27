// Mirrors hb-core Rust types

export interface IdentityInfo {
	hb_id: string;
	hb_id_short: string;
}

export interface Profile {
	display_name: string;
	bio?: string;
	tags: string[];
	since?: number;
	est_size?: string;
	languages: string[];
	contact_hint?: string;
	/** Publicly visible — user explicitly opts in by filling this field. */
	email?: string;
	updated: string; // ISO datetime
}

export interface ReceivedMessage {
	from: string;  // sender hb_id
	to: string;    // recipient hb_id
	content: string;
	sent_at: string; // ISO datetime
}

export interface DirectoryItem {
	name: string;
	item_type: 'File' | 'Folder';
	size?: string;
	format?: string;
	year?: number;
	tags: string[];
	note?: string;
	children: DirectoryItem[];
}

export interface Collection {
	slug: string;
	path_alias: string;
	description?: string;
	item_count: number;
	est_size?: string;
	content_type: string[];
	last_updated: string;
	listing: DirectoryItem[];
}

export interface CachedPeer {
	hb_id: string;
	profile?: Profile;
	collections: Collection[];
	online: boolean;
	node_addr?: string;
	last_fetched: string;
}

export interface ScanOptions {
	path: string;
	path_alias: string;
	depth: number;
	exclude: string[];
}

export interface ShareSettings {
	enabled: boolean;
	root_path?: string;
	allowed_paths: string[];
	speed_cap_kbps?: number;
	download_limit?: number;
	require_follow: boolean;
}
