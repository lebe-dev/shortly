/**
 * Validates if a string is a valid URL with a proper protocol.
 *
 * Accepts URLs with:
 * - Standard protocols: http, https, ftp, gopher, file, ws, wss, mailto, data, tel, etc.
 * - Spaces and special characters (including non-encoded characters like Cyrillic)
 *
 * Rejects URLs that:
 * - Start with the service's own baseUrl (to prevent redirect loops)
 *
 * @param url - The URL string to validate
 * @param maxLength - Maximum allowed URL length in characters
 * @param baseUrl - Optional base URL of the service to prevent self-referencing
 * @returns true if the URL is valid, false otherwise
 */
export function isUrlValid(url: string, maxLength: number, baseUrl?: string): boolean {
	// Check if URL is not empty
	if (!url || url.trim().length === 0) {
		return false;
	}

	// Check URL length
	if (url.length > maxLength) {
		console.warn('url length exceeded > ', maxLength);
		return false;
	}

	try {
		// Use the built-in URL API to parse and validate the URL
		const parsedUrl = new URL(url);

		// Ensure the URL has a protocol
		// URL API automatically validates the protocol format
		if (parsedUrl.protocol.length === 0) {
			return false;
		}

		// Validate hostname for http/https protocols
		// For web URLs, require a valid domain with a dot (e.g., example.com) or localhost
		if (parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:') {
			const hostname = parsedUrl.hostname.toLowerCase();

			// Allow localhost and IP addresses
			if (
				hostname === 'localhost' ||
				hostname === '127.0.0.1' ||
				hostname === '[::1]' ||
				/^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$/.test(hostname) || // IPv4
				/^\[[\da-f:]+\]$/i.test(hostname) // IPv6
			) {
				// Valid localhost or IP
			} else {
				// Require at least one dot in the hostname (e.g., example.com)
				// This prevents URLs like https://adasdadas
				if (!hostname.includes('.')) {
					return false;
				}

				// Ensure the hostname has at least one character before and after the dot
				const parts = hostname.split('.');
				for (const part of parts) {
					if (part.length === 0) {
						return false;
					}
				}
			}
		}

		// Prevent self-referencing URLs to avoid redirect loops
		if (baseUrl) {
			const normalizedUrl = url.toLowerCase();
			const normalizedBaseUrl = baseUrl.toLowerCase();

			// Remove trailing slash from baseUrl for comparison
			const baseUrlWithoutTrailingSlash = normalizedBaseUrl.endsWith('/')
				? normalizedBaseUrl.slice(0, -1)
				: normalizedBaseUrl;

			// Check if URL starts with baseUrl and is followed by end of string, '/', '?', or '#'
			// This prevents matching subdomains like "short.ly.evil.com" when baseUrl is "short.ly"
			if (
				normalizedUrl === baseUrlWithoutTrailingSlash ||
				normalizedUrl.startsWith(baseUrlWithoutTrailingSlash + '/') ||
				normalizedUrl.startsWith(baseUrlWithoutTrailingSlash + '?') ||
				normalizedUrl.startsWith(baseUrlWithoutTrailingSlash + '#')
			) {
				return false;
			}
		}

		return true;
	} catch {
		// URL constructor throws TypeError for invalid URLs
		return false;
	}
}
