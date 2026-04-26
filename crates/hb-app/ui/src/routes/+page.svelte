<script lang="ts">
	import { saveProfile, publishProfile, publishCollection } from '$lib/api.js';
	import { profile, collections, identity, toast } from '$lib/stores.js';
	import { icons, avatarHue } from '$lib/icons.js';
	import CollectionPanel from '$lib/components/CollectionPanel.svelte';
	import ScanDialog from '$lib/components/ScanDialog.svelte';
	import ShareSettingsDialog from '$lib/components/ShareSettingsDialog.svelte';
	import Avatar from '$lib/components/Avatar.svelte';
	import type { Collection, Profile } from '$lib/types.js';

	let scanOpen = false;
	let saving = false;
	let publishing = false;
	let shareSlug = '';
	let shareOpen = false;

	let form: Profile = {
		display_name: '',
		bio: undefined,
		tags: [],
		since: undefined,
		est_size: undefined,
		languages: [],
		contact_hint: undefined,
		email: undefined,
		updated: new Date().toISOString(),
	};

	// Only initialize once — don't reset form after save/publish
	let profileLoaded = false;
	$: if ($profile && !profileLoaded) {
		form = { ...$profile };
		profileLoaded = true;
	}

	$: nameInitial = form.display_name?.[0]?.toUpperCase() ?? 'Y';
	$: nameHue = avatarHue(nameInitial);

	async function handleSave() {
		if (!form.display_name) return;
		saving = true;
		try {
			form.updated = new Date().toISOString();
			await saveProfile(form);
			profile.set(form);
			toast('Profile saved');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			saving = false;
		}
	}

	async function handlePublish() {
		if (!form.display_name) return;
		publishing = true;
		try {
			form.updated = new Date().toISOString();
			await saveProfile(form);
			profile.set(form);
			await publishProfile();
			toast('Profile published to relay');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			publishing = false;
		}
	}

	async function handlePublishCollection(slug: string) {
		try {
			await publishCollection(slug);
			toast('Collection published');
		} catch (e) {
			toast(String(e), 'error');
		}
	}

	function onScanned(event: CustomEvent<Collection>) {
		const col = event.detail;
		collections.update((cols) => {
			const idx = cols.findIndex((c) => c.slug === col.slug);
			if (idx >= 0) {
				const next = [...cols];
				next[idx] = col;
				return next;
			}
			return [...cols, col];
		});
		toast(`Scanned "${col.path_alias}" — ${col.item_count} items`);
	}

	function openShare(slug: string) {
		shareSlug = slug;
		shareOpen = true;
	}

	$: totalItems = $collections.reduce((s, c) => s + c.item_count, 0);
</script>

{#if !$identity}
	<!-- Onboarding state -->
	<div class="onboarding">
		<div class="ob-logo">H</div>
		<div class="ob-text">
			<div class="ob-title">Welcome to Hoardbook</div>
			<div class="ob-sub">A peer-to-peer field guide for data hoarders. Publish what you keep, find others who keep it too.</div>
		</div>
		<div class="ob-card">
			<div class="ob-card-head">
				<span class="sect-label">Step 1 of 3</span>
			</div>
			<div class="ob-card-title">Create your identity</div>
			<div class="ob-card-sub">Hoardbook uses a local Ed25519 keypair as your identity. No email, no server account.</div>
			<div class="ob-notice">
				<span class="ob-notice-icon">{@html icons.shield}</span>
				<div class="ob-notice-text">Your private key is stored locally and never transmitted. Back it up somewhere safe.</div>
			</div>
			<a href="/settings" class="btn-primary btn-full">Generate keypair</a>
		</div>
	</div>
{:else}
	<!-- TopBar -->
	<div class="topbar">
		<div>
			<div class="topbar-title">My Profile</div>
			<div class="topbar-sub">Visible to anyone with your hb_id</div>
		</div>
		<div class="topbar-actions">
			<button class="btn-ghost btn-sm" on:click={handleSave} disabled={!form.display_name || saving}>
				{saving ? 'Saving…' : 'Save draft'}
			</button>
			<button class="btn-primary btn-sm" on:click={handlePublish} disabled={publishing}>
				{publishing ? 'Publishing…' : 'Publish profile'}
			</button>
		</div>
	</div>

	<div class="body">
		<!-- Left: Profile editor -->
		<div class="profile-pane">
			<div class="profile-header">
				<Avatar letter={nameInitial} size={48} hue={nameHue} />
				<div class="profile-header-info">
					<div class="profile-name">{form.display_name || 'DataHoarder'}</div>
					<span class="mono">{$identity?.hb_id_short ?? ''}</span>
				</div>
			</div>

			<div class="fields">
				<div class="field">
					<label class="field-label">Display name <span class="accent-dot">•</span></label>
					<input class="hb-input" type="text" placeholder="e.g. DataHoarder_42" bind:value={form.display_name} />
				</div>

				<div class="field">
					<label class="field-label">Bio</label>
					<textarea class="hb-input hb-textarea" rows="3" placeholder="What do you collect?" bind:value={form.bio}></textarea>
				</div>

				<div class="field-row">
					<div class="field">
						<label class="field-label">Hoarding since</label>
						<input class="hb-input" type="number" min="1990" max="2099" placeholder="2003" bind:value={form.since} />
					</div>
					<div class="field">
						<label class="field-label">Est. size</label>
						<input class="hb-input" type="text" placeholder="14.2 TB" bind:value={form.est_size} />
					</div>
				</div>

				<div class="field">
					<div class="field-label-row">
						<label class="field-label">Languages</label>
						<span class="field-hint">ISO codes</span>
					</div>
					<input
						class="hb-input"
						type="text"
						placeholder="en, jp, de"
						value={form.languages.join(', ')}
						on:change={(e) => {
							form.languages = e.currentTarget.value.split(',').map((s) => s.trim()).filter(Boolean);
						}}
					/>
				</div>

				<div class="field">
					<label class="field-label">Contact email</label>
					<input class="hb-input hb-input-mono" type="email" placeholder="you@example.com" bind:value={form.contact_hint} />
				</div>
			</div>
		</div>

		<!-- Right: Collections -->
		<div class="collections-pane">
			<div class="coll-head">
				<div>
					<div class="coll-title">Collections</div>
					<div class="coll-sub">{$collections.length} published · {totalItems.toLocaleString()} items</div>
				</div>
				<button class="btn-add" on:click={() => (scanOpen = true)}>
					<span>{@html icons.plus}</span>Add collection
				</button>
			</div>

			<!-- Stat strip -->
			<div class="stats">
				<div class="stat">
					<div class="stat-label">Items</div>
					<div class="stat-value">{totalItems.toLocaleString()}</div>
				</div>
				<div class="stat">
					<div class="stat-label">Collections</div>
					<div class="stat-value">{$collections.length}</div>
				</div>
			</div>

			<div class="coll-list">
				{#if $collections.length === 0}
					<div class="empty">No collections yet. Click "Add collection" to scan a directory.</div>
				{:else}
					{#each $collections as col}
						<CollectionPanel collection={col}>
							<div class="coll-actions">
								<button class="btn-ghost btn-sm" on:click={() => openShare(col.slug)}>Share</button>
								<button class="btn-ghost btn-sm" on:click={() => handlePublishCollection(col.slug)}>Publish</button>
							</div>
						</CollectionPanel>
					{/each}
				{/if}
			</div>
		</div>
	</div>

	<ScanDialog bind:open={scanOpen} on:scanned={onScanned} />
	<ShareSettingsDialog bind:open={shareOpen} slug={shareSlug} />
{/if}

<style>
	/* Onboarding */
	.onboarding {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: 22px;
		padding: 40px;
		background: radial-gradient(circle at 50% 30%, var(--accent-soft) 0%, var(--bg) 60%);
	}

	.ob-logo {
		width: 56px; height: 56px;
		border-radius: 14px;
		background: linear-gradient(135deg, var(--accent) 0%, oklch(0.55 0.18 100) 100%);
		display: flex; align-items: center; justify-content: center;
		font-weight: 800; font-size: 28px; color: var(--accent-text);
		letter-spacing: -1.5px;
		box-shadow: 0 12px 40px -8px var(--accent-soft), inset 0 1px 0 oklch(1 0 0 / 0.2);
	}

	.ob-text { text-align: center; max-width: 380px; }

	.ob-title { font-size: 26px; font-weight: 700; letter-spacing: -0.6px; color: var(--fg); }

	.ob-sub { font-size: 14px; color: var(--fg-muted); margin-top: 8px; line-height: 1.55; }

	.ob-card {
		width: 400px;
		background: var(--bg-elev1);
		border: 1px solid var(--border);
		border-radius: 10px;
		padding: 22px;
	}

	.ob-card-head { margin-bottom: 16px; }

	.ob-card-title { font-size: 17px; font-weight: 600; color: var(--fg); margin-bottom: 6px; }

	.ob-card-sub { font-size: 12.5px; color: var(--fg-muted); margin-bottom: 18px; line-height: 1.5; }

	.ob-notice {
		background: var(--bg-elev2);
		border: 1px solid var(--border);
		border-radius: 8px;
		padding: 12px;
		margin-bottom: 16px;
		display: flex;
		gap: 10px;
		align-items: flex-start;
	}

	.ob-notice-icon { color: var(--accent); margin-top: 1px; flex-shrink: 0; }

	.ob-notice-text { font-size: 11.5px; color: var(--fg-muted); line-height: 1.5; }

	/* TopBar */
	.topbar {
		padding: 16px 24px;
		border-bottom: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
		background: var(--bg);
		flex-shrink: 0;
	}

	.topbar-title { font-size: 17px; font-weight: 600; color: var(--fg); letter-spacing: -0.3px; }
	.topbar-sub { font-size: 12px; color: var(--fg-muted); margin-top: 2px; }
	.topbar-actions { display: flex; gap: 8px; align-items: center; }

	/* Body layout */
	.body {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	/* Profile pane */
	.profile-pane {
		width: 320px;
		flex-shrink: 0;
		border-right: 1px solid var(--border);
		padding: 20px;
		overflow-y: auto;
		background: var(--bg);
	}

	.profile-header {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 18px;
	}

	.profile-header-info { flex: 1; min-width: 0; }

	.profile-name { font-size: 14px; font-weight: 600; color: var(--fg); }

	.fields { display: flex; flex-direction: column; gap: 12px; }

	.field { display: flex; flex-direction: column; gap: 5px; }

	.field-row { display: flex; gap: 10px; }

	.field-row .field { flex: 1; }

	.field-label-row { display: flex; justify-content: space-between; align-items: baseline; }

	.field-hint { font-size: 10.5px; color: var(--fg-dim); }

	/* Collections pane */
	.collections-pane {
		flex: 1;
		padding: 20px;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
	}

	.coll-head {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 4px;
		flex-shrink: 0;
	}

	.coll-title { font-size: 15px; font-weight: 600; }

	.coll-sub { font-size: 12px; color: var(--fg-muted); margin-top: 2px; }

	.stats {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 10px;
		margin: 16px 0 18px;
		flex-shrink: 0;
	}

	.stat {
		background: var(--bg-elev1);
		border: 1px solid var(--border);
		border-radius: 8px;
		padding: 10px 12px;
	}

	.stat-label {
		font-size: 10.5px;
		color: var(--fg-dim);
		text-transform: uppercase;
		letter-spacing: 0.8px;
		font-weight: 600;
	}

	.stat-value {
		font-size: 18px;
		font-weight: 600;
		color: var(--fg);
		margin-top: 3px;
		letter-spacing: -0.3px;
		font-feature-settings: 'tnum';
	}

	.coll-list { display: flex; flex-direction: column; gap: 10px; }

	.coll-actions {
		display: flex;
		gap: 4px;
		padding: 8px 10px;
		border-top: 1px solid var(--divider);
	}

	.empty { color: var(--fg-dim); font-size: 12.5px; text-align: center; padding: 32px 0; }

	/* Shared */
	.sect-label {
		font-size: 10.5px;
		color: var(--fg-dim);
		text-transform: uppercase;
		letter-spacing: 1.4px;
		font-weight: 600;
	}

	.mono {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--fg-muted);
	}

	.field-label {
		font-size: 11px;
		color: var(--fg-muted);
		font-weight: 500;
		letter-spacing: 0.1px;
	}

	.accent-dot { color: var(--accent); margin-left: 3px; }

	.hb-input {
		display: flex;
		align-items: center;
		padding: 0 11px;
		height: 34px;
		background: var(--bg-input);
		border: 1px solid var(--border);
		border-radius: 7px;
		font-family: var(--font-ui);
		font-size: 13px;
		color: var(--fg);
		outline: none;
		width: 100%;
		transition: border-color 0.1s;
	}

	.hb-input:focus { border-color: var(--accent); }

	.hb-input::placeholder { color: var(--fg-dim); }

	.hb-input-mono { font-family: var(--font-mono); }

	.hb-textarea {
		height: auto;
		min-height: 64px;
		align-items: flex-start;
		padding: 9px 11px;
		resize: vertical;
	}

	/* Add collection button — dedicated class to avoid global style interference */
	.btn-add {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 5px 11px;
		height: 28px;
		font-family: var(--font-ui);
		font-size: 12px;
		font-weight: 600;
		color: var(--accent-text);
		background: var(--accent);
		border: 1px solid var(--accent);
		border-radius: 7px;
		cursor: pointer;
		white-space: nowrap;
		line-height: 1;
		flex-shrink: 0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex; align-items: center; justify-content: center; gap: 6px;
		padding: 5px 11px; font-family: var(--font-ui); font-size: 13px; font-weight: 600;
		color: var(--accent-text); background: var(--accent);
		border: 1px solid var(--accent); border-radius: 7px;
		cursor: pointer; letter-spacing: -0.1px; white-space: nowrap; user-select: none;
		line-height: 1; height: 28px; flex-shrink: 0; text-decoration: none;
	}

	.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }

	.btn-ghost {
		display: inline-flex; align-items: center; justify-content: center; gap: 6px;
		padding: 5px 11px; font-family: var(--font-ui); font-size: 12px; font-weight: 500;
		color: var(--fg-muted); background: transparent;
		border: 1px solid transparent; border-radius: 7px;
		cursor: pointer; white-space: nowrap; user-select: none;
		line-height: 1; height: 28px; flex-shrink: 0;
	}

	.btn-ghost:disabled { opacity: 0.5; cursor: not-allowed; }

	.btn-sm { padding: 5px 11px; font-size: 12px; height: 28px; }

	.btn-full { width: 100%; height: auto; padding: 10px 14px; }
</style>
