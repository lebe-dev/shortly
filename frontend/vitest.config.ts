import { configDefaults, defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		exclude: [...configDefaults.exclude, 'tests/integration/**'],
		globals: true,
		environment: 'jsdom'
	}
});
