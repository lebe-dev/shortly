<script lang="ts">
	import { cn } from '$lib/utils';

	interface Props {
		size?: number;
		class?: string;
	}

	let { size = 30, class: className }: Props = $props();
</script>

<!-- Three bars shrinking from long to short: a long link becomes a short one -->
<svg
	aria-hidden="true"
	width={size}
	height={Math.round((size * 40) / 56)}
	viewBox="4 12 56 40"
	class={cn('shrink-0', className)}
>
	<rect x="6" y="14" width="52" height="8" rx="4" fill="var(--logo-long)" />
	<rect x="6" y="28" width="36" height="8" rx="4" fill="var(--logo-mid)" />
	<rect x="6" y="42" width="20" height="8" rx="4" fill="var(--primary)" />
</svg>
