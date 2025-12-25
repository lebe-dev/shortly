<script lang="ts">
	import type { CreateUrlConfig } from '$lib/domain/config';
	import { t } from 'svelte-intl-precompile';
	import UserHint from './UserHint.svelte';

	interface Props {
		config: CreateUrlConfig;
	}

	let { config }: Props = $props();

	const currentTotal = $derived(config.currentUrls ?? 0);
	const currentDaily = $derived(config.currentUrlsToday ?? 0);

	const totalPercentage = $derived(Math.round((currentTotal / config.maxPerUser) * 100));
	const dailyPercentage = $derived(Math.round((currentDaily / config.maxPerDay) * 100));

	const shouldShow = $derived(totalPercentage >= 70 || dailyPercentage >= 70);

	const showDaily = $derived(dailyPercentage >= totalPercentage);
	const percentage = $derived(showDaily ? dailyPercentage : totalPercentage);
	const remaining = $derived(
		showDaily ? config.maxPerDay - currentDaily : config.maxPerUser - currentTotal
	);
	const total = $derived(showDaily ? config.maxPerDay : config.maxPerUser);

	const isNearLimit = $derived(percentage >= 80);
</script>

{#if shouldShow}
	<div class="mt-2 flex items-center justify-center gap-1.5 text-xs">
		<span class={isNearLimit ? 'text-orange-600 dark:text-orange-400' : 'text-muted-foreground/50'}>
			{showDaily
				? $t('common.consumption.badge.remainingToday', { values: { remaining, total } })
				: $t('common.consumption.badge.remainingTotal', { values: { remaining, total } })}
		</span>
		<UserHint hint="common.consumption.hint.limits" size="xs" side="top" />
	</div>
{/if}
