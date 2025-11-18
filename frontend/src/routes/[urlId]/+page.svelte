<script lang="ts">
	import { HttpError } from '$lib/api/error.js';
	import { fetchUrlById } from '$lib/api/url';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';

	let { data } = $props();

	let inProgress = $state(true);
	let url = $state('');
	let notFound: boolean = $state(false);

	onMount(async () => {
		console.log('url-id', data.urlId);

		await fetchUrlById(data.urlId)
			.then((response) => {
				console.log('url: ', response.url);
				url = response.url;
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
	<title>URL page</title>
	<meta name="description" content="Page with original URL" />
</svelte:head>

<div class="border-secondary w-full max-w-3xl rounded border-3 px-24 py-10 text-left">
	{#if inProgress}
		<div>Loading...</div>
	{:else if notFound}
		<div class="mb-4 text-xl font-bold">URL was not found</div>
		<ul class="ms-4 list-disc">
			<li>Wrong URL</li>
			<li>URL has been expired</li>
		</ul>
	{:else}
		<div>
			<div>Full URL:</div>
			<div class="mb-4 text-3xl wrap-break-word">{url}</div>

			<CopyButton data={url} label="Copy URL" />
		</div>
	{/if}
</div>
