<script lang="ts">
	import { resolve } from '$app/paths';
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
	import UserRound from 'lucide-svelte/icons/user-round';
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
				variant="ghost"
				class="h-11 gap-2 px-1.5"
				aria-label={$authStore.user.username}
				title={$authStore.user.username}
			>
				{#if $authStore.user.avatarUrl}
					<img src={$authStore.user.avatarUrl} alt="" class="size-8 rounded-full" />
				{:else}
					<span
						class="bg-foreground text-background flex size-8 items-center justify-center rounded-full text-xs font-semibold uppercase"
						>{$authStore.user.username.slice(0, 2)}</span
					>
				{/if}
				<ChevronDown class="size-3.5" />
			</Button>
		</DropdownMenuTrigger>
		<DropdownMenuContent align="end" class="w-56">
			<!-- User info header -->
			<DropdownMenuLabel class="flex items-center gap-2 py-2 ">
				{#if $authStore.user.avatarUrl}
					<img src={$authStore.user.avatarUrl} alt="Avatar" class="h-6 w-6 rounded-full" />
				{/if}
				<span class="text-sm font-medium">{$authStore.user.username}</span>
			</DropdownMenuLabel>
			<DropdownMenuSeparator />

			<!-- My Links -->
			<DropdownMenuItem>
				<a href={resolve('/links')} class="flex w-full items-center gap-2">
					<Link2 class="h-4 w-4" />
					<span class="text-sm">{$t('layout.myLinks')}</span>
				</a>
			</DropdownMenuItem>

			<!-- Profile -->
			<DropdownMenuItem>
				<a href={resolve('/profile')} class="flex w-full items-center gap-2">
					<UserRound class="h-4 w-4" />
					<span class="text-sm">{$t('layout.profile')}</span>
				</a>
			</DropdownMenuItem>

			<!-- Admin Panel (only visible for admins) -->
			{#if config?.admin}
				<DropdownMenuItem>
					<a href={resolve('/admin')} class="flex w-full items-center gap-2">
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
				<DropdownMenuSubContent>
					{#each languages as lang (lang)}
						<DropdownMenuItem
							onclick={() => changeLanguage(lang)}
							class={$locale === lang
								? 'bg-accent text-link text-sm hover:cursor-pointer'
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
	<a href={resolve('/login')} title={$t('layout.login')}>
		<Button variant="ghost" class="h-11 px-3">
			<LogIn class="size-4" />
			{$t('layout.login')}
		</Button>
	</a>
{/if}
