<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
</script>

<div class="flex min-h-screen flex-col">
	<!-- Header -->
	<div class="bg-blue-800">
		<div class="mx-auto flex max-w-7xl items-center gap-5 px-62">
			<div class="bg-blue-700 ps-3 pe-4 pt-3 pb-3 text-sm text-gray-300">SHORTLY</div>
			<div class="text-secondary flex items-center gap-5 text-sm">
				<div>ITEM1</div>
				<div>ITEM2</div>
				<div>ITEM3</div>
			</div>
		</div>
	</div>

	<!-- Main Content -->
	<div class="flex flex-1 items-start justify-center bg-white pt-10">
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
	</div>

	<!-- Footer -->
	<div class="flex items-center justify-center bg-gray-50 p-2 text-center text-xs text-gray-500">
		<span class="">v0.1.0</span> <span class="ms-1.5 me-1.5">|</span>
		<span class="">FAQ</span>
		<span class="ms-1.5 me-1.5">|</span>
		<span class="">GITHUB</span>
	</div>
</div>
