<script lang="ts">
	import { t } from 'svelte-intl-precompile';
	import { refreshConfig } from '$lib/api/config';
	import { configStore } from '$lib/stores/config';
	import UserCard from '$lib/components/UserCard.svelte';

	const sortedUsers = $derived.by(() => {
		if (!$configStore?.admin?.users) return [];
		return [...$configStore.admin.users].sort((a, b) => a.username.localeCompare(b.username));
	});

	async function handleQuotasUpdated(
		userId: number,
		updatedQuotas: { maxUrlsPerUser: number; maxUrlsPerDay: number }
	) {
		if ($configStore?.admin?.users) {
			const userIndex = $configStore.admin.users.findIndex((u) => u.id === userId);
			if (userIndex !== -1) {
				$configStore.admin.users[userIndex].maxUrlsPerUser = updatedQuotas.maxUrlsPerUser;
				$configStore.admin.users[userIndex].maxUrlsPerDay = updatedQuotas.maxUrlsPerDay;
			}
		}
		// Явное обновление с сервера для консистентности
		await refreshConfig();
	}
</script>

<svelte:head>
	<title>{$t('adminPage.title')} - {$t('adminPage.tabs.users')}</title>
</svelte:head>

{#if !$configStore?.admin}
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
		{#each sortedUsers as user (user.id)}
			<UserCard
				{user}
				onQuotasUpdated={(quotas) => handleQuotasUpdated(user.id, quotas)}
				onPasskeysDeleted={refreshConfig}
			/>
		{/each}
	</div>
{/if}
