import { writable } from 'svelte/store';
import type { AppConfig } from '$lib/domain/config';

export const configStore = writable<AppConfig | null>(null);
