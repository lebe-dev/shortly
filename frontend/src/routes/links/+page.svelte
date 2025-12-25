<script lang="ts">
	import { getUserUrls, deleteUrl, type UserUrlResponse } from '$lib/api/url';
	import { fetchConfig } from '$lib/api/config';
	import { authStore, authLoading } from '$lib/stores/auth';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button';
	import { Card, CardHeader, CardTitle, CardContent } from '$lib/components/ui/card';
	import ConsumptionDisplay from '$lib/components/ConsumptionDisplay.svelte';
	import type { AppConfig } from '$lib/domain/config';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import Link2 from 'lucide-svelte/icons/link-2';
	import Copy from 'lucide-svelte/icons/copy';
	import CornerDownRight from 'lucide-svelte/icons/corner-down-right';
	import { formatRemainingTime, formatCreatedDate, formatExpiryDate } from '$lib/date';

	let urls: UserUrlResponse[] = $state([]);
	let loading = $state(true);
	let config: AppConfig | null = $state(null);

	$effect(() => {
		if (!$authLoading) {
			if (!$authStore.authenticated) {
				window.location.href = '/login';
			} else if (loading && urls.length === 0) {
				loadUrls();
				loadConfig();
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

	async function loadConfig() {
		try {
			config = await fetchConfig();
		} catch (e) {
			console.error('Failed to load config:', e);
		}
	}

	async function handleDelete(urlId: string) {
		const confirmed = confirm($t('linksPage.deleteConfirm'));
		if (!confirmed) return;

		try {
			await deleteUrl(urlId);
			toast.success($t('linksPage.deleteSuccess'));
			await loadUrls();
			await loadConfig();
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

	function truncateUrl(url: string, maxLength: number = 50): string {
		if (url.length <= maxLength) return url;
		return url.substring(0, maxLength - 3) + '...';
	}

	function getShortUrlDisplay(url: UserUrlResponse): string {
		return url.custom_name || url.id;
	}
</script>

<svelte:head>
	<title>{$t('linksPage.title')}</title>
	<meta name="description" content={$t('linksPage.description')} />
</svelte:head>

<div
	class="w-full max-w-[1300px] rounded bg-white px-3 pt-8 pb-18 shadow md:px-14 dark:bg-gray-900"
>
	<div class="mb-6 flex items-center gap-2">
		<Link2 class="h-5 w-5" />
		<h1 class="text-xl font-bold">{$t('linksPage.header')}</h1>
	</div>

	{#if config}
		<div class="mb-6">
			<ConsumptionDisplay config={config.features.createUrl} variant="default" showHint={true} />
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
				<Card>
					<CardHeader>
						<CardTitle>
							<div class="flex items-center justify-between gap-2">
								<div class="flex items-center gap-2">
									<a
										href={url.url}
										class="text-blue-600 hover:underline dark:text-blue-400"
										target="_blank"
										rel="noopener noreferrer"
										title={url.url}
									>
										{getShortUrlDisplay(url)}
									</a>
									<Button
										variant="ghost"
										size="sm"
										class="h-6 w-6 p-0 text-gray-400 hover:cursor-pointer"
										title={$t('common.buttons.copy')}
										onclick={() => copyToClipboard(url.url)}
									>
										<Copy class="h-2 w-2" />
									</Button>
								</div>
								<Button
									variant="ghost"
									class="h-6 w-6 text-gray-400  hover:cursor-pointer hover:bg-red-100 hover:text-red-600 dark:text-gray-500 dark:hover:bg-red-900/20 dark:hover:text-red-400"
									size="sm"
									onclick={() => handleDelete(url.id)}
								>
									<Trash2 class="h-3 w-3" />
								</Button>
							</div>
						</CardTitle>
					</CardHeader>
					<CardContent>
						<div class="space-y-3">
							<!-- Original URL -->
							<div class="space-y-1">
								<div class="text-muted-foreground text-xs font-medium">
									{$t('linksPage.table.originalUrl')}
								</div>
								<div class="flex items-center gap-1">
									<CornerDownRight class="text-muted-foreground h-3 w-3 shrink-0" />
									<a
										href={url.original_url}
										class="truncate text-xs text-blue-600 hover:underline dark:text-blue-400"
										target="_blank"
										rel="noopener noreferrer"
										title={url.original_url}
									>
										{url.original_url}
									</a>
									<Button
										variant="ghost"
										size="sm"
										class="h-6 w-6 shrink-0 p-0 text-gray-400 hover:cursor-pointer"
										title={$t('common.buttons.copy')}
										onclick={() => copyToClipboard(url.original_url)}
									>
										<Copy class="h-2 w-2" />
									</Button>
								</div>
							</div>

							<!-- Created date and Expires -->
							<div
								class="flex items-start justify-between border-t border-gray-200 pt-3 dark:border-gray-700"
							>
								<div class="space-y-1">
									<div class="text-muted-foreground text-xs font-medium">
										{$t('linksPage.table.created')}
									</div>
									<div class="text-muted-foreground text-sm">
										{formatCreatedDate(url.created)}
									</div>
								</div>
								<div class="space-y-1 text-right">
									<div class="text-muted-foreground text-xs font-medium">
										{$t('linksPage.table.expires')}
									</div>
									<div
										class="text-muted-foreground text-sm"
										title={url.ttl === 0 ? '—' : formatExpiryDate(url.created, url.ttl)}
									>
										{url.ttl === 0 ? '—' : formatRemainingTime(url.ttl, url.created, $t, 'short')}
									</div>
								</div>
							</div>
						</div>
					</CardContent>
				</Card>
			{/each}
		</div>
	{/if}
</div>
