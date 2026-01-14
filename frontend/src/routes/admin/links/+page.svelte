<script lang="ts">
	import { deleteUrl } from '$lib/api/url';
	import { refreshConfig } from '$lib/api/config';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import UrlCard from '$lib/components/UrlCard.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
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
	<div class="py-12 text-center">
		<p class="text-muted-foreground text-lg">{$t('adminPage.empty')}</p>
	</div>
{:else}
	<!-- Controls -->
	<div class="mb-6 space-y-4">
		<!-- Search -->
		<div class="relative">
			<Search class="text-muted-foreground absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2" />
			<Input
				type="text"
				bind:value={searchQuery}
				placeholder={$t('adminPage.search.placeholder')}
				class="pl-10"
			/>
		</div>

		<!-- Filters and Sort -->
		<div class="flex flex-wrap items-center gap-4">
			<!-- Filters -->
			<div class="flex items-center gap-4">
				<Label class="flex cursor-pointer items-center gap-2">
					<input
						type="checkbox"
						bind:checked={onlyNamed}
						class="h-4 w-4 cursor-pointer rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800"
					/>
					<span class="text-sm">{$t('adminPage.filters.onlyNamed')}</span>
				</Label>
			</div>

			<!-- Sort -->
			<div class="flex items-center gap-2">
				<Label for="sort" class="text-sm">{$t('adminPage.sort.label')}:</Label>
				<select
					id="sort"
					bind:value={sortBy}
					class="border-input bg-background focus:border-primary h-10 cursor-pointer rounded-sm border-2 px-3 py-1 text-sm outline-none dark:bg-gray-800"
				>
					<option value="created">{$t('adminPage.sort.byCreated')}</option>
					<option value="named">{$t('adminPage.sort.namedFirst')}</option>
				</select>
			</div>
		</div>
	</div>

	<!-- Item count -->
	<div class="mb-4 ps-1 text-xs text-gray-400 dark:text-gray-500">
		{#if filteredAndSortedUrls.length !== $configStore.admin.allUrls.length}
			{$t('adminPage.filtered')}: {filteredAndSortedUrls.length} / {$t('adminPage.items')}: {$configStore
				.admin.allUrls.length}
		{:else}
			{$t('adminPage.items')}: {$configStore.admin.allUrls.length}
		{/if}
	</div>

	<!-- Results -->
	{#if filteredAndSortedUrls.length === 0}
		<div class="py-12 text-center">
			<p class="text-muted-foreground text-lg">{$t('adminPage.empty')}</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
			{#each filteredAndSortedUrls as url}
				<UrlCard
					id={url.id}
					url={getShortUrl(url)}
					originalUrl={url.originalUrl}
					created={url.created}
					ttl={url.ttl}
					customName={url.customName}
					username={url.username}
					userId={url.userId}
					lastAccessed={url.lastAccessed}
					onDelete={handleDelete}
					onCopy={copyToClipboard}
					showUserInfo={true}
				/>
			{/each}
		</div>
	{/if}
{/if}
