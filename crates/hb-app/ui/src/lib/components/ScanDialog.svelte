<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { open as openDialog } from '@tauri-apps/plugin-dialog';
	import { scanDirectory } from '../api.js';
	import { toast } from '../stores.js';
	import type { Collection } from '../types.js';

	const dispatch = createEventDispatcher<{ scanned: Collection; close: void }>();

	export let open = false;

	let path = '';

	async function browse() {
		const selected = await openDialog({ directory: true, multiple: false, title: 'Select directory' });
		if (selected) path = selected as string;
	}
	let pathAlias = '';
	let depth = 3;
	let excludeRaw = '.git, node_modules, __pycache__';
	let scanning = false;

	async function handleScan() {
		if (!path || !pathAlias) return;
		scanning = true;
		try {
			const exclude = excludeRaw
				.split(',')
				.map((s) => s.trim())
				.filter(Boolean);
			const collection = await scanDirectory({ path, path_alias: pathAlias, depth, exclude });
			dispatch('scanned', collection);
			close();
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			scanning = false;
		}
	}

	function close() {
		open = false;
		path = '';
		pathAlias = '';
		depth = 3;
		dispatch('close');
	}
</script>

{#if open}
	<!-- Backdrop -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		class="fixed inset-0 bg-black/60 z-40 flex items-center justify-center"
		on:click|self={close}
		on:keydown={(e) => e.key === 'Escape' && close()}
		role="presentation"
	>
		<div class="card bg-zinc-800 border border-zinc-600 shadow-2xl p-6 w-full max-w-md space-y-4 z-50">
			<h3 class="text-lg font-semibold">Add Collection</h3>

			<label class="label">
				<span>Directory path</span>
				<div class="flex gap-2">
					<input
						class="input flex-1"
						type="text"
						placeholder="C:\Movies or /mnt/data/books"
						bind:value={path}
					/>
					<button class="btn variant-ghost-surface shrink-0" type="button" on:click={browse}>
						Browse…
					</button>
				</div>
			</label>

			<label class="label">
				<span>Display name (alias)</span>
				<input
					class="input"
					type="text"
					placeholder="My Movie Collection"
					bind:value={pathAlias}
				/>
			</label>

			<label class="label">
				<span>Scan depth: {depth}</span>
				<input class="input" type="range" min="1" max="8" bind:value={depth} />
			</label>

			<label class="label">
				<span>Exclude (comma-separated)</span>
				<input class="input" type="text" bind:value={excludeRaw} />
			</label>

			<div class="flex justify-end gap-2 pt-2">
				<button class="btn variant-ghost" on:click={close}>Cancel</button>
				<button
					class="btn variant-filled-primary"
					on:click={handleScan}
					disabled={!path || !pathAlias || scanning}
				>
					{scanning ? 'Scanning…' : 'Scan'}
				</button>
			</div>
		</div>
	</div>
{/if}
