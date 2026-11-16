<script lang="ts">
	import '../app.css';
	import { resolve } from '$app/paths';
	import { page } from '$app/stores';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher } from 'mode-watcher';
	import { t, waitLocale } from 'svelte-intl-precompile';
	import LightSwitch from '$lib/components/LightSwitch.svelte';
	import Logo from '$lib/components/Logo.svelte';
	import UserMenu from '$lib/components/UserMenu.svelte';
	import MobileMenu from '$lib/components/MobileMenu.svelte';
	import LanguageSelector from '$lib/components/LanguageSelector.svelte';
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';

	let { children } = $props();

	const pathname = $derived($page.url.pathname);

	const navItems = $derived.by(() => {
		if (!$authStore.authenticated) return [];
		const items = [{ href: resolve('/links'), label: $t('layout.myLinks'), match: '/links' }];
		if ($configStore?.admin) {
			items.push({ href: resolve('/admin'), label: $t('layout.adminPanel'), match: '/admin' });
		}
		return items;
	});
</script>

<Toaster position="top-right" />

<ModeWatcher defaultMode="light" />

{#await waitLocale() then}
	<div class="flex min-h-screen flex-col">
		<header class="border-border bg-card border-b">
			<div
				class="mx-auto flex min-h-16 w-full max-w-[1200px] items-center justify-between gap-4 px-6"
			>
				<a
					href={resolve('/')}
					data-sveltekit-reload
					title={$t('layout.homeLink')}
					class="text-foreground -mx-1 flex items-center gap-2.5 rounded-lg px-1 py-2 transition-opacity hover:opacity-80"
				>
					<Logo />
					<span class="text-[17px] font-semibold tracking-[-0.03em]">shortly</span>
				</a>

				<!-- Desktop controls (hidden on mobile) -->
				<nav class="hidden items-center gap-0.5 md:flex" aria-label="Main">
					{#each navItems as item (item.match)}
						<a
							href={item.href}
							aria-current={pathname.startsWith(item.match) ? 'page' : undefined}
							class="text-muted-foreground hover:text-foreground aria-[current=page]:bg-secondary aria-[current=page]:text-foreground rounded-lg px-3.5 py-2.5 text-sm font-medium transition-colors"
						>
							{item.label}
						</a>
					{/each}
					{#if navItems.length > 0}
						<span aria-hidden="true" class="bg-border mx-2 h-5 w-px"></span>
					{/if}
					{#if !$authStore.authenticated}
						<LanguageSelector />
					{/if}
					<LightSwitch />
					{#if $configStore?.auth.enabled}
						<UserMenu config={$configStore} />
					{/if}
				</nav>

				<!-- Mobile menu (hidden on desktop) -->
				<div class="md:hidden">
					<MobileMenu config={$configStore} />
				</div>
			</div>
		</header>

		<main class="flex flex-1 flex-col">
			{@render children()}
		</main>
	</div>
{/await}
