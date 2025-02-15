<script>
	import Portfolio from '../sections/Portfolio.svelte';
	import ResumeModal from '../components/ResumeModal.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		const theme = localStorage.getItem('theme') || 'light'; // Get theme from localStorage or default to light
		document.documentElement.setAttribute('data-theme', theme);
	});
</script>

<ResumeModal />
<Portfolio />
