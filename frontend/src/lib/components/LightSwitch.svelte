<script lang="ts">
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index.js';
</script>

<Button
	onclick={() => toggleMode()}
	variant="outline"
	size="icon"
	class="border-0 bg-transparent align-middle hover:cursor-pointer hover:bg-blue-700 dark:bg-transparent"
	title="Toggle theme"
>
	<Sun
		class="text-secondary-foreground hover:text-primary-foreground h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0  dark:hover:text-amber-400"
	/>
	<Moon
		class="text-secondary absolute h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all hover:cursor-pointer hover:text-gray-950 dark:scale-0 dark:-rotate-90 dark:hover:bg-green-500"
	/>

	<span class="sr-only">Toggle theme</span>
</Button>
