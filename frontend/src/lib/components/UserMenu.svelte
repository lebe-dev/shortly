<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { logout } from '$lib/api/logout';
	import { Button } from './ui/button';
	import { toast } from 'svelte-sonner';
	import { locale, t } from 'svelte-intl-precompile';
	import type { AppConfig } from '$lib/domain/config';
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
	import Globe from 'lucide-svelte/icons/globe';
	import LogOut from 'lucide-svelte/icons/log-out';
	import LogIn from 'lucide-svelte/icons/log-in';
	import ChevronDown from 'lucide-svelte/icons/chevron-down';
	import Link2 from 'lucide-svelte/icons/link-2';
	import MonitorCog from 'lucide-svelte/icons/monitor-cog';

	interface Props {
		config?: AppConfig | null;
	}

	let { config = null }: Props = $props();

	const languages = ['en', 'ru', 'de', 'es', 'fr', 'zh', 'jp', 'ge', 'he'];

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

{#if $authStore.authenticated && $authStore.user}
	<DropdownMenu>
		<DropdownMenuTrigger>
			<Button
				variant="outline"
				class="dark:hover:bg-secondary/50 flex items-center gap-2 rounded border-0 bg-transparent px-2.5 py-1.5 text-xs font-medium text-gray-300 hover:cursor-pointer hover:bg-blue-700 hover:text-gray-300"
			>
				{#if $authStore.user.avatar_url}
					<img src={$authStore.user.avatar_url} alt="Avatar" class="h-6 w-6 rounded-full" />
				{/if}
				<span class="text-xs">{$authStore.user.username}</span>
				<ChevronDown class="h-3 w-3" />
			</Button>
		</DropdownMenuTrigger>
		<DropdownMenuContent align="end" class="border-secondary w-52 rounded-xs border">
			<!-- User info header -->
			<DropdownMenuLabel class="flex items-center gap-2 py-2 ">
				{#if $authStore.user.avatar_url}
					<img src={$authStore.user.avatar_url} alt="Avatar" class="h-6 w-6 rounded-full" />
				{/if}
				<span class="text-sm font-medium">{$authStore.user.username}</span>
			</DropdownMenuLabel>
			<DropdownMenuSeparator />

			<!-- My Links -->
			<DropdownMenuItem>
				<a href="/links" class="flex w-full items-center gap-2">
					<Link2 class="h-4 w-4" />
					<span class="text-sm">{$t('layout.myLinks')}</span>
				</a>
			</DropdownMenuItem>

			<!-- Admin Panel (only visible for admins) -->
			{#if config?.admin}
				<DropdownMenuItem>
					<a href="/admin" class="flex w-full items-center gap-2">
						<MonitorCog class="h-4 w-4" />
						<span class="text-sm">{$t('layout.adminPanel')}</span>
					</a>
				</DropdownMenuItem>
			{/if}
			<DropdownMenuSeparator />

			<!-- Language Submenu -->
			<DropdownMenuSub>
				<DropdownMenuSubTrigger>
					<Globe class="h-4 w-4" />
					<span class="text-sm hover:cursor-pointer"
						>{$t('layout.language')} ({$locale.toUpperCase()})</span
					>
				</DropdownMenuSubTrigger>
				<DropdownMenuSubContent class="border-secondary border">
					{#each languages as lang}
						<DropdownMenuItem
							onclick={() => changeLanguage(lang)}
							class={$locale === lang
								? 'bg-primary dark:text-primary-foreground text-sm  text-gray-100 hover:cursor-pointer dark:bg-blue-300'
								: 'text-sm hover:cursor-pointer'}
						>
							<span class="uppercase">{lang}</span>
						</DropdownMenuItem>
					{/each}
				</DropdownMenuSubContent>
			</DropdownMenuSub>

			<!-- Logout -->
			<DropdownMenuSeparator />
			<DropdownMenuItem onclick={handleLogout} variant="destructive" class="hover:cursor-pointer">
				<LogOut class="h-4 w-4" />
				<span class="text-sm">{$t('layout.logout')}</span>
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenu>
{:else}
	<a href="/login" title={$t('layout.login')}>
		<Button
			variant="outline"
			size="icon"
			class="dark:hover:bg-secondary/50 flex items-center gap-1.5 rounded border border-transparent bg-transparent px-2.5 py-1.5 text-xs font-medium text-gray-300 uppercase transition-colors hover:cursor-pointer hover:bg-blue-700 hover:text-gray-300"
		>
			<LogIn class="h-4 w-4" />
		</Button>
	</a>
{/if}
