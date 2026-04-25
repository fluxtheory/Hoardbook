<script lang="ts">
	import { refreshContact } from '$lib/api.js';
	import { contacts, toast } from '$lib/stores.js';
	import ProfileCard from '$lib/components/ProfileCard.svelte';
	import CollectionPanel from '$lib/components/CollectionPanel.svelte';

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
</script>

<div class="p-6 space-y-4">
	<h2 class="text-lg font-semibold">Contacts</h2>

	{#if $contacts.length === 0}
		<p class="text-surface-400 text-sm">
			No contacts yet. Go to <a href="/browse" class="text-primary-400 underline">Browse</a> to follow someone.
		</p>
	{:else}
		{#each $contacts as peer}
			<div class="space-y-2">
				<ProfileCard {peer}>
					<div class="flex flex-col gap-1 flex-shrink-0">
						<button
							class="btn variant-ghost btn-sm"
							on:click={() => handleRefresh(peer.hb_id)}
							disabled={refreshing === peer.hb_id}
						>
							{refreshing === peer.hb_id ? '…' : 'Refresh'}
						</button>
						<button
							class="btn variant-ghost btn-sm"
							on:click={() => (expanded = expanded === peer.hb_id ? null : peer.hb_id)}
						>
							{expanded === peer.hb_id ? 'Collapse' : 'Collections'}
						</button>
					</div>
				</ProfileCard>

				{#if expanded === peer.hb_id}
					<div class="pl-4 space-y-2">
						{#if peer.collections.length === 0}
							<p class="text-surface-500 text-sm">No collections published.</p>
						{:else}
							{#each peer.collections as col}
								<CollectionPanel collection={col} />
							{/each}
						{/if}
					</div>
				{/if}
			</div>
		{/each}
	{/if}
</div>
