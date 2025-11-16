<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
</script>

<div class="text-secondary container flex items-start gap-3 self-center bg-blue-800">
	<div class="bg-blue-700 ps-3 pe-4 pt-3 pb-3 text-sm text-gray-300">SHORTLY</div>
	<div class="flex items-center gap-5 ps-3 pe-3 pt-3 pb-3 text-sm">
		<div>ITEM1</div>
		<div>ITEM2</div>
		<div>ITEM3</div>
	</div>
</div>
<div class="container w-auto content-center bg-white ps-5 pe-5 pt-10 pb-10 text-center">
	<Input
		placeholder="https://my.super-long-url.com/article/12345"
		class="mb-5 inline-block w-1/2 ps-3 pe-3 text-xl"
	/>

	<div>
		<Button size="lg" class="rounded-5 rounded">Generate</Button>
	</div>
</div>
<div class="text-secondary-foreground bg-gray-50 p-2 text-center text-xs">
	<span class="underline">GITHUB</span> | <span class=" underline">FAQ</span> |
	<span class="">v0.1.0</span>
</div>
