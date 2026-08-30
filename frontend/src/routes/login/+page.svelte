<script lang="ts">
	import { resolve } from '$app/paths';
	import { Button } from '$lib/components/ui/button';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import KeyRound from 'lucide-svelte/icons/key-round';
	import {
		isPasskeySupported,
		loginWithPasskey,
		PasskeyCancelledError,
		PasskeyError
	} from '$lib/api/passkey';

	const config = $derived($configStore);

	let passkeyLoginInProgress = $state(false);

	const passkeySupported = $derived(isPasskeySupported());

	async function handlePasskeyLogin() {
		passkeyLoginInProgress = true;

		try {
			await loginWithPasskey();
			window.location.href = '/';
		} catch (e) {
			if (e instanceof PasskeyCancelledError) {
				return;
			}

			console.error('Passkey login failed:', e);

			if (e instanceof PasskeyError && e.code === 'unknown_account') {
				toast.error($t('loginPage.errors.passkeyUnknownAccount'));
				return;
			}

			toast.error($t('loginPage.errors.passkeyLoginFailed'));
		} finally {
			passkeyLoginInProgress = false;
		}
	}
</script>

<svelte:head>
	<title>{$t('loginPage.title')}</title>
	<meta name="description" content={$t('loginPage.description')} />
</svelte:head>

<div
	class="surface-card animate-surface-in h-80 w-full bg-white px-8 py-22 text-center md:px-24 dark:bg-gray-900"
>
	{#if !config}
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
			<div class="text-2xl font-semibold tracking-tight">
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
				{#if config.auth.passkey?.enabled && passkeySupported}
					<Button
						size="lg"
						variant="outline"
						disabled={passkeyLoginInProgress}
						onclick={handlePasskeyLogin}
					>
						<KeyRound class="me-2 h-5 w-5" />
						{passkeyLoginInProgress
							? $t('loginPage.buttons.passkeyInProgress')
							: $t('loginPage.buttons.passkey')}
					</Button>
				{/if}
				{#if config.auth.note}
					<div class="text-primary mt-2 mb-4 text-xs">
						{config.auth.note}
					</div>
				{/if}
				<a href={resolve('/')} class="text-muted-foreground text-sm hover:underline">
					{$t('common.backToHome')}
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
