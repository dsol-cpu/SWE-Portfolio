<script lang="ts">
	import ExperienceItem from '../components/ExperienceItem.svelte';
	import LanguageCategories from '../components/LanguageCategories.svelte';
	import { experienceItems, languageCategories } from '../lib/data';

	import { theme } from '../lib/stores/theme';
	$: sectionClass = $theme === 'dark' ? 'text-white' : 'text-zinc-900';
</script>

<section id="experience" class="styled-section flex flex-col items-center {sectionClass}">
	<div class="container mx-auto px-4 py-8">
		<LanguageCategories {languageCategories} />

		<div class="mx-auto flex max-w-4xl flex-col items-center space-y-6">
			{#each experienceItems as item}
				<ExperienceItem {...item} />
			{/each}
		</div>
	</div>
</section>

<style>
	@import '../lib/styles/styles.css';
	:global(.styled-section) {
		width: 100%;
	}
</style>
