<script lang="ts">
	import { fetchConfig } from '$lib/api/config';
	import { generateShortUrl } from '$lib/api/url';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { formatDuration } from '$lib/date';
	import { isUrlValid } from '$lib/validator';
	import { onMount, tick } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-intl-precompile';
	import type { AppConfig } from '$lib/domain/config';
	import { authStore, authLoading } from '$lib/stores/auth';

	let inProgress = $state(true);

	let urlInputRef: HTMLInputElement | null = $state(null);

	let url: string = $state('');
	let shortUrl: string = $state('');
	let config: AppConfig | null = $state(null);
	let shortUrlTtl = $state(0);
	let maxUrlLength = $state(2048);

	const ttlFormatted = $derived(formatDuration(shortUrlTtl));

	onMount(async () => {
		await fetchConfig()
			.then((data) => {
				config = data;
				shortUrlTtl = data.shortUrlTtl;
				maxUrlLength = data.maxUrlLength;
				inProgress = false;
			})
			.catch((e) => {
				console.error(e);
				toast.error($t('homePage.errors.configLoadFailed'));
				inProgress = false;
			});

		await tick();
		if (urlInputRef) {
			urlInputRef.focus();
		}
	});

	async function generateUrl() {
		if (url.length >= maxUrlLength) {
			toast.error($t('homePage.errors.urlTooLong', { values: { maxLength: maxUrlLength } }));
		} else {
			if (isUrlValid(url, maxUrlLength)) {
				inProgress = true;
				await generateShortUrl(url)
					.then((data) => {
						shortUrl = data.url;
						console.log('short url:', shortUrl);
						inProgress = false;
					})
					.catch((e) => {
						console.error(e);
						inProgress = false;
					});
			} else {
				toast.error($t('homePage.errors.invalidUrl'));
				if (urlInputRef) {
					urlInputRef.focus();
				}
			}
		}
	}
</script>

<svelte:head>
	<title>{$t('homePage.title')}</title>
	<meta name="description" content={$t('homePage.description')} />
</svelte:head>

<div
	class="xs:w-[100px] h-80 w-[1300px] max-w-[1300px] rounded bg-white px-6 py-22 text-center shadow md:px-24 dark:bg-gray-900"
>
	{#if !config}
		<div class="text-muted-foreground">{$t('common.loading')}</div>
	{:else if $authLoading}
		<div class="text-muted-foreground">{$t('common.loadingEllipsis')}</div>
	{:else if !config.features.createUrlEnabled}
		<div class="text-muted-foreground text-lg">
			{$t('homePage.errors.serviceTitle')}
		</div>
	{:else if config.features.createUrlAuthOnly && !$authStore.authenticated}
		<div class="flex flex-col items-center justify-center gap-3">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="text-muted-foreground lucide lucide-user-lock-icon lucide-user-lock"
			>
				<circle cx="10" cy="7" r="4" />
				<path d="M10.3 15H7a4 4 0 0 0-4 4v2" />
				<path d="M15 15.5V14a2 2 0 0 1 4 0v1.5" />
				<rect width="8" height="5" x="13" y="16" rx=".899" />
			</svg>
			<div class="text-muted-foreground text-lg">
				{$t('homePage.errors.authRequired')}
			</div>
		</div>
	{:else if shortUrl === ''}
		<div class="mb-1 text-left">{$t('homePage.form.label')}</div>
		<Input
			bind:ref={urlInputRef}
			type="text"
			bind:value={url}
			disabled={inProgress}
			placeholder={$t('homePage.form.placeholder')}
			maxlength={maxUrlLength}
			class="md:text-md mb-2 w-full text-lg"
		/>
		{#if ttlFormatted}
			<div class="text-muted-foreground mb-3 text-sm">
				{$t('homePage.form.storageInfo', { values: { ttl: ttlFormatted } })}
			</div>
		{:else}
			<div class="text-muted-foreground mb-3 text-sm">{$t('common.loadingEllipsis')}</div>
		{/if}
		<div class="flex items-center justify-center gap-3">
			<Button size="lg" disabled={inProgress} onclick={generateUrl}
				>{$t('common.buttons.generate')}</Button
			>
		</div>
	{:else}
		<div>
			<div>{$t('homePage.result.title')}</div>
			<div class="mb-2 text-3xl">{shortUrl}</div>
			{#if ttlFormatted}
				<div class="text-muted-foreground mb-4 text-sm">
					{$t('homePage.result.expirationInfo', { values: { ttl: ttlFormatted } })}
				</div>
			{/if}
			<CopyButton data={shortUrl} label={$t('common.buttons.copy')} />
		</div>
	{/if}
</div>
