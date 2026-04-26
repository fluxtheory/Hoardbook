<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { getShareSettings, saveShareSettings } from '$lib/api.js';
	import { toast } from '$lib/stores.js';
	import type { ShareSettings } from '$lib/types.js';

	export let open = false;
	export let slug = '';

	const dispatch = createEventDispatcher();

	let loading = false;
	let saving = false;
	let settings: ShareSettings = {
		enabled: false,
		allowed_paths: [],
		speed_cap_kbps: undefined,
		download_limit: undefined,
		require_follow: false,
	};

	let speedCapStr = '';
	let downloadLimitStr = '';

	$: if (open && slug) loadSettings();

	async function loadSettings() {
		loading = true;
		try {
			const s = await getShareSettings(slug);
			settings = s;
			speedCapStr = s.speed_cap_kbps != null ? String(s.speed_cap_kbps) : '';
			downloadLimitStr = s.download_limit != null ? String(s.download_limit) : '';
		} catch {
			// no saved settings yet — use defaults
		} finally {
			loading = false;
		}
	}

	async function handleSave() {
		saving = true;
		try {
			const toSave: ShareSettings = {
				...settings,
				speed_cap_kbps: speedCapStr ? parseInt(speedCapStr) || undefined : undefined,
				download_limit: downloadLimitStr ? parseInt(downloadLimitStr) || undefined : undefined,
			};
			await saveShareSettings(slug, toSave);
			settings = toSave;
			toast('Share settings saved');
			dispatch('close');
			open = false;
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			saving = false;
		}
	}

	function handleBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			open = false;
			dispatch('close');
		}
	}
</script>

{#if open}
	<div class="backdrop" on:click={handleBackdrop} role="dialog" aria-modal="true">
		<div class="modal">
			<div class="modal-head">
				<div class="modal-title">Share settings</div>
				<div class="modal-sub">Control who can download from this collection</div>
			</div>

			{#if loading}
				<div class="modal-loading">Loading…</div>
			{:else}
				<div class="modal-body">
					<!-- Enable sharing -->
					<div class="row-group">
						<div class="toggle-row">
							<div>
								<div class="row-label">Enable file sharing</div>
								<div class="row-sub">Allow peers to request downloads from this collection</div>
							</div>
							<button
								class="toggle"
								class:toggle-on={settings.enabled}
								on:click={() => (settings.enabled = !settings.enabled)}
							>
								<span class="toggle-thumb" />
							</button>
						</div>
					</div>

					{#if settings.enabled}
						<div class="row-group">
							<div class="group-label">Restrictions</div>

							<div class="field">
								<label class="field-label" for="speed-cap">Speed cap (KB/s)</label>
								<input
									id="speed-cap"
									class="hb-input"
									type="number"
									min="1"
									placeholder="Unlimited"
									bind:value={speedCapStr}
								/>
							</div>

							<div class="field">
								<label class="field-label" for="dl-limit">Max simultaneous downloads</label>
								<input
									id="dl-limit"
									class="hb-input"
									type="number"
									min="1"
									placeholder="Unlimited"
									bind:value={downloadLimitStr}
								/>
							</div>

							<div class="toggle-row">
								<div>
									<div class="row-label">Followers only</div>
									<div class="row-sub">Only people you follow can download</div>
								</div>
								<button
									class="toggle"
									class:toggle-on={settings.require_follow}
									on:click={() => (settings.require_follow = !settings.require_follow)}
								>
									<span class="toggle-thumb" />
								</button>
							</div>
						</div>

						<div class="row-group">
							<div class="group-label">Allowed files</div>
							<div class="row-sub" style="margin-bottom:0">
								Leaving this empty allows downloading any file in the collection.
								Specific path restrictions will be configurable once a collection is scanned.
							</div>
						</div>
					{/if}
				</div>
			{/if}

			<div class="modal-foot">
				<button
					class="btn-ghost"
					on:click={() => { open = false; dispatch('close'); }}
				>Cancel</button>
				<button class="btn-primary" on:click={handleSave} disabled={saving || loading}>
					{saving ? 'Saving…' : 'Save'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.backdrop {
		position: fixed;
		inset: 0;
		background: oklch(0 0 0 / 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
	}

	.modal {
		background: var(--bg-elev1);
		border: 1px solid var(--border);
		border-radius: 12px;
		width: 420px;
		max-width: calc(100vw - 32px);
		box-shadow: 0 24px 60px oklch(0 0 0 / 0.5);
		overflow: hidden;
	}

	.modal-head {
		padding: 20px 20px 16px;
		border-bottom: 1px solid var(--divider);
	}

	.modal-title { font-size: 15px; font-weight: 600; color: var(--fg); }
	.modal-sub { font-size: 12px; color: var(--fg-muted); margin-top: 3px; }

	.modal-loading { padding: 32px; text-align: center; color: var(--fg-dim); font-size: 13px; }

	.modal-body { padding: 16px 20px; display: flex; flex-direction: column; gap: 16px; }

	.row-group {
		background: var(--bg-elev2);
		border: 1px solid var(--border);
		border-radius: 8px;
		padding: 12px 14px;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.group-label {
		font-size: 10.5px;
		color: var(--fg-dim);
		text-transform: uppercase;
		letter-spacing: 0.8px;
		font-weight: 600;
	}

	.toggle-row { display: flex; justify-content: space-between; align-items: center; gap: 12px; }

	.row-label { font-size: 13px; font-weight: 500; color: var(--fg); }

	.row-sub { font-size: 11.5px; color: var(--fg-dim); margin-top: 2px; line-height: 1.5; }

	.field { display: flex; flex-direction: column; gap: 5px; }

	.field-label { font-size: 11px; color: var(--fg-muted); font-weight: 500; }

	.hb-input {
		height: 34px;
		padding: 0 11px;
		background: var(--bg-input);
		border: 1px solid var(--border);
		border-radius: 7px;
		font-family: var(--font-ui);
		font-size: 13px;
		color: var(--fg);
		outline: none;
		width: 100%;
	}
	.hb-input::placeholder { color: var(--fg-dim); }
	.hb-input:focus { border-color: var(--accent); }

	.toggle {
		width: 30px; height: 17px;
		border-radius: 99px;
		background: var(--bg-elev3);
		border: 1px solid var(--border-strong);
		position: relative;
		flex-shrink: 0;
		cursor: pointer;
		transition: background 0.15s, border-color 0.15s;
	}
	.toggle-on { background: var(--accent); border-color: var(--accent); }
	.toggle-thumb {
		position: absolute;
		top: 1px; left: 1px;
		width: 13px; height: 13px;
		border-radius: 50%;
		background: var(--fg-muted);
		transition: left 0.15s, background 0.15s;
	}
	.toggle-on .toggle-thumb { left: 14px; background: var(--accent-text); }

	.modal-foot {
		padding: 14px 20px;
		border-top: 1px solid var(--divider);
		display: flex;
		justify-content: flex-end;
		gap: 8px;
	}

	.btn-primary {
		display: inline-flex; align-items: center; gap: 6px;
		padding: 7px 16px; font-family: var(--font-ui); font-size: 13px; font-weight: 600;
		color: var(--accent-text); background: var(--accent);
		border: 1px solid var(--accent); border-radius: 7px;
		cursor: pointer; line-height: 1;
	}
	.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }

	.btn-ghost {
		display: inline-flex; align-items: center; gap: 6px;
		padding: 7px 16px; font-family: var(--font-ui); font-size: 13px; font-weight: 500;
		color: var(--fg-muted); background: transparent;
		border: 1px solid transparent; border-radius: 7px;
		cursor: pointer; line-height: 1;
	}
</style>
