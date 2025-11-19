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

	const ttlFormatted = $derived(formatDuration(shortUrlTtl));

	onMount(async () => {
		await fetchConfig()
			.then((data) => {
				shortUrlTtl = data.shortUrlTtl;
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
		if (isUrlValid(url)) {
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
</script>

<svelte:head>
	<title>Create short url :: SHRTLY</title>
	<meta name="description" content="Create convenient short url from long one" />
</svelte:head>

<div class="h-96 w-full max-w-3xl rounded bg-white px-24 py-24 text-center shadow dark:bg-gray-900">
	{#if shortUrl === ''}
		<div class="mb-1 text-left">Paste long URL here</div>
		<Input
			bind:ref={urlInputRef}
			type="text"
			bind:value={url}
			disabled={inProgress}
			placeholder="https://my.super-long-url.com/article/12345"
			class="mb-2 inline-block text-2xl font-medium"
		/>
		{#if ttlFormatted}
			<div class="text-muted-foreground mb-3 text-sm">Links will be stored for {ttlFormatted}</div>
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
