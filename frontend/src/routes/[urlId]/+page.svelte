<script lang="ts">
	import { fetchUrlById } from '$lib/api/url';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { onMount } from 'svelte';

	let { data } = $props();

	let inProgress = $state(true);
	let url = $state('');

	onMount(async () => {
		console.log('url-id', data.urlId);

		await fetchUrlById(data.urlId).then((response) => {
			console.log('url: ', response.url);
			url = response.url;
			inProgress = false;
		});
	});
</script>

{#if inProgress}
	<div>Loading...</div>
{:else}
	<div class="justify-start text-left">
		<div>Full URL:</div>
		<div class="mb-4 text-3xl">{url}</div>

		<CopyButton data={url} label="Copy URL" />
	</div>
{/if}
