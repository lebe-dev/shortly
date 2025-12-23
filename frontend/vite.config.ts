import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import precompileIntl from 'svelte-intl-precompile/sveltekit-plugin';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), precompileIntl('locales')],

	server: {
		allowedHosts: ['shortly.dev']
		// proxy: {
		// 	'/api': {
		// 		target: 'http://localhost:18080',
		// 		changeOrigin: true
		// 	}
		// }
	}
});
