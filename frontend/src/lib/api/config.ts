import type { AppConfig } from '$lib/domain/config';
import { HttpError } from './error';

export async function fetchConfig(): Promise<AppConfig> {
	const response = await fetch(`/api/config`, {
		method: 'GET'
	});

	if (response.status === 200) {
		return response.json();
	} else if (response.status !== 0) {
		throw new HttpError(response.status, response.statusText);
	} else if (response.status == 0) {
		throw new HttpError(response.status, 'Network error');
	} else {
		throw new Error('Config fetch error');
	}
}
