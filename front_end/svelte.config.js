import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html', // for SPA mode
			precompress: false,
			strict: false
		}),
		paths: {
			base: process.env.BASE_PATH || '' // Only set if BASE_PATH is defined
		}
	},
	preprocess: vitePreprocess()
};

export default config;
