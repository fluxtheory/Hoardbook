<script lang="ts">
	import { onMount } from 'svelte';
	import { generateKeypair, getHbId, getSettings, saveSettings } from '$lib/api.js';
	import { identity, toast } from '$lib/stores.js';

	let generating = false;
	let copied = false;

	let relayUrls: string[] = [];
	let newRelay = '';
	let savingRelays = false;

	onMount(async () => {
		try {
			const s = await getSettings();
			relayUrls = s.relay_urls;
		} catch { /* no settings file yet */ }
	});

	async function handleGenerate() {
		generating = true;
		try {
			const info = await generateKeypair();
			identity.set(info);
			toast('Keypair generated');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			generating = false;
		}
	}

	async function handleCopy() {
		try {
			const id = await getHbId();
			await navigator.clipboard.writeText(id);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		} catch {
			toast('Could not copy to clipboard', 'error');
		}
	}

	function addRelay() {
		const url = newRelay.trim().replace(/\/$/, '');
		if (!url || relayUrls.includes(url)) return;
		relayUrls = [...relayUrls, url];
		newRelay = '';
	}

	function removeRelay(url: string) {
		relayUrls = relayUrls.filter((u) => u !== url);
	}

	async function handleSaveRelays() {
		savingRelays = true;
		try {
			await saveSettings({ relay_urls: relayUrls });
			toast('Relay settings saved');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			savingRelays = false;
		}
	}
</script>

<div class="p-6 space-y-8 max-w-lg">
	<h2 class="text-lg font-semibold">Settings</h2>

	<!-- Identity -->
	<section class="space-y-3">
		<h3 class="text-sm font-semibold text-surface-400 uppercase tracking-wide">Identity</h3>

		{#if $identity}
			<div class="card bg-surface-800 p-4 space-y-3">
				<div>
					<div class="text-xs text-surface-400 mb-1">Your Hoardbook ID</div>
					<div class="font-mono text-sm break-all bg-surface-900 rounded px-3 py-2">
						{$identity.hb_id}
					</div>
				</div>
				<button class="btn variant-ghost btn-sm" on:click={handleCopy}>
					{copied ? 'Copied!' : 'Copy ID'}
				</button>
				<p class="text-xs text-surface-500">Share this ID with others so they can look you up.</p>
			</div>
		{:else}
			<div class="card bg-surface-800 p-4 space-y-3">
				<p class="text-sm text-surface-300">No identity yet. Generate a keypair to get started.</p>
				<button class="btn variant-filled-primary" on:click={handleGenerate} disabled={generating}>
					{generating ? 'Generating…' : 'Generate Keypair'}
				</button>
			</div>
		{/if}
	</section>

	<!-- Relays -->
	<section class="space-y-3">
		<h3 class="text-sm font-semibold text-surface-400 uppercase tracking-wide">Relays</h3>
		<div class="card bg-surface-800 p-4 space-y-3">
			{#if relayUrls.length === 0}
				<p class="text-sm text-surface-500">No relays configured. Add one to publish and browse.</p>
			{:else}
				<ul class="space-y-1">
					{#each relayUrls as url}
						<li class="flex items-center gap-2">
							<span class="font-mono text-sm text-surface-300 flex-1 truncate">{url}</span>
							<button class="btn variant-ghost-error btn-sm" on:click={() => removeRelay(url)}>✕</button>
						</li>
					{/each}
				</ul>
			{/if}

			<div class="flex gap-2">
				<input
					class="input flex-1 font-mono text-sm"
					type="text"
					placeholder="http://localhost:3000"
					bind:value={newRelay}
					on:keydown={(e) => e.key === 'Enter' && addRelay()}
				/>
				<button class="btn variant-ghost" on:click={addRelay} disabled={!newRelay.trim()}>Add</button>
			</div>

			<button
				class="btn variant-filled-primary w-full"
				on:click={handleSaveRelays}
				disabled={savingRelays}
			>
				{savingRelays ? 'Saving…' : 'Save Relays'}
			</button>
		</div>
	</section>
</div>
