import { getLocaleFromNavigator, init, locale } from 'svelte-intl-precompile';
import { registerAll } from '$locales';
import { fetchConfig } from '$lib/api/config';

registerAll();

const SUPPORTED_LOCALES = ['en', 'ru', 'de', 'es', 'fr', 'zh', 'jp', 'ge', 'he'];

const LOCALE_STORAGE_KEY = 'app-locale';

function getInitialLocale(): string {
	if (typeof window === 'undefined') {
		return 'en';
	}

	const savedLocale = localStorage.getItem(LOCALE_STORAGE_KEY);
	if (savedLocale && SUPPORTED_LOCALES.includes(savedLocale)) {
		return savedLocale;
	}

	const browserLocale = getLocaleFromNavigator();
	if (browserLocale) {
		if (SUPPORTED_LOCALES.includes(browserLocale)) {
			return browserLocale;
		}

		if (browserLocale.startsWith('ka')) {
			return 'ge';
		}

		if (browserLocale.startsWith('ja')) {
			return 'jp';
		}

		const languageCode = browserLocale.split('-')[0];
		if (SUPPORTED_LOCALES.includes(languageCode)) {
			return languageCode;
		}
	}

	return 'en';
}

const initialLocale = getInitialLocale();

init({ initialLocale, fallbackLocale: 'en' });

if (typeof window !== 'undefined') {
	locale.subscribe((value) => {
		if (value) {
			localStorage.setItem(LOCALE_STORAGE_KEY, value);
		}
	});
}

export const ssr = false;

export async function load() {
	try {
		await fetchConfig();
	} catch (e) {
		console.error('Failed to load config:', e);
	}
}
