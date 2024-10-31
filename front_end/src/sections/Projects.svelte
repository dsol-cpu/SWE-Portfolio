<script lang="ts">
	import ProjectCard from '../components/ProjectCard.svelte';
	const projects = [
		{
			title: 'Portfolio Website',
			imageUrl: 'portfolio_website_icon.svg',
			imageAlt: 'Portfolio Website',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/836107239b15f597b011636150c87b1fc753035a1ed4748bf7f038fd561d8903',
			githubUrl: 'https://github.com/dsol-cpu/SWE-Portfolio',
			href: 'https://dsol-cpu.github.io/SWE-Portfolio'
		},
		{
			title: 'MMORPG Themed DB and Interface',
			imageUrl: 'db_icon.svg',
			imageAlt: 'MMORPG Themed Database and Interface',
			arrowIconUrl: '',
			githubUrl: 'https://github.com/dsol-cpu/MMORG-DB',
			href: 'https://github.com/dsol-cpu/MMORG-DB'
		},
		{
			title: 'Mario Kart Blender Addon',
			imageUrl: 'https://img.youtube.com/vi/QDt2I_BZKrI/0.jpg',
			imageAlt: 'Mario Kart Blender Addon',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			githubUrl: 'https://github.com/dsol-cpu/MK-AREAS-plugin',
			href: 'https://www.youtube.com/watch?v=QDt2I_BZKrI'
		},
		{
			title: 'Geospatial Data Visualization',
			imageUrl: 'https://img.youtube.com/vi/coCoAvOaSBM/0.jpg',
			imageAlt: '',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			githubUrl: 'https://github.com/dsol-cpu/AESCodeChallenge-August-2024',
			href: 'https://youtu.be/coCoAvOaSBM'
		}
	];
</script>

<section id="projects" class="flex flex-col items-center">
	<h
		class="mt-7 text-center text-lg font-semibold tracking-[2.4px] text-stone-300 sm:mt-16 sm:text-xl"
	>
		PROJECTS
	</h>
	<div class="mt-7 w-full max-w-5xl px-4">
		<div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
			{#each projects as project}
				<ProjectCard {...project} />
			{/each}
		</div>
	</div>
</section>
