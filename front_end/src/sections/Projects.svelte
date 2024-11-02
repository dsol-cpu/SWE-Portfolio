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
			href: 'https://dsol-cpu.github.io/SWE-Portfolio',
			description:
				"You're here! I wanted to better organize my portfolio website from the one I did in ReactJS a couple years ago, so here we are!",
			projectStatus: 'In Progress',
			deploymentStatus: 'Live',
			keyFeatures: ['Front-end', 'Back-end', 'RESTful API'],
			technologies: ['Svelte', 'Sveltekit', 'Rust', 'Actix', 'Github Pages']
		},
		{
			title: 'MMORPG Database & Interface',
			imageUrl: 'db_icon.svg',
			imageAlt: 'MMORPG Database and Interface',
			arrowIconUrl: '',
			githubUrl: 'https://github.com/dsol-cpu/MMORG-DB',
			href: 'https://github.com/dsol-cpu/MMORG-DB',
			description:
				'A small python CLI interface made for a mock MMORPG relational database postgreSQL',
			projectStatus: 'Completed',
			deploymentStatus: 'Local Only',
			keyFeatures: ['Front-end', 'Back-end'],
			technologies: ['Python', 'PostgreSQL']
		},
		{
			title: 'Moving Terrain Editor Addon (Mario Kart)',
			imageUrl: 'https://img.youtube.com/vi/QDt2I_BZKrI/0.jpg',
			imageAlt: 'Mario Kart Blender Addon',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			githubUrl: 'https://github.com/dsol-cpu/MK-AREAS-plugin',
			description:
				"Allows the user to import .kmp files and represent data as cube primitives, manipulate primitives' transform in Blender, and export transforms to manipulate their corresponding moving terrain (Conveyor belts, water current surfaces)",
			projectStatus: 'Completed',
			deploymentStatus: 'Local Only',
			keyFeatures: ['Data Visualization', 'Blender Addon'],
			technologies: ['Blender API', 'Python'],
			youtubeUrl: 'https://www.youtube.com/watch?v=QDt2I_BZKrI'
		},
		{
			title: 'Geospatial Mapping Challenge',
			imageUrl: 'https://img.youtube.com/vi/coCoAvOaSBM/0.jpg',
			imageAlt: '',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			githubUrl: 'https://github.com/dsol-cpu/GeospatialDataVisualization-Aug2024',
			projectStatus: 'Completed',
			deploymentStatus: 'Live',
			keyFeatures: ['Data Visualization', 'Game Development'],
			technologies: ['Unity', 'C#', 'JSON'],
			youtubeUrl: 'https://youtu.be/coCoAvOaSBM',
			demoUrl: 'https://dsol-cpu.github.io/GeospatialDataVisualization-Aug2024/'
		},
		{
			title: 'UFO Sightings Challenge',
			imageUrl: 'alien_search_icon.svg',
			imageAlt: '',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			githubUrl: 'https://github.com/dsol-cpu/alien-search',
			href: 'https://github.com/dsol-cpu/alien-search',
			projectStatus: 'Completed',
			deploymentStatus: 'Live',
			keyFeatures: ['Data Visualization', 'Game Development'],
			technologies: ['Unity', 'C#', 'JSON'],
			demoUrl: 'https://dsol-cpu.github.io/alien-search/'
		}
	];
</script>

<section id="projects" class="styled-section mb-16 flex flex-col items-center rounded-lg p-6">
	<h3
		class="mt-7 text-center text-lg font-semibold tracking-[2.4px] text-stone-300 sm:mt-16 sm:text-xl"
	>
		PROJECTS
	</h3>
	<div class="mt-7 w-full max-w-5xl px-4">
		<div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
			{#each projects as project}
				<ProjectCard {...project} />
			{/each}
		</div>
	</div>
</section>

<style>
	@import '../lib/styles/styles.css';
</style>
