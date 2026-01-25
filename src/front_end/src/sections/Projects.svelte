<script lang="ts">
	import ProjectCard from '../components/ProjectCard.svelte';
	import { ENV_CONFIG } from '../lib/constants';
	import { projects } from '../lib/data';
	import type { Project, GitHubInfo } from '../lib/types';
	import { theme } from '../lib/stores/theme';
	import { onMount } from 'svelte';

	$: textClass = $theme === 'dark' ? 'text-stone-300' : 'text-zinc-900';

	let projectsData: Project[] = projects;

	async function fetchGitHubInfo() {
		try {
			const response = await fetch(
				`${ENV_CONFIG.BACKEND_URL}${ENV_CONFIG.BACKEND_ROUTE}${ENV_CONFIG.GITHUB_API_ROUTE}`,
				{
					method: 'GET',
					credentials: 'include',
					headers: {
						'Content-Type': 'application/json'
					}
				}
			);

			if (!response.ok) {
				throw new Error('Failed to fetch GitHub info');
			}

			const data: GitHubInfo[] = await response.json();

			projectsData = projectsData.map((project) => {
				const repoName = project.githubUrl?.split('/').pop();
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

	onMount(() => {
		//TODO: This is where I communicate with my backend
		// fetchGitHubInfo();
	});
</script>

<section id="projects" class="styled-section mb-16 flex flex-col items-center rounded-lg p-6">
	<h class="mt-7 text-center text-lg font-semibold tracking-[2.4px] {textClass} sm:mt-16 sm:text-xl">
		PROJECTS
	</h>
	<div class="mt-7 w-full max-w-5xl px-4">
		<div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
			{#each projectsData as project (project.title)}
				<ProjectCard {...project} />
			{/each}
		</div>
	</div>
</section>
