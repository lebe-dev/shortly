<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { checkCustomName } from '$lib/api/url';
	import { t } from 'svelte-intl-precompile';
	import type { NamedUrlsConfig } from '$lib/domain/config';

	interface Props {
		value: string;
		config: NamedUrlsConfig;
		disabled?: boolean;
		hideStatus?: boolean;
		onValueChange?: (value: string) => void;
		onAvailabilityChange?: (available: boolean | null) => void;
		onErrorChange?: (error: string | null) => void;
		onKeydown?: (event: KeyboardEvent) => void;
		onFocus?: () => void;
		onBlur?: () => void;
		inputRef?: HTMLInputElement | null;
	}

	let {
		value = $bindable(''),
		config,
		disabled = false,
		hideStatus = false,
		onValueChange,
		onAvailabilityChange,
		onErrorChange,
		onKeydown,
		onFocus,
		onBlur,
		inputRef = $bindable(null)
	}: Props = $props();

	let checking = $state(false);
	let available = $state<boolean | null>(null);
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	let internalRef = $state<HTMLInputElement | null>(null);

	$effect(() => {
		if (internalRef) {
			inputRef = internalRef;
		}
	});

	function filterInput(input: string): string {
		return input.replace(/[^a-zA-Z0-9\-_]/g, '');
	}

	function handleKeydown(event: KeyboardEvent) {
		if (onKeydown) {
			onKeydown(event);
		}
	}

	function handleFocus() {
		if (onFocus) onFocus();
	}

	function handleBlur() {
		if (onBlur) onBlur();
	}

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		const filteredValue = filterInput(target.value);

		value = filteredValue;
		target.value = filteredValue;

		if (onValueChange) {
			onValueChange(filteredValue);
		}

		available = null;
		if (onAvailabilityChange) {
			onAvailabilityChange(null);
		}

		if (onErrorChange) {
			onErrorChange(null);
		}

		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		if (filteredValue.length === 0) {
			if (onErrorChange) {
				onErrorChange(null);
			}
			return;
		}

		if (filteredValue.length < config.minLength) {
			available = false;
			const error = $t('homePage.customName.tooShort', {
				values: { minLength: config.minLength }
			});
			if (onAvailabilityChange) {
				onAvailabilityChange(false);
			}
			if (onErrorChange) {
				onErrorChange(error);
			}
			return;
		}

		const isReserved = config.reservedNames.some(
			(reserved) => reserved.toLowerCase() === filteredValue.toLowerCase()
		);

		if (isReserved) {
			available = false;
			const error = $t('homePage.customName.reserved');
			if (onAvailabilityChange) {
				onAvailabilityChange(false);
			}
			if (onErrorChange) {
				onErrorChange(error);
			}
			return;
		}

		debounceTimer = setTimeout(async () => {
			checking = true;
			if (onErrorChange) {
				onErrorChange(null);
			}
			try {
				const isAvailable = await checkCustomName(filteredValue);
				available = isAvailable;
				if (onAvailabilityChange) {
					onAvailabilityChange(isAvailable);
				}
				if (onErrorChange) {
					const error = isAvailable ? null : $t('homePage.customName.taken');
					onErrorChange(error);
				}
			} catch (e) {
				console.error('Failed to check name availability:', e);
				available = null;
				if (onAvailabilityChange) {
					onAvailabilityChange(null);
				}
				if (onErrorChange) {
					onErrorChange(null);
				}
			} finally {
				checking = false;
			}
		}, 500);
	}

	const statusMessage = $derived.by(() => {
		if (value.length === 0) {
			return '';
		}

		if (value.length < config.minLength) {
			return $t('homePage.customName.tooShort', {
				values: { minLength: config.minLength }
			});
		}

		if (value.length > config.maxLength) {
			return $t('homePage.customName.tooLong', {
				values: { maxLength: config.maxLength }
			});
		}

		const isReserved = config.reservedNames.some(
			(reserved) => reserved.toLowerCase() === value.toLowerCase()
		);
		if (isReserved) {
			return $t('homePage.customName.reserved');
		}

		if (checking) {
			return $t('homePage.customName.checking');
		}

		if (available === true) {
			return $t('homePage.customName.available');
		}

		if (available === false) {
			return $t('homePage.customName.taken');
		}

		return '';
	});

	const statusClass = $derived.by(() => {
		if (available === true) return 'text-green-600';
		if (available === false) return 'text-red-600';
		if (checking) return 'text-gray-500';
		return 'text-gray-500';
	});
</script>

<div class="w-full text-left">
	<Input
		bind:ref={internalRef}
		type="text"
		bind:value
		oninput={handleInput}
		onkeydown={handleKeydown}
		onfocus={handleFocus}
		onblur={handleBlur}
		{disabled}
		placeholder={$t('homePage.customName.placeholder')}
		maxlength={config.maxLength}
		class="max-w-xs text-base md:text-sm"
	/>
	{#if !hideStatus}
		{#if statusMessage}
			<div class="mt-1 text-sm {statusClass}">
				{statusMessage}
			</div>
		{:else}
			&nbsp;
		{/if}
	{/if}
</div>
