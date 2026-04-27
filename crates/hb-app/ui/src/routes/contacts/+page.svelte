<script lang="ts">
	import { refreshContact, requestDownload, unfollowContact } from '$lib/api.js';
	import { save } from '@tauri-apps/plugin-dialog';
	import { contacts, toast } from '$lib/stores.js';
	import { icons, avatarHue } from '$lib/icons.js';
	import CollectionPanel from '$lib/components/CollectionPanel.svelte';
	import Avatar from '$lib/components/Avatar.svelte';

	let expanded: string | null = null;
	let refreshing: string | null = null;

	async function handleRefresh(hb_id: string) {
		refreshing = hb_id;
		try {
			const updated = await refreshContact(hb_id);
			contacts.update((cs) => cs.map((c) => (c.hb_id === hb_id ? updated : c)));
			toast('Contact refreshed');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			refreshing = null;
		}
	}

	function shortId(hb_id: string) {
		return hb_id.length > 14 ? hb_id.slice(0, 8) + '…' + hb_id.slice(-4) : hb_id;
	}

	async function handleUnfollow(hb_id: string) {
		try {
			await unfollowContact(hb_id);
			contacts.update((cs) => cs.filter((c) => c.hb_id !== hb_id));
			toast('Contact removed');
		} catch (e) {
			toast(String(e), 'error');
		}
	}

	async function handleDownload(e: CustomEvent<{ peerId: string; slug: string; path: string }>) {
		const filename = e.detail.path.split('/').pop() ?? e.detail.path;
		const savePath = await save({ defaultPath: filename });
		if (!savePath) return;
		const peer = $contacts.find((c) => c.hb_id === e.detail.peerId);
		try {
			await requestDownload(e.detail.peerId, peer?.node_addr ?? null, e.detail.slug, e.detail.path, savePath);
			toast(`Downloading to ${savePath}`);
		} catch (err) {
			toast(String(err), 'error');
		}
	}
</script>

<!-- TopBar -->
<div class="topbar">
	<div>
		<div class="topbar-title">Contacts</div>
		<div class="topbar-sub">
			{$contacts.length} followed peer{$contacts.length !== 1 ? 's' : ''} · {$contacts.filter(c => c.online).length} online
		</div>
	</div>
	<div class="topbar-actions">
		<a href="/browse" class="btn-ghost btn-sm">
			<span>{@html icons.plus}</span>Add by hb_id
		</a>
	</div>
</div>

<div class="body">
	{#if $contacts.length === 0}
		<div class="empty">
			No contacts yet. <a href="/browse" class="empty-link">Browse</a> to follow someone.
		</div>
	{:else}
		<div class="contact-list">
			{#each $contacts as peer}
				{@const name = peer.profile?.display_name ?? 'Unknown'}
				{@const initial = name[0]?.toUpperCase() ?? '?'}
				{@const hue = avatarHue(initial)}
				<div class="contact-block">
					<div class="contact-card">
						<Avatar letter={initial} size={42} {hue} />
						<div class="contact-info">
							<div class="name-row">
								<span class="peer-name">{name}</span>
								{#if peer.online}
									<span class="pill pill-online"><span class="pill-dot" /> Online</span>
								{:else}
									<span class="pill pill-offline">Offline</span>
								{/if}
								<span class="last-seen">seen {peer.last_fetched ? 'recently' : 'never'}</span>
							</div>
							<div class="mono">{shortId(peer.hb_id)}</div>
							{#if peer.profile?.bio}
								<p class="bio">{peer.profile.bio}</p>
							{/if}
							<div class="contact-footer">
								{#if peer.profile?.est_size}
									<span class="tnum">~{peer.profile.est_size}</span>
								{/if}
								{#if peer.collections.length > 0}
									<span class="dot">·</span>
									<span>{peer.collections.length} collection{peer.collections.length !== 1 ? 's' : ''}</span>
								{/if}
								<div style="flex:1" />
								<button
									class="btn-ghost btn-sm btn-icon"
									on:click={() => handleRefresh(peer.hb_id)}
									disabled={refreshing === peer.hb_id}
								>
									<span>{@html icons.refresh}</span>
									{refreshing === peer.hb_id ? '…' : 'Refresh'}
								</button>
								<button
									class="btn-ghost btn-sm btn-danger"
									on:click={() => handleUnfollow(peer.hb_id)}
								>
									Unfollow
								</button>
								<button
									class="btn-default btn-sm"
									on:click={() => (expanded = expanded === peer.hb_id ? null : peer.hb_id)}
								>
									{expanded === peer.hb_id ? 'Hide' : 'View'} collections
								</button>
							</div>
						</div>
					</div>

					{#if expanded === peer.hb_id}
						<div class="collections-indent">
							{#if peer.collections.length === 0}
								<p class="no-coll">No collections published.</p>
							{:else}
								{#each peer.collections as col}
									<CollectionPanel collection={col} peerId={peer.hb_id} on:download={handleDownload} />
								{/each}
							{/if}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.topbar {
		padding: 16px 24px;
		border-bottom: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: var(--bg);
		flex-shrink: 0;
	}
	.topbar-title { font-size: 17px; font-weight: 600; letter-spacing: -0.3px; }
	.topbar-sub { font-size: 12px; color: var(--fg-muted); margin-top: 2px; }
	.topbar-actions { display: flex; gap: 8px; }

	.body { padding: 24px; overflow-y: auto; flex: 1; }

	.empty { color: var(--fg-dim); font-size: 13px; padding: 32px 0; }

	.empty-link { color: var(--accent); text-decoration: none; }

	.contact-list { display: flex; flex-direction: column; gap: 12px; }

	.contact-block { display: flex; flex-direction: column; gap: 8px; }

	.contact-card {
		background: var(--bg-elev1);
		border: 1px solid var(--border);
		border-radius: 10px;
		padding: 14px;
		display: flex;
		gap: 14px;
		align-items: flex-start;
	}

	.contact-info { flex: 1; min-width: 0; }

	.name-row { display: flex; align-items: center; gap: 8px; margin-bottom: 3px; flex-wrap: wrap; }

	.peer-name { font-size: 14px; font-weight: 600; letter-spacing: -0.2px; }

	.last-seen { font-size: 11px; color: var(--fg-dim); margin-left: auto; }

	.mono { font-family: var(--font-mono); font-size: 11px; color: var(--fg-muted); }

	.bio { font-size: 12.5px; color: var(--fg-muted); margin-top: 6px; line-height: 1.5; }

	.contact-footer {
		display: flex;
		gap: 8px;
		align-items: center;
		margin-top: 8px;
		font-size: 11.5px;
		color: var(--fg-muted);
	}

	.tnum { font-feature-settings: 'tnum'; }
	.dot { color: var(--fg-dim); }

	.collections-indent { padding-left: 56px; display: flex; flex-direction: column; gap: 8px; }

	.no-coll { font-size: 12px; color: var(--fg-dim); }

	/* Pills */
	.pill {
		display: inline-flex; align-items: center; gap: 5px;
		font-size: 10.5px; font-weight: 500;
		padding: 2px 8px; border-radius: 999px;
	}
	.pill-dot { width: 5px; height: 5px; border-radius: 50%; }
	.pill-online {
		color: var(--online);
		background: color-mix(in oklch, var(--online) 12%, transparent);
		border: 1px solid color-mix(in oklch, var(--online) 20%, transparent);
	}
	.pill-online .pill-dot { background: var(--online); }
	.pill-offline {
		color: var(--fg-muted);
		background: color-mix(in oklch, var(--fg-muted) 12%, transparent);
		border: 1px solid color-mix(in oklch, var(--fg-muted) 20%, transparent);
	}

	/* Buttons */
	.btn-default {
		display: inline-flex; align-items: center; justify-content: center; gap: 6px;
		padding: 8px 14px; font-family: var(--font-ui); font-size: 13px; font-weight: 500;
		color: var(--fg); background: transparent;
		border: 1px solid var(--border-strong); border-radius: 7px;
		cursor: pointer; white-space: nowrap; user-select: none; line-height: 1; text-decoration: none;
	}
	.btn-ghost {
		display: inline-flex; align-items: center; justify-content: center; gap: 6px;
		padding: 8px 14px; font-family: var(--font-ui); font-size: 13px; font-weight: 500;
		color: var(--fg-muted); background: transparent;
		border: 1px solid transparent; border-radius: 7px;
		cursor: pointer; white-space: nowrap; user-select: none; line-height: 1; text-decoration: none;
	}
	.btn-ghost:disabled { opacity: 0.5; cursor: not-allowed; }
	.btn-sm { padding: 5px 11px; font-size: 12px; }
	.btn-icon { gap: 4px; }
	.btn-danger { color: var(--red, #e05c5c); }
	.btn-danger:hover { background: color-mix(in oklch, var(--red, #e05c5c) 10%, transparent); }
</style>
