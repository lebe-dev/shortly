<script lang="ts">
	import { resolve } from '$app/paths';
	import { Button } from '$lib/components/ui/button';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import KeyRound from 'lucide-svelte/icons/key-round';
	import Info from 'lucide-svelte/icons/info';
	import CircleAlert from 'lucide-svelte/icons/circle-alert';
	import ArrowLeft from 'lucide-svelte/icons/arrow-left';
	import Logo from '$lib/components/Logo.svelte';
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

<div class="flex flex-1 flex-col items-center px-6 pt-16 pb-16 md:pt-28">
	<section class="surface-card animate-surface-in w-full max-w-[420px] overflow-hidden">
		{#if !config}
			<div class="flex flex-col items-center gap-3 px-8 py-14 text-center">
				<CircleAlert class="text-muted-foreground size-10" strokeWidth={1.5} />
				<div class="text-muted-foreground text-lg">{$t('loginPage.errors.configLoadFailed')}</div>
			</div>
		{:else if config.auth.enabled && config.auth.authType === 'gitlab'}
			<div class="px-8 pt-9 pb-7">
				<Logo size={56} />
				<h1 class="mt-6 text-[30px] leading-[1.1] font-semibold tracking-[-0.035em]">
					{$t('loginPage.title').replace(' :: SHRTLY', '')}
				</h1>
				<p class="text-muted-foreground mt-2.5 text-[15px] leading-normal">
					{$t('loginPage.subtitle')}
				</p>

				<div class="mt-7 flex flex-col gap-2.5">
					<Button
						variant="ink"
						size="lg"
						onclick={() => {
							window.location.href = '/api/auth/login';
						}}
					>
						<svg
							aria-hidden="true"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.75"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="size-[18px]"
						>
							<path
								d="m22 13.29-3.33-10a.42.42 0 0 0-.14-.18.38.38 0 0 0-.22-.11.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18l-2.26 6.67H8.32L6.1 3.26a.42.42 0 0 0-.1-.18.38.38 0 0 0-.26-.08.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18L2 13.29a.74.74 0 0 0 .27.83L12 21l9.69-6.88a.71.71 0 0 0 .31-.83Z"
							/>
						</svg>
						{$t('loginPage.buttons.gitlab')}
					</Button>
					{#if config.auth.passkey?.enabled && passkeySupported}
						<Button
							size="lg"
							variant="outline"
							class="font-medium"
							disabled={passkeyLoginInProgress}
							onclick={handlePasskeyLogin}
						>
							<KeyRound class="size-[18px]" />
							{passkeyLoginInProgress
								? $t('loginPage.buttons.passkeyInProgress')
								: $t('loginPage.buttons.passkey')}
						</Button>
					{/if}
				</div>
			</div>
			{#if config.auth.note}
				<div
					class="border-hairline bg-muted text-foreground/80 flex items-start gap-2.5 border-t px-8 py-4 text-[13px] leading-normal"
				>
					<Info class="text-link mt-0.5 size-4 shrink-0" />
					<span>{config.auth.note}</span>
				</div>
			{/if}
		{:else}
			<div class="flex flex-col items-center gap-3 px-8 py-14 text-center">
				<Info class="text-muted-foreground size-10" strokeWidth={1.5} />
				<div class="text-muted-foreground text-lg">{$t('loginPage.errors.authDisabled')}</div>
			</div>
		{/if}
	</section>

	<a
		href={resolve('/')}
		class="text-muted-foreground hover:text-foreground mt-6 inline-flex min-h-11 items-center gap-1.5 text-sm transition-colors"
	>
		<ArrowLeft class="size-3.5" />
		{$t('common.backToHome')}
	</a>
</div>
