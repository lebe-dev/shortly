import { HttpError } from './error';

export async function logout(): Promise<void> {
	const response = await fetch('/api/auth/logout', {
		method: 'POST',
		credentials: 'include'
	});

	if (response.status !== 200) {
		throw new HttpError(response.status, 'Logout failed');
	}
}
