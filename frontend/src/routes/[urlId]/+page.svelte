<script lang="ts">
	import { HttpError } from '$lib/api/error.js';
	import { fetchUrlById } from '$lib/api/url';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import LimitsMeter from '$lib/components/LimitsMeter.svelte';
	import { formatRemainingTime } from '$lib/date';
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { onMount } from 'svelte';
	import { t } from 'svelte-intl-precompile';

	let { data } = $props();

	let inProgress = $state(true);
	let url = $state('');
	let ttl = $state(0);
	let created = $state(0);
	let notFound: boolean = $state(false);

	const remainingTime = $derived(formatRemainingTime(ttl, created, $t, 'long'));

	onMount(async () => {
		await fetchUrlById(data.urlId)
			.then((response) => {
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
	});
</script>

<svelte:head>
	<title>{$t('urlPage.title')}</title>
	<meta name="description" content={$t('urlPage.description')} />
</svelte:head>

<div class="mx-auto w-full max-w-[808px] px-6 pt-16 pb-24 md:pt-20">
	<div class="surface-card animate-surface-in px-6 py-10 md:px-10">
		{#if inProgress}
			<div>{$t('common.loading')}</div>
		{:else if notFound || remainingTime === 'expired'}
			<h1 class="mb-4 text-3xl font-semibold tracking-[-0.03em]">
				{$t('urlPage.notFound.title')}
			</h1>
			<ul class="text-muted-foreground ms-4 list-disc space-y-1">
				<li>{$t('urlPage.notFound.wrongUrl')}</li>
				<li>{$t('urlPage.notFound.expired')}</li>
			</ul>
		{:else}
			<div class="animate-hero-in">
				<div class="eyebrow mb-3">
					{$t('urlPage.display.fullUrlLabel')}
				</div>
				<div
					class="mb-3 font-mono text-xl leading-snug tracking-[-0.02em] wrap-break-word md:text-2xl"
				>
					{url}
				</div>
				{#if remainingTime}
					<div class="text-muted-foreground mb-4 text-sm">
						{$t('urlPage.display.expirationInfo', { values: { remainingTime } })}
					</div>
				{/if}

				<CopyButton data={url} label={$t('common.buttons.copy')} />

				{#if $configStore && $authStore.authenticated}
					<LimitsMeter config={$configStore.features.createUrl} class="mt-8" />
				{/if}
			</div>
		{/if}
	</div>
</div>
