export interface ShortUrlParts {
	prefix: string;
	slug: string;
}

/**
 * Removes the http(s) scheme so URLs read compactly in the UI.
 */
export function stripProtocol(url: string): string {
	return url.replace(/^https?:\/\//i, '');
}

/**
 * Splits a short URL into a dimmed prefix (host and base path) and the slug.
 */
export function splitShortUrl(url: string): ShortUrlParts {
	const value = stripProtocol(url).replace(/\/+$/, '');
	const index = value.lastIndexOf('/');
	if (index === -1) {
		return { prefix: '', slug: value };
	}
	return { prefix: value.slice(0, index + 1), slug: value.slice(index + 1) };
}

export type LimitLevel = 'calm' | 'near' | 'reached';

/**
 * Classifies limit usage so the UI stays quiet until the user approaches the limit.
 */
export function limitLevel(current: number, max: number, nearRatio = 0.8): LimitLevel {
	if (max <= 0 || current >= max) {
		return 'reached';
	}
	return current / max >= nearRatio ? 'near' : 'calm';
}

/**
 * Returns usage as a percentage clamped to 0..100.
 */
export function limitPercent(current: number, max: number): number {
	if (max <= 0) {
		return 100;
	}
	return Math.min(100, Math.max(0, Math.round((current / max) * 100)));
}
