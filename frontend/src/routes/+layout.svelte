<script lang="ts">
	import '../app.css';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher } from 'mode-watcher';
	import LightSwitch from '$lib/components/LightSwitch.svelte';

	let { children } = $props();

	let showFullAppName: boolean = $state(false);
</script>

<Toaster position="top-right" />

<ModeWatcher />

<div class="flex min-h-screen flex-col">
	<!-- Header -->
	<div class="bg-black dark:bg-black">
		<div class="mx-auto flex items-center justify-between gap-5 sm:px-62">
			<a href="/" title="Go to main page">
				<div
					class="dark:text-primary-foreground block w-32 bg-blue-700 ps-3 pe-4 pt-3 pb-3 text-sm font-medium text-gray-300 dark:bg-blue-400"
					onmouseenter={() => (showFullAppName = true)}
					onmouseleave={() => (showFullAppName = false)}
				>
					<div class="flex items-center">
						{#if showFullAppName}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="15"
								height="15"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								class="lucide lucide-tally4-icon lucide-tally-4 me-0.5 inline-block"
								><path d="M4 4v16" /><path d="M9 4v16" /><path d="M14 4v16" /><path
									d="M19 4v16"
								/></svg
							>

							SHORTLY
						{:else}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="15"
								height="15"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								class="lucide lucide-tally3-icon lucide-tally-3 me-0.5 inline-block"
								><path d="M4 4v16" /><path d="M9 4v16" /><path d="M14 4v16" /></svg
							> SHRTLY
						{/if}
					</div>
				</div>
			</a>

			<div
				class="text-secondary flex items-center gap-5 text-left text-xs dark:text-gray-500"
			></div>

			<div class="me-3 text-right">
				<LightSwitch />
			</div>
		</div>
	</div>

	<!-- Main Content -->
	<div class="flex flex-1 items-start justify-center sm:px-62">
		{@render children()}
	</div>

	<!-- Footer -->
	<div
		class="flex items-center justify-center bg-gray-50 p-2 text-center text-xs text-gray-500 dark:bg-gray-900 dark:text-gray-400"
	>
		<span class="">v0.1.0</span> <span class="ms-1.5 me-1.5">|</span>
		<span class="">TINYOPS</span>
	</div>
</div>
