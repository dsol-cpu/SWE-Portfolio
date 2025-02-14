<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { ENV_CONFIG } from '../lib/constants';
	import { theme } from '$lib/stores/theme';

	//TODO: Make this feature actually ping for to query for a cached value
	// https: onMount(async () => {
	// 	try {
	// 		// Make a fetch request to the backend
	// 		const response = await fetch(`${BACKEND_URL}${BACKEND_ROUTE}${PAGE_VISIT_ROUTE_PATH}`, {
	// 			method: 'GET'
	// 		});

	// 		if (!response.ok) {
	// 			console.error('Failed to log visit:', response.statusText);
	// 		} else {
	// 			console.log('Visit logged successfully');
	// 		}
	// 	} catch (error) {
	// 		console.error('Error logging visit:', error);
	// 	}
	// });

	let { children } = $props();
</script>

<svelte:head>
	<meta property="og:title" content="David Solinsky's software engineer portfolio website" />
	<meta property="og:description" content="David Solinsky's software engineer portfolio website" />
	<meta property="og:image" content="portfolio_website_icon.svg" />
	<meta property="og:url" content="https://dsol-cpu.github.io/SWE-Portfolio" />
	<meta property="og:type" content="website" />

	<meta name="twitter:card" content="portfolio_website_icon.svg" />
	<meta name="twitter:title" content="David Solinsky's software engineer portfolio website" />
	<meta
		name="twitter:description"
		content="David Solinsky's software engineer portfolio website."
	/>
	<meta name="twitter:image" content="portfolio_website_icon.svg" />
</svelte:head>

{@render children()}
