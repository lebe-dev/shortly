<script lang="ts">
	import { resolve } from '$app/paths';
	import { getUserUrls, deleteUrl, type UserUrlResponse } from '$lib/api/url';
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button';
	import LimitsMeter from '$lib/components/LimitsMeter.svelte';
	import LinksTable, { type LinkRow } from '$lib/components/LinksTable.svelte';
	import Plus from 'lucide-svelte/icons/plus';

	let urls: UserUrlResponse[] = $state([]);
	let loading = $state(true);
	let hasCheckedAuth = $state(false);

	$effect(() => {
		if ($configStore && !hasCheckedAuth) {
			hasCheckedAuth = true;

			if (!$authStore.authenticated) {
				window.location.href = '/login';
			} else if (loading && urls.length === 0) {
				loadUrls();
			}
		}
	});

	const rows: LinkRow[] = $derived(
		urls.map((u) => ({
			id: u.id,
			shortUrl: u.url,
			originalUrl: u.original_url,
			created: u.created,
			ttl: u.ttl,
			lastAccessed: u.last_accessed
		}))
	);

	async function loadUrls() {
		loading = true;
		try {
			urls = await getUserUrls();
		} catch (e) {
			console.error('Failed to load URLs:', e);
			toast.error($t('linksPage.errors.loadFailed'));
		} finally {
			loading = false;
		}
	}

	async function handleDelete(urlId: string) {
		const confirmed = confirm($t('linksPage.deleteConfirm'));
		if (!confirmed) return;

		try {
			await deleteUrl(urlId);
			toast.success($t('linksPage.deleteSuccess'));
			await loadUrls();
			const { fetchConfig } = await import('$lib/api/config');
			await fetchConfig();
		} catch (e) {
			console.error('Failed to delete URL:', e);
			toast.error($t('linksPage.errors.deleteFailed'));
		}
	}

	async function copyToClipboard(url: string) {
		try {
			await navigator.clipboard.writeText(url);
			toast.success($t('linksPage.copySuccess'));
		} catch (e) {
			console.error('Failed to copy:', e);
			toast.error($t('linksPage.errors.copyFailed'));
		}
	}
</script>

<svelte:head>
	<title>{$t('linksPage.title')}</title>
	<meta name="description" content={$t('linksPage.description')} />
</svelte:head>

<div class="page animate-surface-in">
	<div class="flex flex-wrap items-end justify-between gap-6">
		<h1 class="page-title flex items-baseline gap-3.5">
			{$t('linksPage.header')}
			{#if !loading && urls.length > 0}
				<span class="text-muted-foreground font-mono text-lg font-medium tracking-[-0.02em]"
					>{urls.length}</span
				>
			{/if}
		</h1>
		<Button href={resolve('/')}>
			<Plus />
			{$t('linksPage.createNew')}
		</Button>
	</div>

	{#if $configStore && $configStore.features.createUrl.currentUrls !== undefined}
		<LimitsMeter config={$configStore.features.createUrl} class="mt-4" />
	{/if}

	<div class="mt-7">
		{#if loading}
			<div class="text-muted-foreground py-12 text-center">{$t('common.loading')}</div>
		{:else if urls.length === 0}
			<div class="surface-card flex flex-col items-center gap-5 px-6 py-16 text-center">
				<p class="text-muted-foreground text-lg">{$t('linksPage.empty')}</p>
				<Button href={resolve('/')}>{$t('linksPage.createFirst')}</Button>
			</div>
		{:else}
			<LinksTable {rows} onCopy={copyToClipboard} onDelete={handleDelete} />
		{/if}
	</div>
</div>
