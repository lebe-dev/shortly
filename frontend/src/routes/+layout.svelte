<script lang="ts">
	import '../app.css';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<Toaster position="top-right" />

<ModeWatcher defaultMode="light" />

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
		{@render children()}
	</div>

	<!-- Footer -->
	<div class="flex items-center justify-center bg-gray-50 p-2 text-center text-xs text-gray-500">
		<span class="">v0.1.0</span> <span class="ms-1.5 me-1.5">|</span>
		<span class="">FAQ</span>
		<span class="ms-1.5 me-1.5">|</span>
		<span class="">GITHUB</span>
	</div>
</div>
