<script lang="ts">
	import { copy } from 'svelte-copy';
	import Copy from 'lucide-svelte/icons/copy';
	import Check from 'lucide-svelte/icons/check';
	import { Button } from './ui/button';

	let { data, label } = $props();

	let flag = $state(false);

	function toggle() {
		flag = true;
		setTimeout(() => {
			flag = false;
		}, 1200);
	}
</script>

<div use:copy={data}>
	<Button size="lg" onclick={toggle} aria-live="polite">
		{#if flag}
			<Check class="size-4" strokeWidth={2.5} />
		{:else}
			<Copy class="size-4" />
		{/if}
		{label}
	</Button>
</div>
