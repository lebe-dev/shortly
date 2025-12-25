import { format } from 'date-fns';

/**
 * Formats hours into a human-readable duration string
 * @param hours - Number of hours
 * @param t - Translation function from svelte-intl-precompile
 * @returns Formatted string like "7 days", "2 weeks", "1 month" in current locale
 */
export function formatDuration(hours: number, t: (key: string, options?: any) => string): string {
	if (hours === 0) {
		return '';
	}

	const days = hours / 24;
	const weeks = days / 7;
	const months = days / 30;

	// If it's exactly divisible by 30 days (month)
	if (days >= 30 && days % 30 === 0) {
		const monthCount = Math.floor(months);
		return t('common.timeUnits.month', { values: { count: monthCount } });
	}

	// If it's exactly divisible by 7 days (week)
	if (days >= 7 && days % 7 === 0) {
		const weekCount = Math.floor(weeks);
		return t('common.timeUnits.week', { values: { count: weekCount } });
	}

	// If it's at least 1 day
	if (days >= 1) {
		const dayCount = Math.floor(days);
		return t('common.timeUnits.day', { values: { count: dayCount } });
	}

	// Less than a day - show hours
	return t('common.timeUnits.hour', { values: { count: hours } });
}

/**
 * Calculates and formats the remaining time for a URL
 * @param ttlSeconds - TTL in seconds
 * @param createdSeconds - Creation timestamp in seconds
 * @param t - Translation function from svelte-intl-precompile
 * @param formatType - 'short' for compact format (6d 23h 53m), 'long' for full words
 * @returns Formatted string like "5 days", "2 hours", or "expired" if time has passed
 */
export function formatRemainingTime(
	ttlSeconds: number,
	createdSeconds: number,
	t: (key: string, options?: any) => string,
	formatType: 'short' | 'long' = 'long'
): string {
	const nowSeconds = Math.floor(Date.now() / 1000);
	const expirationSeconds = createdSeconds + ttlSeconds;
	const remainingSeconds = expirationSeconds - nowSeconds;

	if (remainingSeconds <= 0) {
		return t('linksPage.expired');
	}

	if (formatType === 'short') {
		// For "6d 23h 53m" format
		const days = Math.floor(remainingSeconds / 86400);
		const hours = Math.floor((remainingSeconds % 86400) / 3600);
		const minutes = Math.floor((remainingSeconds % 3600) / 60);

		const parts = [];
		if (days > 0) parts.push(t('linksPage.timeRemaining.dayShort', { values: { count: days } }));
		if (hours > 0) parts.push(t('linksPage.timeRemaining.hourShort', { values: { count: hours } }));
		if (minutes > 0 || parts.length === 0)
			parts.push(t('linksPage.timeRemaining.minuteShort', { values: { count: minutes } }));

		return parts.join(' ');
	}

	// For long format
	const remainingHours = remainingSeconds / 3600;

	// If less than 1 hour, show minutes
	if (remainingHours < 1) {
		const minutes = Math.floor(remainingSeconds / 60);
		if (minutes <= 0) {
			return t('common.timeUnits.lessThanMinute');
		}
		return t('common.timeUnits.minute', { values: { count: minutes } });
	}

	// If less than 24 hours, show hours
	if (remainingHours < 24) {
		const hours = Math.floor(remainingHours);
		return t('common.timeUnits.hour', { values: { count: hours } });
	}

	// Otherwise use the same logic as formatDuration
	return formatDuration(Math.floor(remainingHours), t);
}

/**
 * Formats a timestamp in seconds to DD.MM.YYYY HH:mm:ss format
 * @param timestampSeconds - Unix timestamp in seconds
 * @returns Formatted date string in DD.MM.YYYY HH:mm:ss format
 */
export function formatCreatedDate(timestampSeconds: number): string {
	const date = new Date(timestampSeconds * 1000);
	return format(date, 'dd.MM.yyyy HH:mm:ss');
}

/**
 * Formats the expiry date for a URL in DD.MM.YYYY HH:mm:ss format
 * @param createdSeconds - Creation timestamp in seconds
 * @param ttlSeconds - TTL in seconds
 * @returns Formatted expiry date string in DD.MM.YYYY HH:mm:ss format
 */
export function formatExpiryDate(createdSeconds: number, ttlSeconds: number): string {
	const expiresAt = (createdSeconds + ttlSeconds) * 1000;
	return format(new Date(expiresAt), 'dd.MM.yyyy HH:mm:ss');
}
