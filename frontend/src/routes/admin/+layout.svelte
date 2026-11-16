<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Tabs, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

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

	function navigateToTab(tab: 'general' | 'links' | 'users' | 'audit') {
		const path =
			tab === 'general'
				? resolve('/admin/general')
				: tab === 'links'
					? resolve('/admin/links')
					: tab === 'users'
						? resolve('/admin/users')
						: resolve('/admin/audit');
		goto(path);
	}
</script>

<div class="animate-surface-in flex flex-1 flex-col">
	<div class="border-border bg-card border-b">
		<div class="mx-auto w-full max-w-[1200px] px-6 pt-10">
			<h1 class="page-title">{$t('adminPage.header')}</h1>
			{#if $configStore?.admin}
				<Tabs value={activeTab} class="mt-6 w-full">
					<TabsList class="border-b-0">
						<TabsTrigger value="general" onclick={() => navigateToTab('general')}>
							{$t('adminPage.tabs.general')}
						</TabsTrigger>
						<TabsTrigger value="links" onclick={() => navigateToTab('links')}>
							{$t('adminPage.tabs.links')}
							<span
								class="text-link rounded-[5px] bg-[color-mix(in_oklab,var(--primary)_12%,transparent)] px-1.5 py-px font-mono text-[11px] font-medium"
								>{$configStore.admin.allUrls.length}</span
							>
						</TabsTrigger>
						<TabsTrigger value="users" onclick={() => navigateToTab('users')}>
							{$t('adminPage.tabs.users')}
						</TabsTrigger>
						<TabsTrigger value="audit" onclick={() => navigateToTab('audit')}>
							{$t('adminPage.tabs.audit')}
						</TabsTrigger>
					</TabsList>
				</Tabs>
			{:else}
				<div class="h-6"></div>
			{/if}
		</div>
	</div>

	<div class="mx-auto w-full max-w-[1200px] px-6 pt-7 pb-24">
		{#if !$configStore}
			<div class="py-12 text-center">
				<p class="text-muted-foreground">{$t('common.loading')}</p>
			</div>
		{:else if !$configStore.admin}
			<div class="py-12 text-center">
				<p class="text-muted-foreground text-lg">{$t('adminPage.errors.notAuthorized')}</p>
			</div>
		{:else}
			{@render children()}
		{/if}
	</div>
</div>
