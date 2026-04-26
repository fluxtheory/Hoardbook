<script lang="ts">
	import { pasteKey, follow } from '$lib/api.js';
	import { contacts, toast } from '$lib/stores.js';
	import { icons, avatarHue } from '$lib/icons.js';
	import ProfileCard from '$lib/components/ProfileCard.svelte';
	import CollectionPanel from '$lib/components/CollectionPanel.svelte';
	import Avatar from '$lib/components/Avatar.svelte';
	import type { CachedPeer } from '$lib/types.js';

	let input = '';
	let loading = false;
	let following = false;
	let result: CachedPeer | null = null;

	$: alreadyFollowed = $contacts.some((c) => c.hb_id === result?.hb_id);

	async function handleLookup() {
		const id = input.trim();
		if (!id) return;
		loading = true;
		result = null;
		try {
			result = await pasteKey(id);
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			loading = false;
		}
	}

	async function handleFollow() {
		if (!result) return;
		following = true;
		try {
			await follow(result.hb_id);
			contacts.update((cs) => {
				if (cs.find((c) => c.hb_id === result!.hb_id)) return cs;
				return [...cs, result!];
			});
			toast(`Following ${result.profile?.display_name ?? result.hb_id}`);
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			following = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') handleLookup();
	}
</script>

<!-- TopBar -->
<div class="topbar">
	<div>
		<div class="topbar-title">Browse</div>
		<div class="topbar-sub">Look up any peer by their Hoardbook ID</div>
	</div>
</div>

<div class="body">
	<!-- Search bar -->
	<div class="search-row">
		<div class="search-input-wrap">
			<span class="search-icon">{@html icons.search}</span>
			<input
				class="search-input hb-mono"
				type="text"
				placeholder="hb1_…"
				bind:value={input}
				on:keydown={handleKeydown}
			/>
		</div>
		<button
			class="btn-primary"
			on:click={handleLookup}
			disabled={!input.trim() || loading}
		>
			{loading ? 'Looking up…' : 'Lookup'}
		</button>
	</div>

	{#if result}
		<div class="result">
			<!-- Profile card (browse style with banner) -->
			<div class="profile-card">
				<div class="profile-banner" />
				<div class="profile-inner">
					<div class="profile-top">
						<Avatar
							letter={(result.profile?.display_name ?? result.hb_id)[0].toUpperCase()}
							size={56}
							hue={avatarHue((result.profile?.display_name ?? result.hb_id)[0])}
						/>
						<div class="profile-name-col">
							<div class="name-row">
								<span class="peer-name">{result.profile?.display_name ?? 'Unknown'}</span>
								{#if result.online}
									<span class="pill pill-online"><span class="pill-dot" /> Online</span>
								{:else}
									<span class="pill pill-offline">Offline</span>
								{/if}
							</div>
							<span class="mono">{result.hb_id.slice(0, 18)}…{result.hb_id.slice(-4)}</span>
						</div>
						<div class="profile-actions">
							<button class="btn-ghost btn-sm">Message</button>
							<button
								class="btn-primary btn-sm"
								on:click={handleFollow}
								disabled={alreadyFollowed || following}
							>
								{alreadyFollowed ? 'Following' : following ? '…' : 'Follow'}
							</button>
						</div>
					</div>

					{#if result.profile?.bio}
						<p class="peer-bio">{result.profile.bio}</p>
					{/if}

					<div class="peer-metrics">
						{#if result.profile?.est_size}
							<div class="metric">
								<div class="metric-label">Size</div>
								<div class="metric-val">{result.profile.est_size}</div>
							</div>
							<div class="metric-divider" />
						{/if}
						{#if result.collections.length > 0}
							<div class="metric">
								<div class="metric-label">Collections</div>
								<div class="metric-val">{result.collections.length}</div>
							</div>
							<div class="metric-divider" />
						{/if}
						{#if result.profile?.since}
							<div class="metric">
								<div class="metric-label">Since</div>
								<div class="metric-val">{result.profile.since}</div>
							</div>
						{/if}
						{#if result.profile?.languages?.length}
							<div class="metric-divider" />
							<div class="metric">
								<div class="metric-label">Languages</div>
								<div class="metric-val">{result.profile.languages.join(', ')}</div>
							</div>
						{/if}
					</div>
				</div>
			</div>

			{#if result.collections.length > 0}
				<div class="section-label">Public collections</div>
				<div class="coll-list">
					{#each result.collections as col}
						<CollectionPanel collection={col} />
					{/each}
				</div>
			{/if}
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

	.body { padding: 24px; overflow-y: auto; flex: 1; }

	.search-row {
		display: flex;
		gap: 8px;
		margin-bottom: 22px;
		max-width: 600px;
	}

	.search-input-wrap {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0 11px;
		height: 34px;
		background: var(--bg-input);
		border: 1px solid var(--border);
		border-radius: 7px;
	}

	.search-icon { color: var(--fg-dim); display: flex; flex-shrink: 0; }

	.search-input {
		flex: 1;
		background: transparent;
		border: none;
		outline: none;
		font-size: 13px;
		color: var(--fg);
		min-width: 0;
	}
	.search-input::placeholder { color: var(--fg-dim); }
	.hb-mono { font-family: var(--font-mono); }

	.result { max-width: 600px; display: flex; flex-direction: column; gap: 16px; }

	/* Profile card browse variant */
	.profile-card {
		background: var(--bg-elev1);
		border: 1px solid var(--border);
		border-radius: 10px;
		overflow: hidden;
	}

	.profile-banner {
		height: 64px;
		background: linear-gradient(135deg, oklch(0.30 0.10 280) 0%, oklch(0.25 0.12 320) 100%);
		border-bottom: 1px solid var(--border);
	}

	.profile-inner {
		padding: 0 18px 18px;
		margin-top: -28px;
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.profile-top {
		display: flex;
		gap: 14px;
		align-items: flex-end;
	}

	.profile-name-col { flex: 1; min-width: 0; padding-bottom: 4px; }

	.name-row { display: flex; gap: 8px; align-items: center; margin-bottom: 3px; }

	.peer-name { font-weight: 600; font-size: 16px; letter-spacing: -0.3px; }

	.mono { font-family: var(--font-mono); font-size: 11px; color: var(--fg-muted); }

	.profile-actions { display: flex; gap: 8px; padding-bottom: 4px; }

	.peer-bio { font-size: 13px; color: var(--fg); line-height: 1.55; }

	.peer-metrics { display: flex; gap: 16px; font-size: 12px; align-items: center; flex-wrap: wrap; }

	.metric { display: flex; flex-direction: column; gap: 2px; }

	.metric-label {
		font-size: 10.5px;
		color: var(--fg-dim);
		text-transform: uppercase;
		letter-spacing: 0.9px;
		font-weight: 600;
	}

	.metric-val { font-size: 14px; font-weight: 600; color: var(--fg); font-feature-settings: 'tnum'; letter-spacing: -0.2px; }

	.metric-divider { width: 1px; align-self: stretch; background: var(--divider); }

	.section-label {
		font-size: 10.5px;
		color: var(--fg-dim);
		text-transform: uppercase;
		letter-spacing: 1.2px;
		font-weight: 600;
		margin-bottom: -6px;
	}

	.coll-list { display: flex; flex-direction: column; gap: 10px; }

	/* Pills */
	.pill {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		font-size: 10.5px;
		font-weight: 500;
		padding: 2px 8px;
		border-radius: 999px;
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
	.btn-primary {
		display: inline-flex; align-items: center; justify-content: center; gap: 6px;
		padding: 8px 14px; font-family: var(--font-ui); font-size: 13px; font-weight: 600;
		color: var(--accent-text); background: var(--accent);
		border: 1px solid var(--accent); border-radius: 7px;
		cursor: pointer; white-space: nowrap; user-select: none; line-height: 1;
	}
	.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }
	.btn-ghost {
		display: inline-flex; align-items: center; justify-content: center; gap: 6px;
		padding: 8px 14px; font-family: var(--font-ui); font-size: 13px; font-weight: 500;
		color: var(--fg-muted); background: transparent;
		border: 1px solid transparent; border-radius: 7px;
		cursor: pointer; white-space: nowrap; user-select: none; line-height: 1;
	}
	.btn-sm { padding: 5px 11px; font-size: 12px; }
</style>
