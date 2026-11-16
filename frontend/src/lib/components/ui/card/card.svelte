<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn, type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card"
	class={cn(
		'bg-card text-card-foreground border-border flex flex-col gap-6 rounded-2xl border py-6 shadow-[0_1px_2px_rgba(11,13,18,0.04)]',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
