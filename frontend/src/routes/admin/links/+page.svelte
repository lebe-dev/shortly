<script lang="ts">
	import { deleteUrl } from '$lib/api/url';
	import { refreshConfig } from '$lib/api/config';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import LinksTable, { type LinkRow } from '$lib/components/LinksTable.svelte';
	import { Input } from '$lib/components/ui/input';
	import type { AdminUrlDto } from '$lib/domain/config';
	import Search from 'lucide-svelte/icons/search';

	let searchQuery = $state('');
	let onlyNamed = $state(false);
	let sortBy = $state<'created' | 'named'>('created');

	async function handleDelete(urlId: string) {
		const confirmed = confirm($t('linksPage.deleteConfirm'));
		if (!confirmed) return;

		try {
			await deleteUrl(urlId);
			toast.success($t('adminPage.deleteSuccess'));
			await refreshConfig();
		} catch (e) {
			console.error('Failed to delete URL:', e);
			toast.error($t('adminPage.errors.deleteFailed'));
		}
	}

	async function copyToClipboard(url: string) {
		try {
			await navigator.clipboard.writeText(url);
			toast.success($t('adminPage.copySuccess'));
		} catch (e) {
			console.error('Failed to copy:', e);
			toast.error($t('adminPage.errors.copyFailed'));
		}
	}

	function getShortUrl(url: AdminUrlDto): string {
		const base = $configStore?.baseUrl || '';
		const path = url.customName || url.id;
		return `${base}/${path}`;
	}

	const filteredAndSortedUrls = $derived.by(() => {
		if (!$configStore?.admin?.allUrls) return [];

		let urls = [...$configStore.admin.allUrls];

		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			urls = urls.filter((url) => {
				if (url.customName && url.customName.toLowerCase().includes(query)) {
					return true;
				}
				if (url.username && url.username.toLowerCase().includes(query)) {
					return true;
				}
				return false;
			});
		}

		if (onlyNamed) {
			urls = urls.filter((url) => url.customName !== null && url.customName !== undefined);
		}

		if (sortBy === 'created') {
			urls.sort((a, b) => b.created - a.created); // Most recent first
		} else if (sortBy === 'named') {
			urls.sort((a, b) => {
				const aHasName = a.customName ? 1 : 0;
				const bHasName = b.customName ? 1 : 0;
				if (aHasName !== bHasName) {
					return bHasName - aHasName; // Named first
				}
				return b.created - a.created;
			});
		}

		return urls;
	});

	const rows: LinkRow[] = $derived(
		filteredAndSortedUrls.map((u) => ({
			id: u.id,
			shortUrl: getShortUrl(u),
			originalUrl: u.originalUrl,
			created: u.created,
			ttl: u.ttl,
			lastAccessed: u.lastAccessed,
			username: u.username,
			userId: u.userId
		}))
	);
</script>

<svelte:head>
	<title>{$t('adminPage.title')} - {$t('adminPage.tabs.links')}</title>
	<meta name="description" content={$t('adminPage.description')} />
</svelte:head>

{#if !$configStore?.admin}
	<div class="py-12 text-center">
		<p class="text-muted-foreground text-lg">{$t('adminPage.errors.notAuthorized')}</p>
	</div>
{:else if $configStore.admin.allUrls.length === 0}
	<div class="surface-card px-6 py-16 text-center">
		<p class="text-muted-foreground text-lg">{$t('adminPage.empty')}</p>
	</div>
{:else}
	<div class="flex flex-wrap items-center gap-3">
		<label
			class="field border-input bg-card flex min-h-11 flex-[1_1_320px] items-center gap-2.5 rounded-xl border px-3.5"
		>
			<Search class="text-muted-foreground size-4 shrink-0" />
			<span class="sr-only">{$t('adminPage.search.placeholder')}</span>
			<Input
				type="search"
				bind:value={searchQuery}
				placeholder={$t('adminPage.search.placeholder')}
				class="h-11 rounded-none border-0 bg-transparent px-0"
			/>
		</label>

		<label
			class="border-input bg-card text-foreground/90 flex min-h-11 cursor-pointer items-center gap-2.5 rounded-xl border px-3.5 text-sm font-medium"
		>
			<input
				type="checkbox"
				bind:checked={onlyNamed}
				class="accent-primary-strong size-4 cursor-pointer"
			/>
			{$t('adminPage.filters.onlyNamed')}
		</label>

		<label
			class="field border-input bg-card flex min-h-11 items-center gap-2 rounded-xl border ps-3.5 pe-2 text-sm"
		>
			<span class="text-muted-foreground">{$t('adminPage.sort.label')}</span>
			<select
				bind:value={sortBy}
				class="text-foreground h-11 cursor-pointer bg-transparent pe-1 font-medium outline-none"
			>
				<option value="created">{$t('adminPage.sort.byCreated')}</option>
				<option value="named">{$t('adminPage.sort.namedFirst')}</option>
			</select>
		</label>
	</div>

	<div class="text-muted-foreground mt-4 flex gap-5 text-[13px]">
		<span
			>{$t('adminPage.items')}
			<b class="text-foreground font-mono font-semibold">{$configStore.admin.allUrls.length}</b
			></span
		>
		{#if filteredAndSortedUrls.length !== $configStore.admin.allUrls.length}
			<span
				>{$t('adminPage.filtered')}
				<b class="text-foreground font-mono font-semibold">{filteredAndSortedUrls.length}</b></span
			>
		{/if}
	</div>

	<div class="mt-3">
		{#if rows.length === 0}
			<div class="surface-card px-6 py-16 text-center">
				<p class="text-muted-foreground text-lg">{$t('adminPage.empty')}</p>
			</div>
		{:else}
			<LinksTable {rows} showUser onCopy={copyToClipboard} onDelete={handleDelete} />
		{/if}
	</div>
{/if}
