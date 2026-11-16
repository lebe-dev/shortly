<script lang="ts">
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index.js';
	import { t } from 'svelte-intl-precompile';
</script>

<Button
	onclick={() => toggleMode()}
	variant="ghost"
	size="icon"
	class="relative size-11"
	title={$t('layout.toggleTheme')}
>
	<Sun class="size-[18px] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" />
	<Moon
		class="absolute size-[18px] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90"
	/>

	<span class="sr-only">{$t('layout.toggleTheme')}</span>
</Button>
