<script lang="ts">
	import { locale } from 'svelte-intl-precompile';

	const languages = ['en', 'ru', 'de', 'es', 'fr', 'zh', 'jp', 'ge', 'he'];

	let isOpen = $state(false);

	function changeLanguage(lang: string) {
		locale.set(lang);
		isOpen = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function closeDropdown() {
		isOpen = false;
	}

	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.language-selector')) {
			closeDropdown();
		}
	}

	$effect(() => {
		if (isOpen) {
			document.addEventListener('click', handleClickOutside);
			return () => {
				document.removeEventListener('click', handleClickOutside);
			};
		}
	});
</script>

<div class="language-selector relative">
	<button
		onclick={toggleDropdown}
		class="dark:hover:bg-secondary/50 flex items-center gap-1.5 rounded border border-gray-600 bg-transparent px-2.5 py-1.5 text-xs font-medium text-gray-300 uppercase transition-colors hover:cursor-pointer hover:bg-blue-700"
		aria-label="Select language"
		aria-expanded={isOpen}
	>
		<span>{$locale}</span>
		<svg
			class="h-3 w-3 transition-transform {isOpen ? 'rotate-180' : ''}"
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
		>
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
		</svg>
	</button>

	{#if isOpen}
		<div
			class="absolute top-full right-0 z-50 mt-1 min-w-[60px] rounded border border-gray-600 bg-white shadow-lg dark:bg-gray-800"
		>
			<ul class="py-1">
				{#each languages as lang}
					<li>
						<button
							onclick={() => changeLanguage(lang)}
							class="flex w-full items-center justify-center px-3 py-1.5 text-xs font-medium uppercase transition-colors hover:cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 {$locale ===
							lang
								? 'bg-blue-50 text-blue-700 dark:bg-blue-900 dark:text-blue-300'
								: 'text-gray-700 dark:text-gray-300'}"
						>
							{lang}
						</button>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</div>
