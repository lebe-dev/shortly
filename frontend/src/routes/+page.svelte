<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
</script>

<div class="border-secondary w-full max-w-3xl rounded border-3 px-24 py-10 text-center">
	<div class="mb-1 text-left">Paste long URL here</div>
	<Input
		type="text"
		placeholder="https://my.super-long-url.com/article/12345"
		class="mb-5 inline-block text-2xl font-medium"
	/>
	<div class="flex items-center justify-center gap-3">
		<Button size="lg" disabled>GENERATE</Button>
	</div>
</div>
