<script lang="ts">
	import Header from './Header.svelte';
	import Hero from './Hero.svelte';
	import Projects from './Projects.svelte';
	import Experience from './Experience.svelte';
	import Footer from './Footer.svelte';
</script>

<main class="bg-color flex flex-col items-center overflow-hidden">
	<div class="bg-[#1C1C1C]">
		<Header />
		<Hero />
		<Experience />
		<Projects />
	</div>
	<Footer />
</main>

<style>
	.bg-color {
		background-color: #4e1939; /* yellow */
	}
</style>
