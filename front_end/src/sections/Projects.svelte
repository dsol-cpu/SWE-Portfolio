<script lang="ts">
	import ProjectCard from '../components/ProjectCard.svelte';

	const projects = [
		{
			title: 'HTML TUTORIAL',
			imageUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/2bfa928eecd225d8a4be950c63634e9608b7f6783972fe9e2145a6de1adb5426',
			imageAlt: 'HTML Tutorial Project Screenshot',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/836107239b15f597b011636150c87b1fc753035a1ed4748bf7f038fd561d8903',
			githubUrl: 'https://github.com/dsol-cpu/monster-mash',
			href: '#'
		},
		{
			title: 'CSS TUTORIAL',
			imageUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/2c42e7c3136d77099145b9b776d5110dbd9633e1fd884f2f7a7bd94f34899307',
			imageAlt: 'CSS Tutorial Project Screenshot',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			href: '#'
		}
	];
</script>

<section id="projects" class="flex flex-col items-center">
	<h2
		class=" mt-20 text-center text-lg font-semibold tracking-[2.4px] text-stone-300 sm:mt-24 sm:text-xl"
	>
		PROJECTS
	</h2>
	<div class="mt-7 w-[810px] max-w-full">
		<div class="flex gap-5 max-md:flex-col">
			{#each projects as project}
				<ProjectCard {...project} />
			{/each}
		</div>
	</div>
</section>
