<script lang="ts">
	import { theme } from '../lib/stores/theme';
	import Header from './Header.svelte';
	import Hero from './Hero.svelte';
	import Projects from './Projects.svelte';
	import Experience from './Experience.svelte';
	import Footer from './Footer.svelte';

	$: mainBgClass = $theme === 'dark' ? 'bg-[#4e1939]' : 'bg-gray-100';
	$: contentBgClass = $theme === 'dark' ? 'bg-[#1C1C1C]' : 'bg-white';
</script>

<main class="flex flex-col items-center overflow-hidden {mainBgClass}">
	<div class={contentBgClass}>
		<Header />
		<Hero />
		<Experience />
		<Projects />
	</div>
	<Footer />
</main>
