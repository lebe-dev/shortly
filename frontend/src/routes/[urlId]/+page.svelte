<script lang="ts">
	import { HttpError } from '$lib/api/error.js';
	import { fetchUrlById } from '$lib/api/url';
	import { fetchConfig } from '$lib/api/config';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import ConsumptionDisplay from '$lib/components/ConsumptionDisplay.svelte';
	import { formatRemainingTime } from '$lib/date';
	import { authStore } from '$lib/stores/auth';
	import type { AppConfig } from '$lib/domain/config';
	import { onMount } from 'svelte';
	import { t } from 'svelte-intl-precompile';

	let { data } = $props();

	let inProgress = $state(true);
	let url = $state('');
	let ttl = $state(0);
	let created = $state(0);
	let notFound: boolean = $state(false);
	let config: AppConfig | null = $state(null);

	const remainingTime = $derived(formatRemainingTime(ttl, created, $t, 'long'));

	onMount(async () => {
		console.log('url-id', data.urlId);

		await fetchUrlById(data.urlId)
			.then((response) => {
				console.log('url: ', response.url);
				url = response.url;
				ttl = response.ttl;
				created = response.created;
				inProgress = false;
			})
			.catch((e) => {
				if (e instanceof HttpError) {
					if (e.statusCode == 400) {
						notFound = true;
					}
				}

				inProgress = false;
			});

		if ($authStore.authenticated) {
			try {
				config = await fetchConfig();
			} catch (e) {
				console.error('Failed to load config:', e);
			}
		}
	});
</script>

<svelte:head>
	<title>{$t('urlPage.title')}</title>
	<meta name="description" content={$t('urlPage.description')} />
</svelte:head>

<div
	class="xs:w-[100px] h-80 w-[1300px] max-w-[1300px] rounded bg-white px-6 py-22 text-left shadow md:px-24 dark:bg-gray-900"
>
	{#if inProgress}
		<div>{$t('common.loading')}</div>
	{:else if notFound || remainingTime === 'expired'}
		<div class="mb-4 text-xl font-bold">{$t('urlPage.notFound.title')}</div>
		<ul class="ms-4 list-disc">
			<li>{$t('urlPage.notFound.wrongUrl')}</li>
			<li>{$t('urlPage.notFound.expired')}</li>
		</ul>
	{:else}
		<div>
			<div>{$t('urlPage.display.fullUrlLabel')}</div>
			<div class="mb-2 text-2xl wrap-break-word md:text-3xl">{url}</div>
			{#if remainingTime}
				<div class="text-muted-foreground mb-4 text-sm">
					{$t('urlPage.display.expirationInfo', { values: { remainingTime } })}
				</div>
			{/if}

			<CopyButton data={url} label={$t('common.buttons.copy')} />

			{#if config && $authStore.authenticated}
				<div
					class="mt-6 rounded border border-gray-200 bg-gray-50 p-3 dark:border-gray-700 dark:bg-gray-800"
				>
					<div class="text-muted-foreground mb-2 text-xs font-medium">Your Usage</div>
					<ConsumptionDisplay config={config.features.createUrl} variant="compact" />
				</div>
			{/if}
		</div>
	{/if}
</div>
