import { HttpError } from './error';

export interface SessionResponse {
	authenticated: boolean;
	user?: UserInfo;
}

export interface UserInfo {
	username: string;
	email?: string;
	avatar_url?: string;
}

export async function checkSession(): Promise<SessionResponse> {
	const response = await fetch('/api/auth/session', {
		method: 'GET',
		credentials: 'include' // Include cookies
	});

	if (response.status === 200) {
		return response.json();
	} else {
		throw new HttpError(response.status, 'Failed to check session');
	}
}

export async function logout(): Promise<void> {
	const response = await fetch('/api/auth/logout', {
		method: 'POST',
		credentials: 'include'
	});

	if (response.status !== 200) {
		throw new HttpError(response.status, 'Logout failed');
	}
}
