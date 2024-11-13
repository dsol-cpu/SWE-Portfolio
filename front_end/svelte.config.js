import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
const dev = process.env.NODE_ENV === 'development';
/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html', // for SPA mode
			precompress: false,
			strict: false // add this line
		}),
		paths: {
			base: dev ? '' : '/SWE-Portfolio'
		}
	},
	preprocess: vitePreprocess()
};

export default config;
