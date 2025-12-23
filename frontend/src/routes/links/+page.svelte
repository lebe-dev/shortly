<script lang="ts">
	import { getUserUrls, deleteUrl, type UserUrlResponse } from '$lib/api/url';
	import { authStore, authLoading } from '$lib/stores/auth';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import Link2 from 'lucide-svelte/icons/link-2';
	import Copy from 'lucide-svelte/icons/copy';

	let urls: UserUrlResponse[] = [];
	let loading = true;

	$: if (!$authLoading) {
		if (!$authStore.authenticated) {
			window.location.href = '/login';
		} else if (loading && urls.length === 0) {
			loadUrls();
		}
	}

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

	function formatTimeRemaining(created: number, ttl: number): string {
		const expiresAt = created + ttl;
		const now = Math.floor(Date.now() / 1000);
		const remaining = expiresAt - now;

		if (remaining <= 0) {
			return $t('linksPage.expired');
		}

		const days = Math.floor(remaining / 86400);
		const hours = Math.floor((remaining % 86400) / 3600);
		const minutes = Math.floor((remaining % 3600) / 60);

		const parts = [];
		if (days > 0) parts.push(`${days}d`);
		if (hours > 0) parts.push(`${hours}h`);
		if (minutes > 0 || parts.length === 0) parts.push(`${minutes}m`);

		return parts.join(' ');
	}

	function formatExpiryDate(created: number, ttl: number): string {
		const expiresAt = (created + ttl) * 1000;
		return new Date(expiresAt).toLocaleString();
	}

	function formatCreatedDate(created: number): string {
		return new Date(created * 1000).toLocaleString();
	}

	function truncateUrl(url: string, maxLength: number = 50): string {
		if (url.length <= maxLength) return url;
		return url.substring(0, maxLength - 3) + '...';
	}
</script>

<svelte:head>
	<title>{$t('linksPage.title')}</title>
	<meta name="description" content={$t('linksPage.description')} />
</svelte:head>

<div class="w-full max-w-[1300px] rounded bg-white px-6 py-8 shadow md:px-24 dark:bg-gray-900">
	<div class="mb-6 flex items-center gap-2">
		<Link2 class="h-5 w-5" />
		<h1 class="text-xl font-bold">{$t('linksPage.header')}</h1>
	</div>

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
		<div class="overflow-x-auto">
			<table class="w-full border-collapse">
				<thead>
					<tr class="border-b border-gray-200 dark:border-gray-700">
						<th class="px-4 py-3 text-left font-semibold">{$t('linksPage.table.shortUrl')}</th>
						<th class="px-4 py-3 text-left font-semibold">{$t('linksPage.table.originalUrl')}</th>
						<th class="px-4 py-3 text-left font-semibold">{$t('linksPage.table.created')}</th>
						<th class="px-4 py-3 text-left font-semibold">{$t('linksPage.table.expires')}</th>
						<th class="px-4 py-3 text-left font-semibold">{$t('linksPage.table.actions')}</th>
					</tr>
				</thead>
				<tbody>
					{#each urls as url}
						<tr
							class="border-b border-gray-200 text-sm hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800"
						>
							<td class="px-4 py-3">
								<div class="flex items-center gap-2">
									<a
										href={url.url}
										class="text-blue-600 hover:underline dark:text-blue-400"
										target="_blank"
										rel="noopener noreferrer"
									>
										{url.url}
									</a>
									<Button
										variant="ghost"
										size="sm"
										class="h-6 w-6 p-0 hover:cursor-pointer"
										onclick={() => copyToClipboard(url.url)}
									>
										<Copy class="h-2 w-2" />
									</Button>
								</div>
							</td>
							<td class="px-4 py-3" title={url.original_url}>
								<div class="flex items-center gap-2">
									<a
										href={url.original_url}
										class="text-blue-600 hover:underline dark:text-blue-400"
										target="_blank"
										rel="noopener noreferrer"
									>
										{truncateUrl(url.original_url)}
									</a>
									<Button
										variant="ghost"
										size="sm"
										class="h-6 w-6 p-0 hover:cursor-pointer"
										onclick={() => copyToClipboard(url.original_url)}
									>
										<Copy class="h-2 w-2" />
									</Button>
								</div>
							</td>
							<td class="text-muted-foreground px-4 py-3 text-sm">
								{formatCreatedDate(url.created)}
							</td>
							<td
								class="text-muted-foreground px-4 py-3 text-sm"
								title={formatExpiryDate(url.created, url.ttl)}
							>
								{formatTimeRemaining(url.created, url.ttl)}
							</td>
							<td class="px-3 py-2 text-center">
								<Button
									variant="destructive"
									class="hover:cursor-pointer"
									size="sm"
									onclick={() => handleDelete(url.id)}
								>
									<Trash2 class="h-3 w-3" />
								</Button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
