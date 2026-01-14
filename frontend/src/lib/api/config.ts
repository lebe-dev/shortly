import type { AppConfig } from '$lib/domain/config';
import { authStore } from '$lib/stores/auth';
import { configStore } from '$lib/stores/config';
import { HttpError } from './error';

export async function fetchConfig(): Promise<AppConfig> {
	const response = await fetch(`/api/config`, {
		method: 'GET',
		credentials: 'include'
	});

	if (response.status === 200) {
		const config: AppConfig = await response.json();

		authStore.set({
			authenticated: config.session.authenticated,
			user: config.session.user
		});

		configStore.set(config);

		return config;
	} else if (response.status !== 0) {
		throw new HttpError(response.status, response.statusText);
	} else if (response.status == 0) {
		throw new HttpError(response.status, 'Network error');
	} else {
		throw new Error('Config fetch error');
	}
}

export const refreshConfig = fetchConfig;
