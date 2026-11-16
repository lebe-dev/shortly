<script lang="ts">
	import type { CreateUrlConfig } from '$lib/domain/config';
	import { limitLevel, limitPercent, type LimitLevel } from '$lib/display';
	import { cn } from '$lib/utils';
	import { t } from 'svelte-intl-precompile';
	import UserHint from './UserHint.svelte';

	interface Props {
		config: CreateUrlConfig;
		class?: string;
	}

	let { config, class: className }: Props = $props();

	const meters = $derived.by(() => {
		const today = config.currentUrlsToday ?? 0;
		const total = config.currentUrls ?? 0;
		return [
			{
				key: 'today',
				label: $t('common.consumption.today'),
				current: today,
				max: config.maxPerDay,
				level: limitLevel(today, config.maxPerDay),
				percent: limitPercent(today, config.maxPerDay)
			},
			{
				key: 'total',
				label: $t('common.consumption.total'),
				current: total,
				max: config.maxPerUser,
				level: limitLevel(total, config.maxPerUser),
				percent: limitPercent(total, config.maxPerUser)
			}
		];
	});

	const dailyReached = $derived(meters[0].level === 'reached');
	const totalReached = $derived(meters[1].level === 'reached');

	const textClass: Record<LimitLevel, string> = {
		calm: 'text-muted-foreground',
		near: 'text-warning',
		reached: 'text-destructive'
	};

	const fillClass: Record<LimitLevel, string> = {
		calm: 'bg-muted-foreground/45',
		near: 'bg-warning',
		reached: 'bg-destructive'
	};
</script>

<div class={cn('flex flex-col gap-2', className)}>
	<div class="text-muted-foreground flex flex-wrap items-center gap-x-5 gap-y-2 text-xs">
		{#each meters as meter (meter.key)}
			<div class="flex items-center gap-2 {textClass[meter.level]}">
				<span>{meter.label}</span>
				<span
					class="bg-meter-empty relative h-[3px] w-12 overflow-hidden rounded-full"
					role="meter"
					aria-label={meter.label}
					aria-valuemin={0}
					aria-valuemax={meter.max}
					aria-valuenow={meter.current}
				>
					<span
						class="absolute inset-y-0 left-0 rounded-full {fillClass[meter.level]}"
						style="width: {meter.percent}%"
					></span>
				</span>
				<span class="font-mono-tech tabular-nums">
					<span class={meter.level === 'calm' ? 'text-foreground/80' : 'font-semibold'}
						>{meter.current}</span
					>/{meter.max}
				</span>
			</div>
		{/each}
		<UserHint hint="common.consumption.hint.limits" size="sm" side="top" />
	</div>
	{#if dailyReached || totalReached}
		<div class="text-destructive text-sm" role="status">
			{#if dailyReached && totalReached}
				{$t('common.consumption.reachedBothLimits')}
			{:else if totalReached}
				{$t('common.consumption.reachedTotalLimit')}
			{:else}
				{$t('common.consumption.reachedDailyLimit')}
			{/if}
		</div>
	{/if}
</div>
