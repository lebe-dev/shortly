/**
 * Formats hours into a human-readable duration string
 * @param hours - Number of hours
 * @returns Formatted string like "7 days", "2 weeks", "1 month"
 */
export function formatDuration(hours: number): string {
	if (hours === 0) {
		return '';
	}

	const days = hours / 24;
	const weeks = days / 7;
	const months = days / 30;

	// If it's exactly divisible by 30 days (month)
	if (days >= 30 && days % 30 === 0) {
		const monthCount = Math.floor(months);
		return monthCount === 1 ? '1 month' : `${monthCount} months`;
	}

	// If it's exactly divisible by 7 days (week)
	if (days >= 7 && days % 7 === 0) {
		const weekCount = Math.floor(weeks);
		return weekCount === 1 ? '1 week' : `${weekCount} weeks`;
	}

	// If it's at least 1 day
	if (days >= 1) {
		const dayCount = Math.floor(days);
		return dayCount === 1 ? '1 day' : `${dayCount} days`;
	}

	// Less than a day - show hours
	return hours === 1 ? '1 hour' : `${hours} hours`;
}

/**
 * Calculates and formats the remaining time for a URL
 * @param ttlSeconds - TTL in seconds
 * @param createdSeconds - Creation timestamp in seconds
 * @returns Formatted string like "5 days", "2 hours", or "expired" if time has passed
 */
export function formatRemainingTime(ttlSeconds: number, createdSeconds: number): string {
	const nowSeconds = Math.floor(Date.now() / 1000);
	const expirationSeconds = createdSeconds + ttlSeconds;
	const remainingSeconds = expirationSeconds - nowSeconds;

	if (remainingSeconds <= 0) {
		return 'expired';
	}

	const remainingHours = remainingSeconds / 3600;

	// If less than 1 hour, show minutes
	if (remainingHours < 1) {
		const minutes = Math.floor(remainingSeconds / 60);
		if (minutes <= 0) {
			return 'less than a minute';
		}
		return minutes === 1 ? '1 minute' : `${minutes} minutes`;
	}

	// If less than 24 hours, show hours
	if (remainingHours < 24) {
		const hours = Math.floor(remainingHours);
		return hours === 1 ? '1 hour' : `${hours} hours`;
	}

	// Otherwise use the same logic as formatDuration
	return formatDuration(Math.floor(remainingHours));
}
