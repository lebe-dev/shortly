<script lang="ts">
	import { t } from 'svelte-intl-precompile';
	import { fetchConfig } from '$lib/api/config';
	import { toast } from 'svelte-sonner';
	import UserCard from '$lib/components/UserCard.svelte';
	import type { AppConfig } from '$lib/domain/config';

	let config: AppConfig | null = $state(null);
	let loading = $state(true);

	$effect(() => {
		if (loading) {
			loadConfig();
		}
	});

	async function loadConfig() {
		loading = true;
		try {
			config = await fetchConfig();
		} catch (e) {
			console.error('Failed to load config:', e);
			toast.error($t('adminPage.errors.notAuthorized'));
		} finally {
			loading = false;
		}
	}

	const sortedUsers = $derived.by(() => {
		if (!config?.admin?.users) return [];
		return [...config.admin.users].sort((a, b) => a.username.localeCompare(b.username));
	});

	function handleQuotasUpdated(
		userId: number,
		updatedQuotas: { maxUrlsPerUser: number; maxUrlsPerDay: number }
	) {
		if (config?.admin?.users) {
			const userIndex = config.admin.users.findIndex((u) => u.id === userId);
			if (userIndex !== -1) {
				config.admin.users[userIndex].maxUrlsPerUser = updatedQuotas.maxUrlsPerUser;
				config.admin.users[userIndex].maxUrlsPerDay = updatedQuotas.maxUrlsPerDay;
			}
		}
	}
</script>

<svelte:head>
	<title>{$t('adminPage.title')} - {$t('adminPage.tabs.users')}</title>
</svelte:head>

{#if loading}
	<div class="py-12 text-center">
		<p class="text-muted-foreground">{$t('common.loading')}</p>
	</div>
{:else if !config?.admin}
	<div class="py-12 text-center">
		<p class="text-muted-foreground text-lg">{$t('adminPage.errors.notAuthorized')}</p>
	</div>
{:else if sortedUsers.length === 0}
	<div class="py-12 text-center">
		<p class="text-muted-foreground text-lg">{$t('adminPage.users.empty')}</p>
	</div>
{:else}
	<!-- User count -->
	<div class="mb-4 ps-1 text-xs text-gray-400 dark:text-gray-500">
		{$t('adminPage.items')}: {sortedUsers.length}
	</div>

	<!-- User cards grid -->
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
		{#each sortedUsers as user}
			<UserCard {user} onQuotasUpdated={(quotas) => handleQuotasUpdated(user.id, quotas)} />
		{/each}
	</div>
{/if}
