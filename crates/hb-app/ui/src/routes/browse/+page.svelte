<script lang="ts">
	import { pasteKey, follow } from '$lib/api.js';
	import { contacts, toast } from '$lib/stores.js';
	import ProfileCard from '$lib/components/ProfileCard.svelte';
	import CollectionPanel from '$lib/components/CollectionPanel.svelte';
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

<div class="p-6 space-y-6 max-w-2xl">
	<div>
		<h2 class="text-lg font-semibold mb-3">Browse</h2>
		<div class="flex gap-2">
			<input
				class="input flex-1 font-mono text-sm"
				type="text"
				placeholder="Paste a Hoardbook ID (hb1_…)"
				bind:value={input}
				on:keydown={handleKeydown}
			/>
			<button
				class="btn variant-filled-primary"
				on:click={handleLookup}
				disabled={!input.trim() || loading}
			>
				{loading ? 'Looking up…' : 'Lookup'}
			</button>
		</div>
	</div>

	{#if result}
		<div class="space-y-3">
			<ProfileCard peer={result}>
				<button
					class="btn flex-shrink-0"
					class:variant-filled-primary={!alreadyFollowed}
					class:variant-ghost={alreadyFollowed}
					on:click={handleFollow}
					disabled={alreadyFollowed || following}
				>
					{alreadyFollowed ? 'Following' : following ? 'Following…' : 'Follow'}
				</button>
			</ProfileCard>

			{#if result.collections.length > 0}
				<h3 class="text-sm font-semibold text-surface-300">Collections</h3>
				{#each result.collections as col}
					<CollectionPanel collection={col} />
				{/each}
			{/if}
		</div>
	{/if}
</div>
