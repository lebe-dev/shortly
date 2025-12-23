import { getLocaleFromNavigator, init, locale } from 'svelte-intl-precompile';
import { registerAll } from '$locales';

registerAll();

// Supported languages in the app
const SUPPORTED_LOCALES = ['en', 'ru', 'de', 'es', 'fr', 'zh', 'jp', 'ge'];

// Get locale from localStorage, fallback to navigator language, then to 'en'
const LOCALE_STORAGE_KEY = 'app-locale';

function getInitialLocale(): string {
	if (typeof window === 'undefined') {
		return 'en';
	}

	// Check localStorage first
	const savedLocale = localStorage.getItem(LOCALE_STORAGE_KEY);
	if (savedLocale && SUPPORTED_LOCALES.includes(savedLocale)) {
		return savedLocale;
	}

	// Auto-detect from browser
	const browserLocale = getLocaleFromNavigator();
	if (browserLocale) {
		// Direct match
		if (SUPPORTED_LOCALES.includes(browserLocale)) {
			return browserLocale;
		}

		// Special case: Georgian (browser uses 'ka', we use 'ge')
		if (browserLocale.startsWith('ka')) {
			return 'ge';
		}

		// Special case: Japanese (browser uses 'ja', we use 'jp')
		if (browserLocale.startsWith('ja')) {
			return 'jp';
		}

		// Try to match by language code (e.g., 'en-US' -> 'en')
		const languageCode = browserLocale.split('-')[0];
		if (SUPPORTED_LOCALES.includes(languageCode)) {
			return languageCode;
		}
	}

	// Fallback to English
	return 'en';
}

const initialLocale = getInitialLocale();

init({ initialLocale, fallbackLocale: 'en' });

// Save locale to localStorage whenever it changes
if (typeof window !== 'undefined') {
	locale.subscribe((value) => {
		if (value) {
			localStorage.setItem(LOCALE_STORAGE_KEY, value);
		}
	});
}

export const ssr = false;
