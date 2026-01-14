import { writable } from 'svelte/store';
import type { SessionDto } from '$lib/domain/config';

export const authStore = writable<SessionDto>({
	authenticated: false,
	user: undefined
});
