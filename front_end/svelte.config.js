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
			strict: false // add this line
		}),
		paths: {
			base: '' // update this if you're not deploying to root
		}
	},
	preprocess: vitePreprocess()
};

export default config;
