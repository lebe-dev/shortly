<script lang="ts">
	import '../app.css';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher } from 'mode-watcher';
	import { t, waitLocale } from 'svelte-intl-precompile';
	import LightSwitch from '$lib/components/LightSwitch.svelte';
	import UserMenu from '$lib/components/UserMenu.svelte';
	import MobileMenu from '$lib/components/MobileMenu.svelte';
	import LanguageSelector from '$lib/components/LanguageSelector.svelte';
	import { fetchConfig } from '$lib/api/config';
	import { checkSession } from '$lib/api/auth';
	import type { AppConfig } from '$lib/domain/config';
	import { onMount } from 'svelte';
	import { authStore, authLoading } from '$lib/stores/auth';

	let { children } = $props();

	let showFullAppName: boolean = $state(false);
	let config: AppConfig | null = $state(null);

	onMount(async () => {
		await fetchConfig()
			.then((data) => {
				config = data;
			})
			.catch((e) => {
				console.error('Failed to load config in layout:', e);
			});

		if (config?.auth.enabled) {
			try {
				const session = await checkSession();
				authStore.set(session);
			} catch (e) {
				console.error('Failed to check session:', e);
			} finally {
				authLoading.set(false);
			}
		} else {
			authLoading.set(false);
		}
	});
</script>

<Toaster position="top-right" />

<ModeWatcher defaultMode="light" />

{#await waitLocale() then _}
	<div class="flex min-h-screen flex-col">
		<!-- Header -->
		<div class="flex justify-center bg-black dark:bg-black">
			<div class="flex w-full max-w-[1300px] items-center justify-between gap-1 lg:gap-5">
				<a href="/" data-sveltekit-reload title={$t('layout.homeLink')}>
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

				<!-- Desktop controls (hidden on mobile) -->
				<div class="me-1 hidden items-center gap-2 text-right md:flex">
					{#if !$authStore.authenticated}
						<LanguageSelector />
					{/if}
					<LightSwitch />
					{#if config?.auth.enabled}
						<UserMenu {config} />
					{/if}
				</div>

				<!-- Mobile menu (hidden on desktop) -->
				<div class="me-1 md:hidden">
					<MobileMenu {config} />
				</div>
			</div>
		</div>

		<!-- Main Content -->
		<div class="flex flex-1 items-start justify-center">
			{@render children()}
		</div>

		<!-- Footer -->
		<div
			class="flex items-center justify-center bg-gray-50 p-2 text-center text-xs text-gray-500 dark:bg-gray-900 dark:text-gray-400"
		>
			<span>v1.2.0</span>
		</div>
	</div>
{/await}
