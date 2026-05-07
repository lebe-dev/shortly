<script lang="ts">
	import { generateShortUrl } from '$lib/api/url';
	import { fetchConfig } from '$lib/api/config';
	import ConsumptionBadge from '$lib/components/ConsumptionBadge.svelte';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import CustomNameInput from '$lib/components/CustomNameInput.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Kbd } from '$lib/components/ui/kbd';
	import { formatDuration, formatExpiryDate } from '$lib/date';
	import { isUrlValid } from '$lib/validator';
	import { onMount, tick } from 'svelte';
	import { slide } from 'svelte/transition';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-intl-precompile';
	import { authStore } from '$lib/stores/auth';
	import { configStore } from '$lib/stores/config';
	import { copy } from 'svelte-copy';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';

	let inProgress = $state(true);

	let urlInputRef = $state<HTMLInputElement | null>(null);

	let url: string = $state('');
	let shortUrl: string = $state('');
	let shortUrlTtl = $state(0);
	let maxUrlLength = $state(2048);
	let shortUrlExpiryDate: string = $state('');

	let customName: string = $state('');
	let customNameAvailable = $state<boolean | null>(null);
	let customNameError = $state<string | null>(null);
	let showAvailableMessage = $state(false);
	let availableMessageTimer: ReturnType<typeof setTimeout> | null = null;

	let showUrlValidationError = $state(false);
	let urlValidationTimer: ReturnType<typeof setTimeout> | null = null;

	let urlInputFocused = $state(false);
	let customNameInputFocused = $state(false);
	let customNameInputRef = $state<HTMLInputElement | null>(null);

	const ttlFormatted = $derived(formatDuration(shortUrlTtl, $t));

	const showCustomNameInput = $derived.by(() => {
		if (!$configStore) return false;
		return $configStore.features.namedUrls.enabled && $authStore.authenticated;
	});

	const userAtLimit = $derived.by(() => {
		if (!$configStore || !$authStore.authenticated) return false;

		const cfg = $configStore.features.createUrl;
		const totalLimit = cfg.currentUrls !== undefined && cfg.currentUrls >= cfg.maxPerUser;
		const dailyLimit = cfg.currentUrlsToday !== undefined && cfg.currentUrlsToday >= cfg.maxPerDay;

		return totalLimit || dailyLimit;
	});

	const canGenerate = $derived.by(() => {
		if (url.trim().length === 0) {
			return false;
		}

		if (validationError !== null) {
			return false;
		}

		if (customName.length > 0 && customNameAvailable !== true) {
			return false;
		}

		if (userAtLimit) {
			return false;
		}

		return true;
	});

	const validationError = $derived.by(() => {
		const trimmedUrl = url.trim();

		if (trimmedUrl.length === 0) {
			return null;
		}

		if (trimmedUrl.length >= maxUrlLength) {
			return $t('homePage.errors.urlTooLong', { values: { maxLength: maxUrlLength } });
		}

		if (!isUrlValid(trimmedUrl, maxUrlLength, $configStore?.baseUrl)) {
			return $t('homePage.errors.invalidUrl');
		}

		return null;
	});

	const showUrlHint = $derived.by(() => {
		return urlInputFocused && canGenerate;
	});

	const showCustomNameHint = $derived.by(() => {
		if (!customNameInputFocused) return false;
		if (customName.length === 0) return false;
		return canGenerate;
	});

	function handleUrlKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && canGenerate) {
			event.preventDefault();
			generateUrl();
		}
	}

	function handleCustomNameKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();

			if (url.trim().length === 0) {
				if (urlInputRef) {
					urlInputRef.focus();
				}
			} else if (canGenerate) {
				generateUrl();
			}
		}
	}

	function handleUrlInput() {
		showUrlValidationError = false;

		if (urlValidationTimer) {
			clearTimeout(urlValidationTimer);
			urlValidationTimer = null;
		}

		if (url.trim().length === 0) {
			return;
		}

		urlValidationTimer = setTimeout(() => {
			showUrlValidationError = true;
		}, 500);
	}

	function handleAvailabilityChange(available: boolean | null) {
		customNameAvailable = available;

		if (availableMessageTimer) {
			clearTimeout(availableMessageTimer);
			availableMessageTimer = null;
		}

		if (available === true) {
			showAvailableMessage = true;

			availableMessageTimer = setTimeout(() => {
				showAvailableMessage = false;
			}, 3000);
		} else {
			showAvailableMessage = false;
		}
	}

	$effect(() => {
		if ($configStore) {
			shortUrlTtl = $configStore.shortUrlTtl;
			maxUrlLength = $configStore.maxUrlLength;
			inProgress = false;
		}
	});

	onMount(async () => {
		await tick();
		if (urlInputRef) {
			urlInputRef.focus();
		}
	});

	async function generateUrl() {
		const trimmedUrl = url.trim();

		if (trimmedUrl.length >= maxUrlLength) {
			toast.error($t('homePage.errors.urlTooLong', { values: { maxLength: maxUrlLength } }));
			return;
		}

		if (customName.length > 0 && customNameAvailable !== true) {
			toast.error($t('homePage.errors.customNameNotAvailable'));
			return;
		}

		if (!isUrlValid(trimmedUrl, maxUrlLength, $configStore?.baseUrl)) {
			toast.error($t('homePage.errors.invalidUrl'));
			if (urlInputRef) {
				urlInputRef.focus();
			}
			return;
		}

		inProgress = true;
		try {
			const nameToUse = customName.length > 0 ? customName : undefined;
			const data = await generateShortUrl(trimmedUrl, nameToUse);
			shortUrl = data.url;

			if (!nameToUse && shortUrlTtl > 0) {
				const nowSeconds = Math.floor(Date.now() / 1000);
				const ttlSeconds = shortUrlTtl * 3600; // Convert hours to seconds
				shortUrlExpiryDate = formatExpiryDate(nowSeconds, ttlSeconds);
			}

			if ($authStore.authenticated) {
				await fetchConfig();
			}
		} catch (e: any) {
			console.error(e);
			toast.error(e.message || $t('homePage.errors.generateFailed'));
			if (e.message === 'Rate limit exceeded' && $authStore.authenticated) {
				await fetchConfig();
			}
		} finally {
			inProgress = false;
		}
	}

	let shortUrlPulsing = $state(false);
	let shortUrlPulseTimer: ReturnType<typeof setTimeout> | null = null;

	function handleShortUrlClick() {
		toast.success($t('homePage.result.linkCopied'));

		if (shortUrlPulseTimer) {
			clearTimeout(shortUrlPulseTimer);
		}
		shortUrlPulsing = true;
		shortUrlPulseTimer = setTimeout(() => {
			shortUrlPulsing = false;
		}, 480);
	}
</script>

<svelte:head>
	<title>{$t('homePage.title')}</title>
	<meta name="description" content={$t('homePage.description')} />
</svelte:head>

<div
	class="surface-card animate-surface-in xs:w-[100px] w-[1300px] max-w-[1300px] bg-white px-6 pt-22 pb-18 text-center md:px-24 dark:bg-gray-900"
>
	{#if !$configStore}
		<div class="text-muted-foreground">{$t('common.loading')}</div>
	{:else if !$configStore.features.createUrl.enabled}
		<div class="text-muted-foreground text-lg">
			{$t('homePage.errors.serviceTitle')}
		</div>
	{:else if $configStore.features.createUrl.authOnly && !$authStore.authenticated}
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
				class="text-muted-foreground lucide lucide-user-lock-icon lucide-user-lock"
			>
				<circle cx="10" cy="7" r="4" />
				<path d="M10.3 15H7a4 4 0 0 0-4 4v2" />
				<path d="M15 15.5V14a2 2 0 0 1 4 0v1.5" />
				<rect width="8" height="5" x="13" y="16" rx=".899" />
			</svg>
			<div class="text-muted-foreground text-lg">
				{$t('homePage.errors.authRequired')}
			</div>
		</div>
	{:else if shortUrl === ''}
		<div class="mb-2 text-left text-base font-medium">
			{$t('homePage.form.label')}
		</div>
		<Input
			bind:ref={urlInputRef}
			type="text"
			bind:value={url}
			oninput={handleUrlInput}
			onkeydown={handleUrlKeydown}
			onfocus={() => (urlInputFocused = true)}
			onblur={() => (urlInputFocused = false)}
			disabled={inProgress}
			placeholder={$t('homePage.form.placeholder')}
			maxlength={maxUrlLength}
			class="md:text-md mb-2 w-full text-lg"
			autofocus
		/>
		{#if showUrlHint}
			<div
				transition:slide={{ duration: 200 }}
				class="text-muted-foreground mt-1 mb-2 text-left text-xs"
			>
				{$t('homePage.hints.urlField.prefix')}<Kbd>Enter</Kbd>{$t('homePage.hints.urlField.suffix')}
			</div>
		{/if}
		{#if showCustomNameInput && $configStore}
			<div class="mb-8">
				<div
					class="mt-3 mb-1 text-left text-xs {customName.length === 0
						? 'text-muted-foreground'
						: ''}"
				>
					{$t('homePage.customName.label')}
				</div>
				<CustomNameInput
					bind:value={customName}
					bind:inputRef={customNameInputRef}
					config={$configStore.features.namedUrls}
					disabled={inProgress}
					hideStatus={true}
					onAvailabilityChange={handleAvailabilityChange}
					onErrorChange={(error) => {
						customNameError = error;
					}}
					onKeydown={handleCustomNameKeydown}
					onFocus={() => (customNameInputFocused = true)}
					onBlur={() => (customNameInputFocused = false)}
				/>
				{#if showCustomNameHint}
					<div
						transition:slide={{ duration: 200 }}
						class="text-muted-foreground mt-1 text-left text-xs"
					>
						{$t('homePage.hints.customNameField.prefix')}<Kbd>Enter</Kbd>{$t(
							'homePage.hints.customNameField.suffixCreate'
						)}
					</div>
				{/if}
			</div>
		{/if}
		<div class="mb-3 min-h-[1.25rem] text-sm">
			{#if showUrlValidationError && validationError}
				<div class="text-red-600 dark:text-red-400">
					{validationError}
				</div>
			{:else if customNameError}
				<div class="text-red-600 dark:text-red-400">
					{customNameError}
				</div>
			{:else if userAtLimit}
				<div class="text-red-600 dark:text-red-400">
					{$t('homePage.errors.limitReached')}
				</div>
			{:else if showAvailableMessage}
				<div class="text-green-600 dark:text-green-400">
					{$t('homePage.customName.available')}
				</div>
			{:else if showCustomNameInput && customName.length > 0}
				<div class="text-muted-foreground">
					{$t('homePage.customName.noExpiration')}
				</div>
			{:else if ttlFormatted}
				<div class="text-muted-foreground">
					{$t('homePage.form.storageInfo', { values: { ttl: ttlFormatted } })}
				</div>
			{:else if !ttlFormatted}
				<div class="text-muted-foreground">{$t('common.loadingEllipsis')}</div>
			{/if}
		</div>
		<div class="flex items-center justify-center gap-3">
			<Button
				size="lg"
				disabled={inProgress || !canGenerate}
				onclick={generateUrl}
				class="hover:-translate-y-px hover:shadow-md"
			>
				{#if inProgress}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				{$t('common.buttons.generate')}
			</Button>
		</div>
		{#if $authStore.authenticated && $configStore && $configStore.features.createUrl.currentUrls !== undefined}
			<div class="mt-4"><ConsumptionBadge config={$configStore.features.createUrl} /></div>
		{/if}
	{:else}
		<div class="animate-hero-in">
			<div class="text-muted-foreground mb-3 text-sm tracking-wide uppercase">
				{$t('homePage.result.title')}
			</div>
			<div
				use:copy={shortUrl}
				onclick={handleShortUrlClick}
				class="font-mono-tech mb-2 inline-block cursor-pointer text-3xl tracking-tight transition-colors hover:text-blue-600 md:text-4xl dark:hover:text-blue-400 {shortUrlPulsing
					? 'animate-pulse-once'
					: ''}"
			>
				{shortUrl}
			</div>
			<div class="text-muted-foreground mb-4 text-xs">
				{$t('homePage.result.clickToCopy')}
			</div>
			{#if customName.length > 0}
				<div class="text-muted-foreground mb-4 text-sm">
					{$t('homePage.result.namedUrlInfo')}
				</div>
			{:else if shortUrlExpiryDate}
				<div class="text-muted-foreground mb-4 text-sm">
					{$t('homePage.result.expirationDate', { values: { date: shortUrlExpiryDate } })}
				</div>
			{/if}
			<div class="flex flex-col items-center gap-3">
				<CopyButton data={shortUrl} label={$t('common.buttons.copy')} />
				<button
					onclick={() => {
						url = '';
						shortUrl = '';
						customName = '';
						shortUrlExpiryDate = '';
					}}
					class="text-muted-foreground text-sm hover:cursor-pointer hover:underline"
				>
					{$t('common.backToHome')}
				</button>
			</div>
		</div>
	{/if}
</div>
