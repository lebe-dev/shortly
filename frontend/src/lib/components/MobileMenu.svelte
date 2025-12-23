<script lang="ts">
	import Menu from 'lucide-svelte/icons/menu';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';
	import LogIn from 'lucide-svelte/icons/log-in';
	import LogOut from 'lucide-svelte/icons/log-out';
	import Globe from 'lucide-svelte/icons/globe';
	import { Button } from '$lib/components/ui/button';
	import {
		DropdownMenu,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuSeparator,
		DropdownMenuTrigger,
		DropdownMenuSub,
		DropdownMenuSubTrigger,
		DropdownMenuSubContent,
		DropdownMenuLabel
	} from '$lib/components/ui/dropdown-menu';
	import { locale, t } from 'svelte-intl-precompile';
	import { toggleMode } from 'mode-watcher';
	import { authStore } from '$lib/stores/auth';
	import { logout } from '$lib/api/auth';
	import { toast } from 'svelte-sonner';
	import type { AppConfig } from '$lib/domain/config';

	let { config }: { config: AppConfig | null } = $props();

	const languages = ['en', 'ru', 'de', 'es', 'fr', 'zh', 'jp', 'ge'];

	function changeLanguage(lang: string) {
		locale.set(lang);
	}

	async function handleLogout() {
		try {
			await logout();
			authStore.set({ authenticated: false, user: undefined });
			window.location.href = '/';
		} catch (e) {
			console.error('Logout failed:', e);
			toast.error($t('common.error'));
		}
	}
</script>

<DropdownMenu>
	<DropdownMenuTrigger>
		<Button
			variant="outline"
			size="icon"
			class="border-0 bg-transparent align-middle hover:cursor-pointer hover:bg-blue-700 dark:bg-transparent"
			title="Menu"
		>
			<Menu class="h-[1.2rem] w-[1.2rem] text-gray-300" />
		</Button>
	</DropdownMenuTrigger>
	<DropdownMenuContent align="end" class="w-52">
		<!-- User Profile (if authenticated) -->
		{#if config?.auth.enabled && $authStore.authenticated && $authStore.user}
			<DropdownMenuLabel class="flex items-center gap-2 py-2">
				{#if $authStore.user.avatar_url}
					<img src={$authStore.user.avatar_url} alt="Avatar" class="h-6 w-6 rounded-full" />
				{/if}
				<span class="text-sm font-medium">{$authStore.user.username}</span>
			</DropdownMenuLabel>
			<DropdownMenuSeparator />
		{/if}

		<!-- Language Submenu -->
		<DropdownMenuSub>
			<DropdownMenuSubTrigger>
				<Globe class="h-4 w-4" />
				<span class="text-sm">{$t('layout.language')} ({$locale.toUpperCase()})</span>
			</DropdownMenuSubTrigger>
			<DropdownMenuSubContent>
				{#each languages as lang}
					<DropdownMenuItem
						onclick={() => changeLanguage(lang)}
						class={$locale === lang ? 'bg-blue-50 text-sm dark:bg-blue-900' : 'text-sm'}
					>
						<span class="uppercase">{lang}</span>
					</DropdownMenuItem>
				{/each}
			</DropdownMenuSubContent>
		</DropdownMenuSub>

		<!-- Theme Toggle -->
		<DropdownMenuItem onclick={() => toggleMode()}>
			<Sun class="h-4 w-4 scale-0 dark:scale-100" />
			<Moon class="absolute h-4 w-4 scale-100 dark:scale-0" />
			<span class="text-sm dark:hidden">{$t('layout.darkTheme')}</span>
			<span class="hidden text-sm dark:inline">{$t('layout.lightTheme')}</span>
		</DropdownMenuItem>

		<!-- Auth: Login or Logout -->
		{#if config?.auth.enabled}
			{#if $authStore.authenticated && $authStore.user}
				<DropdownMenuSeparator />
				<DropdownMenuItem onclick={handleLogout} variant="destructive">
					<LogOut class="h-4 w-4" />
					<span class="text-sm">{$t('layout.logout')}</span>
				</DropdownMenuItem>
			{:else}
				<DropdownMenuSeparator />
				<DropdownMenuItem onclick={() => (window.location.href = '/login')}>
					<LogIn class="h-4 w-4" />
					<span class="text-sm">{$t('layout.login')}</span>
				</DropdownMenuItem>
			{/if}
		{/if}
	</DropdownMenuContent>
</DropdownMenu>
