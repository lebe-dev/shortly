<script lang="ts">
	import { resolve } from '$app/paths';
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { format } from 'date-fns';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import ArrowLeft from 'lucide-svelte/icons/arrow-left';
	import KeyRound from 'lucide-svelte/icons/key-round';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import UserRound from 'lucide-svelte/icons/user-round';
	import {
		deletePasskey,
		getPasskeys,
		isPasskeySupported,
		PasskeyCancelledError,
		PasskeyError,
		registerPasskey,
		type PasskeyCredentialDto
	} from '$lib/api/passkey';

	let passkeys: PasskeyCredentialDto[] = $state([]);
	let loading = $state(true);
	let registering = $state(false);
	let newPasskeyName = $state('');
	let hasCheckedAuth = $state(false);

	const passkeyEnabled = $derived($configStore?.auth?.passkey?.enabled === true);
	const passkeySupported = $derived(isPasskeySupported());

	$effect(() => {
		if ($configStore && !hasCheckedAuth) {
			hasCheckedAuth = true;

			if (!$authStore.authenticated) {
				window.location.href = '/login';
				return;
			}

			loadPasskeys();
		}
	});

	async function loadPasskeys() {
		if (!$configStore?.auth?.passkey?.enabled) {
			loading = false;
			return;
		}

		loading = true;

		try {
			passkeys = await getPasskeys();
		} catch (e) {
			console.error('Failed to load passkeys:', e);
			toast.error($t('profilePage.errors.loadFailed'));
		} finally {
			loading = false;
		}
	}

	async function handleRegister() {
		registering = true;

		try {
			await registerPasskey(newPasskeyName);
			newPasskeyName = '';
			toast.success($t('profilePage.passkeys.registered'));
			await loadPasskeys();
		} catch (e) {
			if (e instanceof PasskeyCancelledError) {
				return;
			}

			console.error('Failed to register passkey:', e);

			if (e instanceof PasskeyError && e.code === 'passkey_already_registered') {
				toast.error($t('profilePage.errors.alreadyRegistered'));
				return;
			}

			toast.error($t('profilePage.errors.registerFailed'));
		} finally {
			registering = false;
		}
	}

	async function handleDelete(credential: PasskeyCredentialDto) {
		const confirmed = confirm($t('profilePage.passkeys.deleteConfirm'));
		if (!confirmed) return;

		try {
			await deletePasskey(credential.id);
			toast.success($t('profilePage.passkeys.deleted'));
			await loadPasskeys();
		} catch (e) {
			console.error('Failed to delete passkey:', e);
			toast.error($t('profilePage.errors.deleteFailed'));
		}
	}

	function formatTimestamp(timestamp: number): string {
		return format(new Date(timestamp * 1000), 'dd.MM.yyyy HH:mm');
	}
</script>

<svelte:head>
	<title>{$t('profilePage.title')}</title>
	<meta name="description" content={$t('profilePage.description')} />
</svelte:head>

<div
	class="surface-card animate-surface-in w-full max-w-[1300px] bg-white px-3 pt-8 pb-18 md:px-14 dark:bg-gray-900"
>
	<a
		href={resolve('/')}
		class="text-muted-foreground/60 hover:text-muted-foreground mb-2 inline-flex items-center gap-1 text-xs hover:underline"
	>
		<ArrowLeft class="h-3 w-3" />
		{$t('common.backToHome')}
	</a>

	<div class="mb-6 flex items-center gap-2">
		<UserRound class="h-5 w-5" />
		<h1 class="text-xl font-bold">{$t('profilePage.header')}</h1>
	</div>

	{#if $authStore.user}
		<div class="text-muted-foreground mb-8 text-sm">
			{$authStore.user.username}
		</div>
	{/if}

	<div class="mb-4 flex items-center gap-2">
		<KeyRound class="h-4 w-4" />
		<h2 class="text-lg font-semibold">{$t('profilePage.passkeys.header')}</h2>
	</div>

	{#if !passkeyEnabled}
		<p class="text-muted-foreground text-sm">{$t('profilePage.passkeys.disabled')}</p>
	{:else if !passkeySupported}
		<p class="text-muted-foreground text-sm">{$t('profilePage.passkeys.unsupported')}</p>
	{:else}
		<p class="text-muted-foreground mb-4 max-w-2xl text-sm">
			{$t('profilePage.passkeys.hint')}
		</p>

		<div class="mb-6 flex flex-wrap items-center gap-2">
			<Input
				class="max-w-xs"
				bind:value={newPasskeyName}
				placeholder={$t('profilePage.passkeys.namePlaceholder')}
				maxlength={64}
			/>
			<Button disabled={registering} onclick={handleRegister}>
				<KeyRound class="me-2 h-4 w-4" />
				{registering ? $t('profilePage.passkeys.registering') : $t('profilePage.passkeys.register')}
			</Button>
		</div>

		{#if loading}
			<p class="text-muted-foreground text-sm">{$t('common.loading')}</p>
		{:else if passkeys.length === 0}
			<p class="text-muted-foreground text-sm">{$t('profilePage.passkeys.empty')}</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="w-full text-sm">
					<thead>
						<tr class="text-muted-foreground border-b text-left">
							<th class="py-2 pe-4 font-medium">{$t('profilePage.passkeys.name')}</th>
							<th class="py-2 pe-4 font-medium">{$t('profilePage.passkeys.created')}</th>
							<th class="py-2 pe-4 font-medium">{$t('profilePage.passkeys.lastUsed')}</th>
							<th class="py-2 font-medium"></th>
						</tr>
					</thead>
					<tbody>
						{#each passkeys as passkey (passkey.id)}
							<tr class="border-b last:border-0">
								<td class="py-2 pe-4">{passkey.name}</td>
								<td class="py-2 pe-4">{formatTimestamp(passkey.createdAt)}</td>
								<td class="py-2 pe-4">
									{passkey.lastUsedAt
										? formatTimestamp(passkey.lastUsedAt)
										: $t('profilePage.passkeys.neverUsed')}
								</td>
								<td class="py-2 text-right">
									<Button
										variant="ghost"
										size="icon"
										title={$t('profilePage.passkeys.delete')}
										onclick={() => handleDelete(passkey)}
									>
										<Trash2 class="h-4 w-4" />
									</Button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}
</div>
