import { writable } from 'svelte/store';
import type { SessionResponse } from '$lib/api/auth';

export const authStore = writable<SessionResponse>({
	authenticated: false,
	user: undefined
});

export const authLoading = writable<boolean>(true);
