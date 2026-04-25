<script lang="ts">
	import type { DirectoryItem } from '../types.js';
	import DirItem from './DirItem.svelte';

	export let item: DirectoryItem;
	export let depth: number = 0;

	let open = false;
	$: isFolder = item.item_type === 'Folder';
</script>

<li style="padding-left: {depth * 14}px" class="leading-6">
	{#if isFolder}
		<button
			class="flex items-center gap-1 hover:text-primary-400 transition-colors w-full text-left"
			on:click={() => (open = !open)}
		>
			<span class="text-surface-400 text-xs w-3">{open ? '▼' : '▶'}</span>
			<span class="text-surface-300">📁</span>
			<span class="font-medium">{item.name}</span>
			{#if item.children.length > 0}
				<span class="text-surface-500 text-xs ml-1">({item.children.length})</span>
			{/if}
		</button>
		{#if open && item.children.length > 0}
			<ul class="mt-0.5">
				{#each item.children as child}
					<DirItem item={child} depth={depth + 1} />
				{/each}
			</ul>
		{/if}
	{:else}
		<div class="flex items-center gap-1 text-surface-400">
			<span class="w-3"></span>
			<span class="text-surface-500">📄</span>
			<span>{item.name}</span>
			{#if item.size}
				<span class="text-surface-600 text-xs ml-auto">{item.size}</span>
			{/if}
		</div>
	{/if}
</li>
