<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';
	import { t } from 'svelte-intl-precompile';

	onMount(() => {
		goto(resolve('/admin/links'), { replaceState: true });
	});
</script>

<div class="py-12 text-center">
	<p class="text-muted-foreground">{$t('common.loading')}</p>
</div>
