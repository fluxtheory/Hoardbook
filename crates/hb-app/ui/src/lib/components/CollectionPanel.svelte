<script lang="ts">
	import type { Collection } from '../types.js';
	import DirItem from './DirItem.svelte';

	export let collection: Collection;

	let expanded = false;
</script>

<div class="border border-surface-700 rounded-lg overflow-hidden">
	<button
		class="w-full flex items-center justify-between px-4 py-3 bg-surface-800 hover:bg-surface-700 transition-colors text-left"
		on:click={() => (expanded = !expanded)}
	>
		<div>
			<div class="font-medium">{collection.path_alias}</div>
			<div class="text-xs text-surface-400 mt-0.5">
				{collection.item_count} items
				{#if collection.est_size} · ~{collection.est_size}{/if}
				{#if collection.content_type.length > 0} · {collection.content_type.join(', ')}{/if}
			</div>
		</div>
		<span class="text-surface-400 text-sm">{expanded ? '▲' : '▼'}</span>
	</button>

	{#if expanded}
		<div class="p-3 bg-surface-900 max-h-96 overflow-y-auto">
			{#if collection.listing.length === 0}
				<p class="text-surface-500 text-sm">Empty directory</p>
			{:else}
				<ul class="text-sm space-y-0.5">
					{#each collection.listing as item}
						<DirItem {item} depth={0} />
					{/each}
				</ul>
			{/if}
		</div>
	{/if}

	<slot />
</div>
