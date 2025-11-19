/**
 * Validates if a string is a valid URL with a proper protocol.
 *
 * Accepts URLs with:
 * - Standard protocols: http, https, ftp, gopher, file, ws, wss, mailto, data, tel, etc.
 * - Spaces and special characters (including non-encoded characters like Cyrillic)
 *
 * @param url - The URL string to validate
 * @returns true if the URL is valid, false otherwise
 */
export function isUrlValid(url: string): boolean {
	// Check if URL is not empty
	if (!url || url.trim().length === 0) {
		return false;
	}

	try {
		// Use the built-in URL API to parse and validate the URL
		const parsedUrl = new URL(url);

		// Ensure the URL has a protocol
		// URL API automatically validates the protocol format
		return parsedUrl.protocol.length > 0;
	} catch {
		// URL constructor throws TypeError for invalid URLs
		return false;
	}
}
