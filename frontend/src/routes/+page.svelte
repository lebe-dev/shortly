<script lang="ts">
	import { generateShortUrl } from '$lib/api/url';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { onMount, tick } from 'svelte';

	let urlInputRef: HTMLInputElement | null = $state(null);

	let url: string = $state('');
	let shortUrl: string = $state('');

	onMount(async () => {
		await tick();
		if (urlInputRef) {
			urlInputRef.focus();
		}
	});

	async function generateUrl() {
		await generateShortUrl(url)
			.then((data) => {
				shortUrl = data.url;
				console.log('short url:', shortUrl);
			})
			.catch((e) => {
				console.error(e);
			});
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
			class="mb-5 inline-block text-2xl font-medium"
		/>
		<div class="flex items-center justify-center gap-3">
			<Button size="lg" onclick={generateUrl}>GENERATE</Button>
		</div>
	{:else}
		<div>
			<div>Your short url:</div>
			<div class="mb-5 text-3xl">{shortUrl}</div>
			<CopyButton data={shortUrl} label="Copy" />
		</div>
	{/if}
</div>
