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

	let inProgress = $state(true);

	let urlInputRef: HTMLInputElement | null = $state(null);

	let url: string = $state('');
	let shortUrl: string = $state('');
	let shortUrlTtl = $state(0);
	let maxUrlLength = $state(2048);

	const ttlFormatted = $derived(formatDuration(shortUrlTtl));

	onMount(async () => {
		await fetchConfig()
			.then((data) => {
				shortUrlTtl = data.shortUrlTtl;
				maxUrlLength = data.maxUrlLength;
				inProgress = false;
			})
			.catch((e) => {
				console.error(e);
				toast.error('Unable to load app config');
				inProgress = false;
			});

		await tick();
		if (urlInputRef) {
			urlInputRef.focus();
		}
	});

	async function generateUrl() {
		if (url.length >= maxUrlLength) {
			toast.error('Max URL length exceeded: ' + maxUrlLength);
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
				toast.error('Invalid URL');
				if (urlInputRef) {
					urlInputRef.focus();
				}
			}
		}
	}
</script>

<svelte:head>
	<title>Create short url :: SHRTLY</title>
	<meta name="description" content="Create convenient short url from long one" />
</svelte:head>

<div class="h-80 w-full rounded bg-white px-8 py-22 text-center shadow md:px-24 dark:bg-gray-900">
	{#if shortUrl === ''}
		<div class="mb-1 text-left">Paste long URL here</div>
		<Input
			bind:ref={urlInputRef}
			type="text"
			bind:value={url}
			disabled={inProgress}
			placeholder="https://my.long-url.com"
			maxlength={maxUrlLength}
			class="md:text-md mb-2 w-full text-lg"
		/>
		{#if ttlFormatted}
			<div class="text-muted-foreground mb-3 text-sm">Links will be stored for {ttlFormatted}</div>
		{:else}
			<div class="text-muted-foreground mb-3 text-sm">...</div>
		{/if}
		<div class="flex items-center justify-center gap-3">
			<Button size="lg" disabled={inProgress} onclick={generateUrl}>GENERATE</Button>
		</div>
	{:else}
		<div>
			<div>Your short url:</div>
			<div class="mb-2 text-3xl">{shortUrl}</div>
			{#if ttlFormatted}
				<div class="text-muted-foreground mb-4 text-sm">
					This link will expire in {ttlFormatted}
				</div>
			{/if}
			<CopyButton data={shortUrl} label="Copy" />
		</div>
	{/if}
</div>
