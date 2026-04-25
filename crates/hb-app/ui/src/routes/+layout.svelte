<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { getIdentity, getProfile, getCollections, getContacts } from '$lib/api.js';
	import { identity, profile, collections, contacts, toastMessage } from '$lib/stores.js';

	onMount(async () => {
		try {
			identity.set(await getIdentity());
		} catch {
			// no identity yet — settings page will handle it
		}
		try {
			profile.set(await getProfile());
		} catch { /* ignore */ }
		try {
			collections.set(await getCollections());
		} catch { /* ignore */ }
		try {
			contacts.set(await getContacts());
		} catch { /* ignore */ }
	});

	const navItems = [
		{ href: '/', label: 'Home' },
		{ href: '/contacts', label: 'Contacts' },
		{ href: '/browse', label: 'Browse' },
		{ href: '/chat', label: 'Chat' },
		{ href: '/settings', label: 'Settings' },
	];

	$: currentPath = $page.url.pathname;
</script>

<div class="flex h-screen bg-surface-900 text-surface-50 overflow-hidden">
	<!-- Sidebar -->
	<nav class="w-48 flex-shrink-0 bg-surface-800 border-r border-surface-700 flex flex-col py-4">
		<div class="px-4 mb-6">
			<span class="text-lg font-bold text-primary-400">Hoardbook</span>
		</div>
		{#each navItems as item}
			<a
				href={item.href}
				class="px-4 py-2.5 text-sm transition-colors hover:bg-surface-700"
				class:bg-surface-700={currentPath === item.href}
				class:text-primary-400={currentPath === item.href}
				class:text-surface-300={currentPath !== item.href}
			>
				{item.label}
			</a>
		{/each}
	</nav>

	<!-- Main content -->
	<main class="flex-1 overflow-y-auto">
		<slot />
	</main>
</div>

<!-- Toast -->
{#if $toastMessage}
	<div
		class="fixed bottom-4 right-4 z-50 px-4 py-2 rounded shadow-lg text-sm font-medium"
		class:bg-success-500={$toastMessage.kind === 'success'}
		class:bg-error-500={$toastMessage.kind === 'error'}
	>
		{$toastMessage.text}
	</div>
{/if}
