<script lang="ts">
	import type { CachedPeer } from '../types.js';

	export let peer: CachedPeer;

	$: initial = peer.profile?.display_name?.[0]?.toUpperCase() ?? '?';
	$: shortId = peer.hb_id.length > 14
		? peer.hb_id.slice(0, 8) + '…' + peer.hb_id.slice(-4)
		: peer.hb_id;
</script>

<div class="card p-4 flex gap-4 items-start">
	<!-- Avatar -->
	<div class="avatar">
		<div class="w-12 h-12 rounded-full bg-primary-500 flex items-center justify-center text-xl font-bold text-white">
			{initial}
		</div>
	</div>

	<!-- Info -->
	<div class="flex-1 min-w-0">
		<div class="flex items-center gap-2 flex-wrap">
			<span class="font-semibold truncate">
				{peer.profile?.display_name ?? 'Unknown'}
			</span>
			{#if peer.online}
				<span class="badge variant-filled-success text-xs">Online</span>
			{:else}
				<span class="badge variant-ghost-surface text-xs">Offline</span>
			{/if}
		</div>

		<div class="text-xs text-surface-400 font-mono mt-0.5">{shortId}</div>

		{#if peer.profile?.bio}
			<p class="text-sm text-surface-300 mt-1 line-clamp-2">{peer.profile.bio}</p>
		{/if}

		<div class="flex gap-3 mt-1 text-xs text-surface-400 flex-wrap">
			{#if peer.profile?.est_size}
				<span>~{peer.profile.est_size}</span>
			{/if}
			{#if peer.collections.length > 0}
				<span>{peer.collections.length} collection{peer.collections.length !== 1 ? 's' : ''}</span>
			{/if}
		</div>
	</div>

	<slot />
</div>
