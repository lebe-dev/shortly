<script lang="ts">
	import type { CreateUrlConfig } from '$lib/domain/config';
	import { t } from 'svelte-intl-precompile';
	import UserHint from '$lib/components/UserHint.svelte';

	interface Props {
		config: CreateUrlConfig;
		variant?: 'default' | 'compact';
		showHint?: boolean;
	}

	let { config, variant = 'default', showHint = false }: Props = $props();

	const currentTotal = $derived(config.currentUrls ?? 0);
	const currentDaily = $derived(config.currentUrlsToday ?? 0);

	const totalPercentage = $derived(Math.round((currentTotal / config.maxPerUser) * 100));

	const dailyPercentage = $derived(Math.round((currentDaily / config.maxPerDay) * 100));

	const totalNearLimit = $derived(totalPercentage >= 70);
	const dailyNearLimit = $derived(dailyPercentage >= 70);

	const totalAtLimit = $derived(currentTotal >= config.maxPerUser);

	const dailyAtLimit = $derived(currentDaily >= config.maxPerDay);
</script>

{#if variant === 'compact'}
	<div class="space-y-1 text-sm">
		<div class="flex items-center justify-between">
			<span class="text-muted-foreground">{$t('common.consumption.total')}:</span>
			<span class={totalNearLimit ? 'text-orange-600 dark:text-orange-400' : ''}>
				{currentTotal} / {config.maxPerUser}
			</span>
		</div>
		<div class="flex items-center justify-between">
			<span class="text-muted-foreground">{$t('common.consumption.today')}:</span>
			<span class={dailyNearLimit ? 'text-orange-600 dark:text-orange-400' : ''}>
				{currentDaily} / {config.maxPerDay}
			</span>
		</div>
	</div>
{:else}
	<div
		class="rounded border border-gray-200 bg-gray-50 p-4 md:max-w-1/3 dark:border-gray-700 dark:bg-gray-800"
	>
		<div class="mb-2 flex items-center gap-2">
			<div class="text-sm font-medium">{$t('common.consumption.title')}</div>
			{#if showHint}
				<UserHint hint="linksPage.hints.limits" size="sm" />
			{/if}
		</div>
		<div class="space-y-3">
			<div>
				<div class="mb-1 flex items-center justify-between text-sm">
					<span class="text-muted-foreground">{$t('common.consumption.totalLinks')}</span>
					<span class={totalAtLimit ? 'font-medium text-red-600 dark:text-red-400' : ''}>
						{currentTotal} / {config.maxPerUser}
					</span>
				</div>
				<div class="h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
					<div
						class="h-full transition-all {totalAtLimit
							? 'bg-red-600 dark:bg-red-500'
							: totalNearLimit
								? 'bg-orange-500 dark:bg-orange-400'
								: 'bg-gray-400 dark:bg-gray-500'}"
						style="width: {Math.min(totalPercentage, 100)}%"
					></div>
				</div>
			</div>
			<div>
				<div class="mb-1 flex items-center justify-between text-sm">
					<span class="text-muted-foreground">{$t('common.consumption.createdToday')}</span>
					<span class={dailyAtLimit ? 'font-medium text-red-600 dark:text-red-400' : ''}>
						{currentDaily} / {config.maxPerDay}
					</span>
				</div>
				<div class="h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
					<div
						class="h-full transition-all {dailyAtLimit
							? 'bg-red-600 dark:bg-red-500'
							: dailyNearLimit
								? 'bg-orange-500 dark:bg-orange-400'
								: 'bg-gray-400 dark:bg-gray-500'}"
						style="width: {Math.min(dailyPercentage, 100)}%"
					></div>
				</div>
			</div>
		</div>
		{#if totalAtLimit || dailyAtLimit}
			<div class="mt-3 text-sm text-red-600 dark:text-red-400">
				{#if totalAtLimit && dailyAtLimit}
					{$t('common.consumption.reachedBothLimits')}
				{:else if totalAtLimit}
					{$t('common.consumption.reachedTotalLimit')}
				{:else if dailyAtLimit}
					{$t('common.consumption.reachedDailyLimit')}
				{/if}
			</div>
		{/if}
	</div>
{/if}
