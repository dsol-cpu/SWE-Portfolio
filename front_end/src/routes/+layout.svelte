<script>
	import '../app.css';
	let { children } = $props();
</script>

<svelte:head>
	<!-- Basic Open Graph Meta Tags for Preview Thumbnail -->
	<meta property="og:title" content="David Solinsky's software engineer portfolio website" />
	<meta property="og:description" content="David Solinsky's software engineer portfolio website" />
	<meta property="og:image" content="portfolio_website_icon.svg" />
	<meta property="og:url" content="https://dsol-cpu.github.io/SWE-Portfolio" />
	<meta property="og:type" content="website" />

	<!-- Optional Twitter Card Tags -->
	<meta name="twitter:card" content="portfolio_website_icon.svg" />
	<meta name="twitter:title" content="David Solinsky's software engineer portfolio website" />
	<meta
		name="twitter:description"
		content="David Solinsky's software engineer portfolio website."
	/>
	<meta name="twitter:image" content="portfolio_website_icon.svg" />
</svelte:head>

{@render children()}
