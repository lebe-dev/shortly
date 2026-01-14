<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Tabs, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import MonitorCog from 'lucide-svelte/icons/monitor-cog';

	let { children } = $props();

	let hasCheckedAuth = $state(false);

	const activeTab = $derived.by(() => {
		const path = $page.url.pathname;
		if (path.includes('/admin/general')) return 'general';
		if (path.includes('/admin/users')) return 'users';
		if (path.includes('/admin/audit')) return 'audit';
		if (path.includes('/admin/links')) return 'links';
		return 'links'; // default
	});

	$effect(() => {
		if ($configStore && !hasCheckedAuth) {
			hasCheckedAuth = true;

			if (!$authStore.authenticated) {
				window.location.href = '/login';
			} else if (!$configStore.admin) {
				toast.error($t('adminPage.errors.notAuthorized'));
				setTimeout(() => {
					window.location.href = '/';
				}, 2000);
			}
		}
	});

	function navigateToTab(tab: string) {
		goto(`/admin/${tab}`);
	}
</script>

<div
	class="w-full max-w-[1300px] rounded bg-white px-3 pt-8 pb-18 shadow md:px-14 dark:bg-gray-900"
>
	<div class="mb-6 flex items-center gap-2">
		<MonitorCog class="h-5 w-5" />
		<h1 class="text-xl font-bold">{$t('adminPage.header')}</h1>
	</div>

	{#if !$configStore}
		<div class="py-12 text-center">
			<p class="text-muted-foreground">{$t('common.loading')}</p>
		</div>
	{:else if !$configStore.admin}
		<div class="py-12 text-center">
			<p class="text-muted-foreground text-lg">{$t('adminPage.errors.notAuthorized')}</p>
		</div>
	{:else}
		<Tabs value={activeTab} class="w-full">
			<TabsList class="mb-6">
				<TabsTrigger value="general" onclick={() => navigateToTab('general')}>
					{$t('adminPage.tabs.general')}
				</TabsTrigger>
				<TabsTrigger value="links" onclick={() => navigateToTab('links')}>
					{$t('adminPage.tabs.links')}
				</TabsTrigger>
				<TabsTrigger value="users" onclick={() => navigateToTab('users')}>
					{$t('adminPage.tabs.users')}
				</TabsTrigger>
				<TabsTrigger value="audit" onclick={() => navigateToTab('audit')}>
					{$t('adminPage.tabs.audit')}
				</TabsTrigger>
			</TabsList>

			<!-- Child route content rendered here -->
			{@render children()}
		</Tabs>
	{/if}
</div>
