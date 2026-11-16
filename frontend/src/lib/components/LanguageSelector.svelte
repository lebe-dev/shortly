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
		class="text-muted-foreground hover:bg-accent hover:text-foreground flex h-11 items-center gap-1.5 rounded-[10px] px-3 font-mono text-xs font-medium uppercase transition-colors hover:cursor-pointer"
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
			class="border-border bg-popover absolute top-full right-0 z-50 mt-1 min-w-[72px] rounded-xl border p-1 shadow-lg"
		>
			<ul>
				{#each languages as lang (lang)}
					<li>
						<button
							onclick={() => changeLanguage(lang)}
							class="hover:bg-accent flex w-full items-center justify-center rounded-lg px-3 py-1.5 font-mono text-xs font-medium uppercase transition-colors hover:cursor-pointer {$locale ===
							lang
								? 'text-link bg-accent'
								: 'text-foreground/80'}"
						>
							{lang}
						</button>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</div>
