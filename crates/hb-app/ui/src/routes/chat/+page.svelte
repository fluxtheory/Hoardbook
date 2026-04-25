<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { contacts, identity, inboxMessages, sentMessages, toast } from '$lib/stores.js';
	import { getMessages, sendMessage } from '$lib/api.js';
	import type { CachedPeer, ReceivedMessage } from '$lib/types.js';

	let loading = false;
	let sending = false;
	let selectedPeer: CachedPeer | null = null;
	let draft = '';
	let threadEl: HTMLElement;

	$: myId = $identity?.hb_id ?? '';

	$: conversation = selectedPeer
		? [
				...$inboxMessages.filter((m) => m.from === selectedPeer!.hb_id),
				...$sentMessages.filter((m) => m.to === selectedPeer!.hb_id)
			].sort((a, b) => a.sent_at.localeCompare(b.sent_at))
		: [];

	onMount(async () => {
		await refreshInbox();
	});

	async function refreshInbox() {
		if (!$identity) return;
		loading = true;
		try {
			const msgs = await getMessages();
			inboxMessages.set(msgs);
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			loading = false;
		}
	}

	async function selectPeer(peer: CachedPeer) {
		selectedPeer = peer;
		await tick();
		scrollToBottom();
	}

	async function handleSend() {
		if (!selectedPeer || !draft.trim() || sending) return;
		sending = true;
		const content = draft.trim();
		draft = '';
		try {
			const sent = await sendMessage(selectedPeer.hb_id, content);
			sentMessages.update((prev) => [...prev, sent]);
			await tick();
			scrollToBottom();
		} catch (e) {
			toast(String(e), 'error');
			draft = content;
		} finally {
			sending = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSend();
		}
	}

	function scrollToBottom() {
		if (threadEl) threadEl.scrollTop = threadEl.scrollHeight;
	}

	function shortId(hb_id: string) {
		return hb_id.length > 16 ? hb_id.slice(0, 8) + '…' + hb_id.slice(-4) : hb_id;
	}

	function formatTime(iso: string) {
		return new Date(iso).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	function formatDate(iso: string) {
		return new Date(iso).toLocaleDateString([], { month: 'short', day: 'numeric' });
	}

	// Count unread (received) messages per contact for the sidebar badge.
	$: unreadCounts = Object.fromEntries(
		$contacts.map((c) => [c.hb_id, $inboxMessages.filter((m) => m.from === c.hb_id).length])
	);
</script>

{#if !$identity}
	<div class="flex items-center justify-center h-full text-surface-400">
		<div class="text-center">
			<p class="text-lg">No identity yet.</p>
			<a href="/settings" class="btn variant-filled-primary mt-4">Go to Settings →</a>
		</div>
	</div>
{:else}
	<div class="flex h-full">
		<!-- Contact list -->
		<div class="w-56 flex-shrink-0 border-r border-surface-700 flex flex-col">
			<div class="flex items-center justify-between px-4 py-3 border-b border-surface-700">
				<span class="text-sm font-semibold">Conversations</span>
				<button
					class="btn-icon btn-icon-sm variant-ghost"
					on:click={refreshInbox}
					title="Refresh inbox"
					disabled={loading}
				>
					{loading ? '…' : '↺'}
				</button>
			</div>

			{#if $contacts.length === 0}
				<div class="p-4 text-surface-400 text-xs">
					Add contacts via Browse to start chatting.
				</div>
			{:else}
				<ul class="flex-1 overflow-y-auto">
					{#each $contacts as peer}
						<li>
							<button
								class="w-full text-left px-4 py-3 text-sm transition-colors hover:bg-surface-700 flex items-center gap-2"
								class:bg-surface-700={selectedPeer?.hb_id === peer.hb_id}
								class:text-primary-400={selectedPeer?.hb_id === peer.hb_id}
								on:click={() => selectPeer(peer)}
							>
								<!-- Avatar -->
								<div
									class="w-7 h-7 rounded-full bg-primary-700 flex items-center justify-center text-xs font-bold flex-shrink-0"
								>
									{(peer.profile?.display_name ?? peer.hb_id)[0].toUpperCase()}
								</div>
								<div class="flex-1 min-w-0">
									<p class="truncate font-medium">
										{peer.profile?.display_name ?? shortId(peer.hb_id)}
									</p>
								</div>
								{#if unreadCounts[peer.hb_id] > 0}
									<span
										class="flex-shrink-0 bg-primary-500 text-white text-xs rounded-full px-1.5 py-0.5"
									>
										{unreadCounts[peer.hb_id]}
									</span>
								{/if}
							</button>
						</li>
					{/each}
				</ul>
			{/if}
		</div>

		<!-- Conversation area -->
		<div class="flex-1 flex flex-col overflow-hidden">
			{#if !selectedPeer}
				<div class="flex-1 flex items-center justify-center text-surface-400 flex-col gap-3 p-8">
					<p class="text-base">Select a contact to view the conversation.</p>
					<p class="text-xs text-center max-w-sm text-warning-400">
						⚠ Messages are stored unencrypted on relay servers and are publicly readable
						by anyone who knows your Hoardbook ID.
					</p>
				</div>
			{:else}
				<!-- Header -->
				<div class="flex items-center gap-3 px-5 py-3 border-b border-surface-700 flex-shrink-0">
					<div
						class="w-8 h-8 rounded-full bg-primary-700 flex items-center justify-center text-sm font-bold"
					>
						{(selectedPeer.profile?.display_name ?? selectedPeer.hb_id)[0].toUpperCase()}
					</div>
					<div>
						<p class="font-semibold text-sm">
							{selectedPeer.profile?.display_name ?? shortId(selectedPeer.hb_id)}
						</p>
						<p class="text-surface-400 text-xs font-mono">{shortId(selectedPeer.hb_id)}</p>
					</div>
					<span
						class="ml-auto text-xs px-2 py-0.5 rounded-full"
						class:bg-success-700={selectedPeer.online}
						class:text-success-200={selectedPeer.online}
						class:bg-surface-700={!selectedPeer.online}
						class:text-surface-400={!selectedPeer.online}
					>
						{selectedPeer.online ? 'Online' : 'Offline'}
					</span>
				</div>

				<!-- Privacy notice -->
				<div class="bg-warning-900 border-b border-warning-700 px-5 py-1.5 text-warning-300 text-xs flex-shrink-0">
					⚠ Messages are unencrypted and publicly readable on relay servers.
				</div>

				<!-- Thread -->
				<div class="flex-1 overflow-y-auto p-5 space-y-3" bind:this={threadEl}>
					{#if conversation.length === 0}
						<p class="text-surface-400 text-sm text-center pt-8">No messages yet. Say hello!</p>
					{:else}
						{#each conversation as msg, i}
							{@const isMe = msg.from === myId}
							{@const prevMsg = i > 0 ? conversation[i - 1] : null}
							{@const showDate =
								!prevMsg || formatDate(msg.sent_at) !== formatDate(prevMsg.sent_at)}

							{#if showDate}
								<div class="text-center text-surface-500 text-xs py-2">
									{formatDate(msg.sent_at)}
								</div>
							{/if}

							<div class="flex" class:justify-end={isMe}>
								<div
									class="max-w-xs lg:max-w-md px-3 py-2 rounded-2xl text-sm"
									class:bg-primary-600={isMe}
									class:text-white={isMe}
									class:rounded-br-sm={isMe}
									class:bg-surface-700={!isMe}
									class:text-surface-100={!isMe}
									class:rounded-bl-sm={!isMe}
								>
									<p class="whitespace-pre-wrap break-words">{msg.content}</p>
									<p
										class="text-xs mt-1 opacity-60 text-right"
									>
										{formatTime(msg.sent_at)}
									</p>
								</div>
							</div>
						{/each}
					{/if}
				</div>

				<!-- Compose -->
				<div class="border-t border-surface-700 p-3 flex gap-2 flex-shrink-0">
					<textarea
						class="textarea flex-1 resize-none text-sm"
						rows="2"
						placeholder="Type a message… (Enter to send, Shift+Enter for newline)"
						bind:value={draft}
						on:keydown={handleKeydown}
						disabled={sending}
					></textarea>
					<button
						class="btn variant-filled-primary self-end"
						on:click={handleSend}
						disabled={!draft.trim() || sending}
					>
						{sending ? '…' : 'Send'}
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}
