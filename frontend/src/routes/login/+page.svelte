<script lang="ts">
	import { fetchConfig } from '$lib/api/config';
	import { Button } from '$lib/components/ui/button';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-intl-precompile';
	import type { AppConfig } from '$lib/domain/config';

	let inProgress = $state(true);
	let config: AppConfig | null = $state(null);

	onMount(async () => {
		await fetchConfig()
			.then((data) => {
				config = data;
				inProgress = false;
			})
			.catch((e) => {
				console.error(e);
				toast.error($t('loginPage.errors.configLoadFailed'));
				inProgress = false;
			});
	});
</script>

<svelte:head>
	<title>{$t('loginPage.title')}</title>
	<meta name="description" content={$t('loginPage.description')} />
</svelte:head>

<div class="h-80 w-full rounded bg-white px-8 py-22 text-center shadow md:px-24 dark:bg-gray-900">
	{#if !config && inProgress}
		<div class="text-muted-foreground">{$t('common.loading')}</div>
	{:else if !config}
		<div class="flex flex-col items-center justify-center gap-3">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="text-muted-foreground lucide lucide-circle-alert"
			>
				<circle cx="12" cy="12" r="10" />
				<line x1="12" x2="12" y1="8" y2="12" />
				<line x1="12" x2="12.01" y1="16" y2="16" />
			</svg>
			<div class="text-muted-foreground text-lg">
				{$t('loginPage.errors.configLoadFailed')}
			</div>
		</div>
	{:else if config.auth.enabled && config.auth.authType === 'gitlab'}
		<div class="flex flex-col items-center justify-center gap-6">
			<div class="text-2xl font-medium">
				{$t('loginPage.title').replace(' :: SHRTLY', '')}
			</div>
			<div class="flex flex-col items-center gap-3">
				<Button
					size="lg"
					onclick={() => {
						window.location.href = '/api/auth/login';
					}}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="lucide lucide-log-in me-2"
					>
						<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" />
						<polyline points="10 17 15 12 10 7" />
						<line x1="15" x2="3" y1="12" y2="12" />
					</svg>
					{$t('loginPage.buttons.gitlab')}
				</Button>
				<a href="/" class="text-muted-foreground text-sm hover:underline">
					{$t('loginPage.backToHome')}
				</a>
			</div>
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center gap-3">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="text-muted-foreground lucide lucide-info"
			>
				<circle cx="12" cy="12" r="10" />
				<path d="M12 16v-4" />
				<path d="M12 8h.01" />
			</svg>
			<div class="text-muted-foreground text-lg">
				{$t('loginPage.errors.authDisabled')}
			</div>
		</div>
	{/if}
</div>
