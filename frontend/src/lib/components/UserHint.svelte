<script lang="ts">
	import CircleHelp from 'lucide-svelte/icons/circle-help';
	import {
		Tooltip,
		TooltipContent,
		TooltipProvider,
		TooltipTrigger
	} from '$lib/components/ui/tooltip';
	import { t } from 'svelte-intl-precompile';
	import { cn } from '$lib/utils';

	interface Props {
		hint: string;
		size?: 'xs' | 'sm' | 'md' | 'lg';
		side?: 'top' | 'bottom' | 'left' | 'right';
		class?: string;
	}

	let { hint, size = 'sm', side = 'top', class: className }: Props = $props();

	const sizeClasses = {
		xs: 'h-3 w-3',
		sm: 'h-3.5 w-3.5',
		md: 'h-4 w-4',
		lg: 'h-5 w-5'
	};

	// Auto-detect localization key vs literal text
	const isLocalizationKey = $derived(hint.includes('.'));
	const tooltipText = $derived(isLocalizationKey ? $t(hint) : hint);
</script>

<TooltipProvider>
	<Tooltip>
		<TooltipTrigger>
			<CircleHelp
				class={cn(
					'text-muted-foreground hover:text-foreground cursor-help transition-colors',
					sizeClasses[size],
					className
				)}
			/>
		</TooltipTrigger>
		<TooltipContent {side} class="max-w-xs whitespace-pre-line">
			{tooltipText}
		</TooltipContent>
	</Tooltip>
</TooltipProvider>
