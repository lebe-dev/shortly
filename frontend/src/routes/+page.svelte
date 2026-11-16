<script lang="ts">
	import type { UrlAccess } from '$lib/domain/config';
	import { generateShortUrl, NameCheckRateLimitError, RateLimitError } from '$lib/api/url';
	import { fetchConfig } from '$lib/api/config';
	import LimitsMeter from '$lib/components/LimitsMeter.svelte';
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
	import { resolve } from '$app/paths';
	import { splitShortUrl, stripProtocol } from '$lib/display';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import Link2 from 'lucide-svelte/icons/link-2';
	import Globe from 'lucide-svelte/icons/globe';
	import Lock from 'lucide-svelte/icons/lock';
	import Copy from 'lucide-svelte/icons/copy';
	import Check from 'lucide-svelte/icons/check';
	import Plus from 'lucide-svelte/icons/plus';
	import UserLock from 'lucide-svelte/icons/user-lock';

	let inProgress = $state(true);

	let urlInputRef = $state<HTMLInputElement | null>(null);

	let url: string = $state('');
	let shortUrl: string = $state('');
	let shortUrlTtl = $state(0);
	let maxUrlLength = $state(2048);
	let shortUrlExpiryDate: string = $state('');

	let customName: string = $state('');
	let selectedAccess = $state<UrlAccess | null>(null);
	let createdAccess = $state<UrlAccess>('public');
	const effectiveAccess = $derived(
		selectedAccess ?? $configStore?.features.namedUrls.defaultAccess ?? 'public'
	);
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
			const access =
				nameToUse && $configStore?.features.namedUrls.allowAccessOverride
					? effectiveAccess
					: undefined;
			const data = await generateShortUrl(trimmedUrl, nameToUse, access);
			createdAccess = data.access;
			shortUrl = data.url;

			if (!nameToUse && shortUrlTtl > 0) {
				const nowSeconds = Math.floor(Date.now() / 1000);
				const ttlSeconds = shortUrlTtl * 3600; // Convert hours to seconds
				shortUrlExpiryDate = formatExpiryDate(nowSeconds, ttlSeconds);
			}

			if ($authStore.authenticated) {
				await fetchConfig();
			}
		} catch (e) {
			console.error(e);

			if (e instanceof NameCheckRateLimitError) {
				toast.error($t('homePage.customName.rateLimited'));
				return;
			}

			if (e instanceof RateLimitError) {
				toast.error(
					$t(
						e.scope === 'total'
							? 'common.consumption.reachedTotalLimit'
							: 'common.consumption.reachedDailyLimit'
					)
				);
				if ($authStore.authenticated) {
					await fetchConfig();
				}
				return;
			}

			const message = e instanceof Error ? e.message : '';
			toast.error(message || $t('homePage.errors.generateFailed'));
		} finally {
			inProgress = false;
		}
	}

	let shortUrlPulsing = $state(false);
	let shortUrlPulseTimer: ReturnType<typeof setTimeout> | null = null;

	const shortUrlParts = $derived(splitShortUrl(shortUrl));
	const namePrefix = $derived(
		$configStore ? `${stripProtocol($configStore.baseUrl).replace(/\/+$/, '')}/` : ''
	);
	const accessOptions = $derived([
		{ value: 'public' as UrlAccess, label: $t('homePage.customName.accessPublic'), icon: Globe },
		{ value: 'internal' as UrlAccess, label: $t('homePage.customName.accessInternal'), icon: Lock }
	]);
	const showLimits = $derived(
		$authStore.authenticated && $configStore?.features.createUrl.currentUrls !== undefined
	);

	function resetForm() {
		url = '';
		shortUrl = '';
		customName = '';
		selectedAccess = null;
		shortUrlExpiryDate = '';
		customNameAvailable = null;
	}

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

<div class="mx-auto w-full max-w-[808px] px-6 pt-16 pb-24 md:pt-20">
	{#if !$configStore}
		<div class="text-muted-foreground">{$t('common.loading')}</div>
	{:else if !$configStore.features.createUrl.enabled}
		<div class="text-muted-foreground text-lg">
			{$t('homePage.errors.serviceTitle')}
		</div>
	{:else if $configStore.features.createUrl.authOnly && !$authStore.authenticated}
		<div
			class="surface-card animate-surface-in flex flex-col items-center gap-4 px-6 py-16 text-center"
		>
			<UserLock class="text-muted-foreground size-10" strokeWidth={1.5} />
			<div class="text-muted-foreground text-lg">
				{$t('homePage.errors.authRequired')}
			</div>
			<Button href={resolve('/login')} variant="ink">{$t('layout.login')}</Button>
		</div>
	{:else if shortUrl === ''}
		<div class="animate-surface-in">
			<div class="eyebrow">
				<span class="bg-primary size-1.5 rounded-[1px]"></span>
				{$t('homePage.hero.eyebrow')}
			</div>
			<h1 class="mt-3.5 text-[40px] leading-[1.02] font-semibold tracking-[-0.04em] md:text-5xl">
				{$t('homePage.hero.title')}
			</h1>
			{#if ttlFormatted}
				<p class="text-muted-foreground mt-3.5 max-w-[560px] text-[17px] leading-normal">
					{$t('homePage.form.storageInfo', {
						values: { ttl: ttlFormatted }
					})}{#if showCustomNameInput}. {$t('homePage.customName.noExpiration')}{/if}
				</p>
			{/if}

			<form
				class="surface-card mt-9 overflow-hidden"
				onsubmit={(event) => {
					event.preventDefault();
					if (canGenerate && !inProgress) generateUrl();
				}}
			>
				<div class="px-5 pt-6 pb-5 md:px-6">
					<label for="url-input" class="flex items-baseline gap-2.5 text-[13px] font-medium">
						<span class="text-muted-foreground font-mono text-[11px]">01</span>
						{$t('homePage.form.label')}
					</label>
					<div
						class="field border-input bg-card mt-2.5 flex min-h-14 items-center gap-3 rounded-xl border px-4"
					>
						<Link2 class="text-muted-foreground size-[18px] shrink-0" />
						<Input
							id="url-input"
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
							aria-invalid={showUrlValidationError && validationError ? true : undefined}
							class="h-14 rounded-none border-0 bg-transparent px-0 font-mono text-base md:text-base"
							autofocus
						/>
					</div>
					{#if showUrlValidationError && validationError}
						<div class="text-destructive mt-2.5 text-[13px]" role="alert">
							{validationError}
						</div>
					{:else if showUrlHint}
						<div
							transition:slide={{ duration: 200 }}
							class="text-muted-foreground mt-2.5 text-[13px]"
						>
							{$t('homePage.hints.urlField.prefix')}<Kbd>Enter</Kbd>{$t(
								'homePage.hints.urlField.suffix'
							)}
						</div>
					{/if}
				</div>

				{#if showCustomNameInput && $configStore}
					<div class="bg-hairline h-px"></div>
					<div
						class="grid grid-cols-[repeat(auto-fit,minmax(260px,1fr))] gap-x-6 gap-y-5 px-5 pt-5 pb-6 md:px-6"
					>
						<div class="min-w-0">
							<label for="custom-name" class="flex items-baseline gap-2.5 text-[13px] font-medium">
								<span class="text-muted-foreground font-mono text-[11px]">02</span>
								{$t('homePage.customName.label')}
							</label>
							<div
								class="field border-input bg-card mt-2.5 flex min-h-12 items-center overflow-hidden rounded-xl border"
							>
								<span
									class="text-muted-foreground max-w-[55%] truncate ps-3.5 pe-0.5 font-mono text-sm whitespace-nowrap"
									>{namePrefix}</span
								>
								<CustomNameInput
									id="custom-name"
									bind:value={customName}
									bind:inputRef={customNameInputRef}
									config={$configStore.features.namedUrls}
									disabled={inProgress}
									hideStatus={true}
									inputClass="h-12 rounded-none border-0 bg-transparent px-0 font-mono text-sm font-medium md:text-sm"
									onAvailabilityChange={handleAvailabilityChange}
									onErrorChange={(error) => {
										customNameError = error;
									}}
									onKeydown={handleCustomNameKeydown}
									onFocus={() => (customNameInputFocused = true)}
									onBlur={() => (customNameInputFocused = false)}
								/>
								{#if customName.length > 0 && customNameAvailable === true}
									<span
										class="bg-success-soft text-success me-2 flex shrink-0 items-center gap-1 rounded-md px-2 py-1 text-xs font-medium whitespace-nowrap"
									>
										<Check class="size-3" strokeWidth={3} />
										{$t('homePage.customName.available').replace(/^[✓✔]\s*/, '')}
									</span>
								{/if}
							</div>
							{#if showCustomNameHint}
								<div
									transition:slide={{ duration: 200 }}
									class="text-muted-foreground mt-2.5 text-[13px]"
								>
									{$t('homePage.hints.customNameField.prefix')}<Kbd>Enter</Kbd>{$t(
										'homePage.hints.customNameField.suffixCreate'
									)}
								</div>
							{/if}
						</div>

						{#if customName.length > 0 && $configStore.features.namedUrls.allowAccessOverride}
							<fieldset class="m-0 min-w-0 border-0 p-0" transition:slide={{ duration: 200 }}>
								<legend class="flex items-baseline gap-2.5 p-0 text-[13px] font-medium">
									<span class="text-muted-foreground font-mono text-[11px]">03</span>
									{$t('homePage.customName.accessLabel')}
								</legend>
								<div class="bg-secondary mt-2.5 grid grid-cols-2 gap-1 rounded-xl p-1">
									{#each accessOptions as option (option.value)}
										{@const Icon = option.icon}
										<button
											type="button"
											aria-pressed={effectiveAccess === option.value}
											disabled={inProgress}
											onclick={() => (selectedAccess = option.value)}
											class="text-muted-foreground hover:text-foreground aria-pressed:border-border aria-pressed:bg-card aria-pressed:text-foreground flex min-h-10 cursor-pointer items-center justify-center gap-2 rounded-[9px] border border-transparent px-2 text-sm font-medium transition-colors aria-pressed:shadow-[0_1px_2px_rgba(11,13,18,0.06)]"
										>
											<Icon class="size-[15px] shrink-0" />
											<span class="truncate">{option.label}</span>
										</button>
									{/each}
								</div>
							</fieldset>
						{/if}
					</div>
				{/if}

				<div
					class="border-hairline bg-muted flex flex-wrap items-center justify-between gap-4 border-t px-5 py-4 md:px-6"
				>
					<div class="min-h-5 min-w-0 flex-1 text-[13px]" aria-live="polite">
						{#if customNameError}
							<span class="text-destructive">{customNameError}</span>
						{:else if userAtLimit}
							<span class="text-destructive">{$t('homePage.errors.limitReached')}</span>
						{:else if showAvailableMessage}
							<span class="text-success">{$t('homePage.customName.available')}</span>
						{:else if showCustomNameInput && customName.length > 0}
							<span class="text-muted-foreground">{$t('homePage.customName.noExpiration')}</span>
						{/if}
					</div>
					<Button
						type="submit"
						size="lg"
						disabled={inProgress || !canGenerate}
						class="gap-3.5 pe-3.5"
					>
						{#if inProgress}
							<LoaderCircle class="size-4 animate-spin" />
						{/if}
						{$t('common.buttons.generate')}
						<span class="rounded-md bg-white/20 px-1.5 py-0.5 font-mono text-xs" aria-hidden="true"
							>↵</span
						>
					</Button>
				</div>
			</form>

			{#if showLimits && $configStore}
				<LimitsMeter config={$configStore.features.createUrl} class="mt-5 px-1" />
			{/if}
		</div>
	{:else}
		<div class="animate-hero-in">
			<div class="eyebrow text-success">
				<span class="bg-success size-1.5 rounded-full shadow-[0_0_0_4px_var(--success-soft)]"
				></span>
				{$t('homePage.result.eyebrow')}
			</div>
			<h1 class="mt-3.5 text-[40px] leading-[1.02] font-semibold tracking-[-0.04em] md:text-5xl">
				{$t('homePage.result.heading')}
			</h1>

			<section class="surface-card mt-9 overflow-hidden">
				<div class="px-5 pt-7 pb-6 md:px-6">
					<button
						type="button"
						use:copy={shortUrl}
						onclick={handleShortUrlClick}
						aria-label={`${$t('common.buttons.copy')} ${shortUrl}`}
						class="group flex w-full cursor-pointer items-center justify-between gap-4 rounded-[14px] border-[1.5px] border-dashed border-[color-mix(in_oklab,var(--primary)_45%,transparent)] bg-[color-mix(in_oklab,var(--primary)_6%,var(--card))] py-5 ps-6 pe-5 text-left transition-colors hover:bg-[color-mix(in_oklab,var(--primary)_10%,var(--card))] {shortUrlPulsing
							? 'animate-pulse-once'
							: ''}"
					>
						<span
							class="min-w-0 font-mono text-2xl leading-tight tracking-[-0.03em] [overflow-wrap:anywhere] md:text-[34px]"
						>
							<span class="text-muted-foreground">{shortUrlParts.prefix}</span><span
								class="text-foreground font-semibold">{shortUrlParts.slug}</span
							>
						</span>
						<span
							class="border-input bg-card text-link flex size-11 shrink-0 items-center justify-center rounded-[10px] border"
						>
							<Copy class="size-[18px]" />
						</span>
					</button>
					<div class="text-muted-foreground mt-2.5 text-[13px]">
						{$t('homePage.result.clickToCopy')}
					</div>
				</div>

				{#if createdAccess === 'internal'}
					<div
						class="border-warning-border bg-warning-soft text-warning mx-5 mb-6 flex items-start gap-3 rounded-xl border px-4 py-3.5 text-sm md:mx-6"
						role="status"
					>
						<Lock class="mt-0.5 size-[18px] shrink-0" />
						{$t('homePage.result.internalAccess')}
					</div>
				{/if}

				<dl class="border-hairline grid grid-cols-[repeat(auto-fit,minmax(200px,1fr))] border-t">
					<div class="border-hairline min-w-0 border-e px-5 py-4 md:px-6">
						<dt class="text-muted-foreground text-xs">{$t('linksPage.table.originalUrl')}</dt>
						<dd class="mt-1.5 truncate font-mono text-[13px]" title={url}>
							{stripProtocol(url.trim())}
						</dd>
					</div>
					<div class="border-hairline border-e px-5 py-4 md:px-6">
						<dt class="text-muted-foreground text-xs">{$t('homePage.customName.accessLabel')}</dt>
						<dd class="mt-1.5 text-sm font-medium">
							{createdAccess === 'internal'
								? $t('homePage.customName.accessInternal')
								: $t('homePage.customName.accessPublic')}
						</dd>
					</div>
					<div class="px-5 py-4 md:px-6">
						<dt class="text-muted-foreground text-xs">{$t('linksPage.table.expires')}</dt>
						<dd class="mt-1.5 text-sm font-medium">
							{customName.length === 0 && shortUrlExpiryDate
								? shortUrlExpiryDate
								: $t('common.never')}
						</dd>
					</div>
				</dl>

				<div
					class="border-hairline bg-muted flex flex-wrap items-center justify-between gap-3 border-t px-5 py-4 md:px-6"
				>
					<Button variant="ghost" class="h-11 px-2 text-sm" onclick={resetForm}>
						<Plus class="size-4" />
						{$t('homePage.result.shortenAnother')}
					</Button>
					<div class="flex flex-wrap gap-2.5">
						{#if $authStore.authenticated}
							<Button href={resolve('/links')} variant="outline" size="lg" class="font-medium">
								{$t('layout.myLinks')}
							</Button>
						{/if}
						<CopyButton data={shortUrl} label={$t('common.buttons.copy')} />
					</div>
				</div>
			</section>

			{#if showLimits && $configStore}
				<LimitsMeter config={$configStore.features.createUrl} class="mt-5 px-1" />
			{/if}
		</div>
	{/if}
</div>
