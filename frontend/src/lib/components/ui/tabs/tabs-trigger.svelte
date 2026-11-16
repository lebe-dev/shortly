<script lang="ts">
	import { Tabs as TabsPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: TabsPrimitive.TriggerProps = $props();
</script>

<TabsPrimitive.Trigger
	bind:ref
	data-slot="tabs-trigger"
	class={cn(
		"text-muted-foreground hover:text-foreground data-[state=active]:text-foreground data-[state=active]:border-primary focus-visible:ring-ring/30 -mb-px inline-flex items-center justify-center gap-2 border-b-2 border-transparent px-3.5 py-3 text-sm font-medium whitespace-nowrap transition-colors outline-none hover:cursor-pointer focus-visible:ring-4 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:font-semibold [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
		className
	)}
	{...restProps}
/>
