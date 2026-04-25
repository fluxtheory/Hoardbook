<script lang="ts">
	import { saveProfile, publishProfile, publishCollection } from '$lib/api.js';
	import { profile, collections, identity, toast } from '$lib/stores.js';
	import CollectionPanel from '$lib/components/CollectionPanel.svelte';
	import ScanDialog from '$lib/components/ScanDialog.svelte';
	import type { Collection, Profile } from '$lib/types.js';

	let scanOpen = false;
	let saving = false;
	let publishing = false;

	// Local form copy
	let form: Profile = {
		display_name: '',
		bio: undefined,
		tags: [],
		since: undefined,
		est_size: undefined,
		languages: [],
		contact_hint: undefined,
		email: undefined,
		updated: new Date().toISOString(),
	};

	$: if ($profile && !saving) {
		form = { ...$profile };
	}

	async function handleSave() {
		if (!form.display_name) return;
		saving = true;
		try {
			form.updated = new Date().toISOString();
			await saveProfile(form);
			profile.set(form);
			toast('Profile saved');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			saving = false;
		}
	}

	async function handlePublish() {
		publishing = true;
		try {
			await publishProfile();
			toast('Profile published to relay');
		} catch (e) {
			toast(String(e), 'error');
		} finally {
			publishing = false;
		}
	}

	async function handlePublishCollection(slug: string) {
		try {
			await publishCollection(slug);
			toast('Collection published');
		} catch (e) {
			toast(String(e), 'error');
		}
	}

	function onScanned(event: CustomEvent<Collection>) {
		const col = event.detail;
		collections.update((cols) => {
			const idx = cols.findIndex((c) => c.slug === col.slug);
			if (idx >= 0) {
				const next = [...cols];
				next[idx] = col;
				return next;
			}
			return [...cols, col];
		});
		toast(`Scanned "${col.path_alias}" — ${col.item_count} items`);
	}
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
		<!-- Left panel: Profile editor -->
		<div class="w-96 flex-shrink-0 border-r border-surface-700 p-6 overflow-y-auto space-y-4">
			<h2 class="text-lg font-semibold">My Profile</h2>

			<label class="label">
				<span>Display name <span class="text-error-400">*</span></span>
				<input class="input" type="text" placeholder="e.g. DataHoarder_42" bind:value={form.display_name} />
			</label>

			<label class="label">
				<span>Bio</span>
				<textarea class="textarea" rows="3" placeholder="What do you collect?" bind:value={form.bio}></textarea>
			</label>

			<div class="grid grid-cols-2 gap-3">
				<label class="label">
					<span>Since (year)</span>
					<input class="input" type="number" min="1990" max="2099" placeholder="2018" bind:value={form.since} />
				</label>
				<label class="label">
					<span>Est. total size</span>
					<input class="input" type="text" placeholder="12 TB" bind:value={form.est_size} />
				</label>
			</div>

			<label class="label">
				<span>Languages (comma-separated)</span>
				<input
					class="input"
					type="text"
					placeholder="en, jp, fr"
					value={form.languages.join(', ')}
					on:change={(e) => {
						form.languages = e.currentTarget.value.split(',').map((s) => s.trim()).filter(Boolean);
					}}
				/>
			</label>

			<label class="label">
				<span>Contact hint</span>
				<input class="input" type="text" placeholder="Matrix: @you:matrix.org" bind:value={form.contact_hint} />
			</label>

			<label class="label">
				<span>Email <span class="text-warning-400 text-xs">(public)</span></span>
				<input class="input" type="email" placeholder="you@example.com" bind:value={form.email} />
				<p class="text-surface-400 text-xs mt-0.5">Visible to anyone who views your profile.</p>
			</label>

			<div class="flex gap-2 pt-2">
				<button class="btn variant-ghost flex-1" on:click={handleSave} disabled={!form.display_name || saving}>
					{saving ? 'Saving…' : 'Save draft'}
				</button>
				<button class="btn variant-filled-primary flex-1" on:click={handlePublish} disabled={publishing}>
					{publishing ? 'Publishing…' : 'Publish'}
				</button>
			</div>
		</div>

		<!-- Right panel: Collections -->
		<div class="flex-1 p-6 overflow-y-auto space-y-4">
			<div class="flex items-center justify-between">
				<h2 class="text-lg font-semibold">My Collections</h2>
				<button class="btn variant-filled-primary btn-sm" on:click={() => (scanOpen = true)}>
					+ Add Collection
				</button>
			</div>

			{#if $collections.length === 0}
				<div class="text-surface-400 text-sm py-8 text-center">
					<p>No collections yet.</p>
					<p class="mt-1">Click "Add Collection" to scan a directory.</p>
				</div>
			{:else}
				{#each $collections as col}
					<CollectionPanel collection={col}>
						<div class="px-4 py-2 border-t border-surface-700 flex justify-end">
							<button
								class="btn variant-ghost-primary btn-sm"
								on:click={() => handlePublishCollection(col.slug)}
							>
								Publish
							</button>
						</div>
					</CollectionPanel>
				{/each}
			{/if}
		</div>
	</div>

	<ScanDialog bind:open={scanOpen} on:scanned={onScanned} />
{/if}
