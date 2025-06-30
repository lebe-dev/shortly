# Svelte 5 component example

```typescript
<script lang="ts">
	import * as HoverCard from '$lib/components/ui/hover-card/index.js';

	interface Props {
		propertyPath: string;
	}

	let { propertyPath }: Props = $props();
</script>

<HoverCard.Root>
	<HoverCard.Trigger>
		<span class="text-muted-foreground"> - Disabled in configuration</span>
	</HoverCard.Trigger>
	<HoverCard.Content>
		<p class="text-sm">
			Property in config file <span class="text-primary">{propertyPath}</span>
		</p>
	</HoverCard.Content>
</HoverCard.Root>
```
