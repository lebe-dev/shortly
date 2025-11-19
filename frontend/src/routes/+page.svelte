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

	let urlInputRef: HTMLInputElement | null = $state(null);

	let url: string = $state('');
	let shortUrl: string = $state('');
	let shortUrlTtl = $state(0);

	const ttlFormatted = $derived(formatDuration(shortUrlTtl));

	onMount(async () => {
		await fetchConfig()
			.then((data) => {
				shortUrlTtl = data.shortUrlTtl;
			})
			.catch((e) => {
				console.error(e);
				toast.error('Unable to load app config');
			});

		await tick();
		if (urlInputRef) {
			urlInputRef.focus();
		}
	});

	async function generateUrl() {
		if (isUrlValid(url)) {
			await generateShortUrl(url)
				.then((data) => {
					shortUrl = data.url;
					console.log('short url:', shortUrl);
				})
				.catch((e) => {
					console.error(e);
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

<div class="border-secondary w-full max-w-3xl rounded border-3 px-24 py-10 text-center">
	{#if shortUrl === ''}
		<div class="mb-1 text-left">Paste long URL here</div>
		<Input
			bind:ref={urlInputRef}
			type="text"
			bind:value={url}
			placeholder="https://my.super-long-url.com/article/12345"
			class="mb-2 inline-block text-2xl font-medium"
		/>
		{#if ttlFormatted}
			<div class="text-muted-foreground mb-3 text-sm">Links will be stored for {ttlFormatted}</div>
		{/if}
		<div class="flex items-center justify-center gap-3">
			<Button size="lg" onclick={generateUrl}>GENERATE</Button>
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
