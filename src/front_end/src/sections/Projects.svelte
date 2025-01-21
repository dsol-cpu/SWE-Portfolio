<script lang="ts">
	import ProjectCard from '../components/ProjectCard.svelte';
	import { BACKEND_ROUTE, BACKEND_URL, GITHUB_API_ROUTE } from '../lib/constants';

	interface Project {
		title: string;
		imageUrl: string;
		imageAlt: string;
		arrowIconUrl: string;
		githubUrl: string;
		description: string;
		projectStatus: string;
		deploymentStatus: string;
		keyFeatures: string[];
		technologies: string[];
		youtubeUrl?: string;
		demoUrl?: string;
		last_updated_at?: string;
	}

	interface GitHubInfo {
		name: string;
		last_updated_at: string;
	}

	let projects: Project[] = [
		{
			title: 'Portfolio Website',
			imageUrl: 'portfolio_website_icon.svg',
			imageAlt: 'Portfolio Website',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/836107239b15f597b011636150c87b1fc753035a1ed4748bf7f038fd561d8903',
			githubUrl: 'https://github.com/dsol-cpu/SWE-Portfolio',
			description:
				"You're here! I wanted to better organize my portfolio website from the one I did in ReactJS a couple years ago, so here we are!",
			projectStatus: 'In Progress',
			deploymentStatus: 'Live',
			keyFeatures: ['Front-end', 'Back-end', 'RESTful API', 'Serverless'],
			technologies: ['Svelte', 'Sveltekit', 'Rust', 'Actix', 'Github Pages', 'Redis', 'MongoDB'],
			demoUrl: 'https://dsol-cpu.github.io/SWE-Portfolio'
		},
		{
			title: 'Geospatial Mapping Challenge',
			imageUrl: 'https://img.youtube.com/vi/coCoAvOaSBM/0.jpg',
			imageAlt: '',
			arrowIconUrl:
				'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
			githubUrl: 'https://github.com/dsol-cpu/GeospatialDataVisualization-Aug2024',
			description:
				'Parses a json file for geospatial data and uses a regex to deal with dirty/corrupted/invalid data. That data is correlated with each pin instantiated on the globe, allowing the user to mouse hover and display the corresponding location data.',
			projectStatus: 'Completed',
			deploymentStatus: 'Live',
			keyFeatures: ['Data Visualization', 'Game Development'],
			technologies: ['Unity', 'C#', 'JSON', 'Blender'],
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
			description:
				'This was a challenge to gameify ufo sightings data. Extracts a number of random pieces of data and displays on UI near screens. Intended behavior was that all screens would amount to a password using the data and the ship would take that password, ending the game.',
			projectStatus: 'Completed',
			deploymentStatus: 'Live',
			keyFeatures: ['Data Visualization', 'Game Development'],
			technologies: ['Unity', 'C#', 'JSON'],
			demoUrl: 'https://dsol-cpu.github.io/alien-search/'
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
			title: 'MMORPG Database & Interface',
			imageUrl: 'db_icon.svg',
			imageAlt: 'MMORPG Database and Interface',
			arrowIconUrl: '',
			githubUrl: 'https://github.com/dsol-cpu/MMORG-DB',
			description:
				'A small python CLI interface made for a mock MMORPG relational database postgreSQL',
			projectStatus: 'Completed',
			deploymentStatus: 'Local Only',
			keyFeatures: ['CLI', 'Database'],
			technologies: ['Python', 'PostgreSQL']
		}
	];

	async function fetchGitHubInfo() {
		try {
			const response = await fetch(`${BACKEND_URL}${BACKEND_ROUTE}${GITHUB_API_ROUTE}`, {
				method: 'GET',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				}
			});

			if (!response.ok) {
				throw new Error('Failed to fetch GitHub info');
			}

			const data: GitHubInfo[] = await response.json();

			// Update projects with last_updated_at information
			projects = projects.map((project) => {
				const repoName = project.githubUrl.split('/').pop();
				const repoInfo = data.find((info) => info.name === repoName);
				return {
					...project,
					last_updated_at: repoInfo?.last_updated_at
				};
			});
		} catch (error) {
			console.error('Error fetching GitHub info:', error);
		}
	}

	import { onMount } from 'svelte';
	onMount(() => {
		fetchGitHubInfo();
	});
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
