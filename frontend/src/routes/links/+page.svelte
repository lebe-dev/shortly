<script lang="ts">
	import { getUserUrls, deleteUrl, type UserUrlResponse } from '$lib/api/url';
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button';
	import ConsumptionDisplay from '$lib/components/ConsumptionDisplay.svelte';
	import UrlCard from '$lib/components/UrlCard.svelte';
	import Link2 from 'lucide-svelte/icons/link-2';
	import Plus from 'lucide-svelte/icons/plus';
	import ArrowLeft from 'lucide-svelte/icons/arrow-left';

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

<div
	class="w-full max-w-[1300px] rounded bg-white px-3 pt-8 pb-18 shadow md:px-14 dark:bg-gray-900"
>
	<a
		href="/"
		class="text-muted-foreground/60 hover:text-muted-foreground mb-2 inline-flex items-center gap-1 text-xs hover:underline"
	>
		<ArrowLeft class="h-3 w-3" />
		{$t('common.backToHome')}
	</a>
	<div class="mb-6 flex items-center justify-between gap-2">
		<div class="flex items-center gap-2">
			<Link2 class="h-5 w-5" />
			<h1 class="text-xl font-bold">{$t('linksPage.header')}</h1>
		</div>
		<a href="/">
			<Button variant="secondary" class="hover:cursor-pointer">
				<Plus class="h-4 w-4" />
				{$t('linksPage.createNew')}
			</Button>
		</a>
	</div>

	{#if $configStore}
		<div class="mb-6">
			<ConsumptionDisplay
				config={$configStore.features.createUrl}
				variant="default"
				showHint={true}
			/>
		</div>
	{/if}

	{#if loading}
		<div class="py-12 text-center">
			<p class="text-muted-foreground">{$t('common.loading')}</p>
		</div>
	{:else if urls.length === 0}
		<div class="py-12 text-center">
			<p class="text-muted-foreground text-lg">{$t('linksPage.empty')}</p>
			<a href="/" class="mt-4 inline-block">
				<Button>{$t('linksPage.createFirst')}</Button>
			</a>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
			{#each urls as url}
				<UrlCard
					id={url.id}
					url={url.url}
					originalUrl={url.original_url}
					created={url.created}
					ttl={url.ttl}
					customName={url.custom_name}
					lastAccessed={url.last_accessed}
					onDelete={handleDelete}
					onCopy={copyToClipboard}
					showUserInfo={false}
				/>
			{/each}
		</div>
	{/if}
</div>
