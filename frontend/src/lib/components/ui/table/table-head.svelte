<script lang="ts">
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { HTMLThAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLThAttributes> = $props();
</script>

<th
	bind:this={ref}
	data-slot="table-head"
	class={cn(
		'text-muted-foreground bg-muted h-10 bg-clip-padding px-4 text-start align-middle text-xs font-medium whitespace-nowrap first:ps-5 last:pe-5 [&:has([role=checkbox])]:pe-0',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</th>
