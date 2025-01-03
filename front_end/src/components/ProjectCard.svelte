<script lang="ts">
	// Replace enums with constant objects
	const PROJECT_STATUS = {
		COMPLETED: 'Completed',
		IN_PROGRESS: 'In Progress',
		MAINTAINED: 'Maintained'
	} as const;

	const DEPLOYMENT_STATUS = {
		LIVE: 'Live',
		DEMO: 'Demo Available',
		LOCAL: 'Local Only'
	} as const;

	// Type aliases using typeof
	type ProjectStatus = (typeof PROJECT_STATUS)[keyof typeof PROJECT_STATUS];
	type DeploymentStatus = (typeof DEPLOYMENT_STATUS)[keyof typeof DEPLOYMENT_STATUS];

	// Type validation functions
	function validateProjectStatus(status: string): ProjectStatus {
		const validStatuses = Object.values(PROJECT_STATUS);
		if (!validStatuses.includes(status as ProjectStatus)) {
			throw new Error(
				`Invalid project status: ${status}. Must be one of: ${validStatuses.join(', ')}`
			);
		}
		return status as ProjectStatus;
	}

	function validateDeploymentStatus(status: string): DeploymentStatus {
		const validStatuses = Object.values(DEPLOYMENT_STATUS);
		if (!validStatuses.includes(status as DeploymentStatus)) {
			throw new Error(
				`Invalid deployment status: ${status}. Must be one of: ${validStatuses.join(', ')}`
			);
		}
		return status as DeploymentStatus;
	}

	// Updated interface for API response
	interface GitHubInfo {
		name: string;
		updated_at: string;
		pushed_at: string;
		cached: boolean; // Added to indicate if data came from cache
		cache_age?: number; // Optional: time since last cache update in seconds
	}

	// Props
	export let title: string;
	export let imageUrl: string;
	export let imageAlt: string;
	export let githubUrl: string;
	export let href: string;
	export let description: string;
	export let technologies: string[] = [];
	export let projectStatus: string;
	export let deploymentStatus: string;
	export let keyFeatures: string[] = [];
	export let role: string = '';
	// New props for YouTube and demo links
	export let youtubeUrl: string = '';
	export let demoUrl: string = '';

	// Updated interface for API response
	interface GitHubInfo {
		name: string;
		updated_at: string;
		pushed_at: string;
		cached: boolean; // Added to indicate if data came from cache
		cache_age?: number; // Optional: time since last cache update in seconds
	}

	// State variables
	let lastUpdated: string = '';
	let isLoadingDate = true;
	let dateError = false;
	let isCached = false;
	let cacheAge: number | null = null;

	// Validate statuses on component initialization
	let validatedProjectStatus: ProjectStatus;
	let validatedDeploymentStatus: DeploymentStatus;

	try {
		validatedProjectStatus = validateProjectStatus(projectStatus);
		validatedDeploymentStatus = validateDeploymentStatus(deploymentStatus);
	} catch (error) {
		console.error(error);
		// Provide fallback values if validation fails
		validatedProjectStatus = PROJECT_STATUS.IN_PROGRESS;
		validatedDeploymentStatus = DEPLOYMENT_STATUS.LOCAL;
	}

	// GraphQL query
	const GITHUB_INFO_QUERY = `
		query GetGitHubInfo($repoUrl: String!) {
			githubInfo(repoUrl: $repoUrl) {
				name
				updatedAt
				pushedAt
				cached
				cacheAge
			}
		}
	`;
	import { BACKEND_URL } from '../lib/constants';

	// Updated fetch function to use GraphQL
	async function fetchGitHubInfo() {
		try {
			isLoadingDate = true;
			const response_url: string = `${BACKEND_URL}/api/github_repos`;
			const response = await fetch(response_url, {
				method: 'POST',
				credentials: 'include',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					query: GITHUB_INFO_QUERY,
					variables: {
						repoUrl: githubUrl
					}
				})
			});

			if (!response.ok) {
				throw new Error('Failed to fetch GitHub info');
			}

			const { data, errors } = await response.json();

			if (errors) {
				throw new Error(errors[0].message);
			}

			const info = data.githubInfo;
			lastUpdated = info.pushedAt;
			isCached = info.cached;
			cacheAge = info.cacheAge;
			isLoadingDate = false;
		} catch (error) {
			console.error('Error fetching GitHub info:', error);
			dateError = true;
			isLoadingDate = false;
		}
	}

	// Format date helper function
	const formatDate = (dateString: string): string => {
		try {
			return new Date(dateString).toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'short'
			});
		} catch {
			return 'Unknown date';
		}
	};

	// Get status color classes
	const getProjectStatusColor = (status: ProjectStatus): string => {
		const colors = {
			[PROJECT_STATUS.COMPLETED]: 'bg-green-500/20 text-green-300',
			[PROJECT_STATUS.IN_PROGRESS]: 'bg-blue-500/20 text-blue-300',
			[PROJECT_STATUS.MAINTAINED]: 'bg-purple-500/20 text-purple-300'
		};
		return colors[status] ?? colors[PROJECT_STATUS.IN_PROGRESS];
	};

	const getDeploymentStatusColor = (status: DeploymentStatus): string => {
		const colors = {
			[DEPLOYMENT_STATUS.LIVE]: 'bg-emerald-500/20 text-emerald-300',
			[DEPLOYMENT_STATUS.DEMO]: 'bg-yellow-500/20 text-yellow-300',
			[DEPLOYMENT_STATUS.LOCAL]: 'bg-gray-500/20 text-gray-300'
		};
		return colors[status] ?? colors[DEPLOYMENT_STATUS.LOCAL];
	};

	import { onMount } from 'svelte';
	onMount(() => {
		fetchGitHubInfo();
	});
</script>

<article
	class="group relative h-auto min-h-[28rem] w-full overflow-hidden rounded-2xl bg-zinc-800 transition-all duration-300 hover:shadow-xl"
>
	<a {href} target="_blank" rel="noopener noreferrer" class="block h-48 overflow-hidden">
		<img
			loading="lazy"
			src={imageUrl}
			alt={imageAlt}
			class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
		/>
	</a>
	<div class="flex flex-col justify-between p-4 sm:p-6">
		<div class="space-y-4">
			<div class="flex items-start justify-between">
				<h3 class="flex-1 text-lg font-bold text-white sm:text-xl">
					{title}
				</h3>
				<div class="relative">
					<div class="absolute right-0 top-0 flex flex-col items-end space-y-2">
						{#if githubUrl}
							<a
								href={githubUrl}
								target="_blank"
								rel="noopener noreferrer"
								class="flex h-10 w-10 items-center justify-center rounded-full bg-zinc-700/50 p-2 transition-all duration-300 hover:bg-zinc-600/50"
								aria-label="View source on GitHub"
							>
								<img
									src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/github/github-original.svg"
									alt="GitHub"
									class="h-6 w-6 transition-all duration-300 hover:scale-[1.02] group-hover:brightness-125"
								/>
							</a>
						{/if}

						{#if youtubeUrl}
							<a
								href={youtubeUrl}
								target="_blank"
								rel="noopener noreferrer"
								class="flex h-10 w-10 items-center justify-center rounded-full bg-zinc-700/50 p-2 transition-all duration-300 hover:bg-zinc-600/50"
								aria-label="Watch demo on YouTube"
							>
								<svg class="h-6 w-6 fill-current text-red-500" viewBox="0 0 24 24">
									<path
										d="M19.615 3.184c-3.604-.246-11.631-.245-15.23 0-3.897.266-4.356 2.62-4.385 8.816.029 6.185.484 8.549 4.385 8.816 3.6.245 11.626.246 15.23 0 3.897-.266 4.356-2.62 4.385-8.816-.029-6.185-.484-8.549-4.385-8.816zm-10.615 12.816v-8l8 3.993-8 4.007z"
									/>
								</svg>
							</a>
						{/if}

						{#if demoUrl}
							<a
								href={demoUrl}
								target="_blank"
								rel="noopener noreferrer"
								class="flex h-10 w-10 items-center justify-center rounded-full bg-zinc-700/50 p-2 transition-all duration-300 hover:bg-zinc-600/50"
								aria-label="View live demo"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="h-6 w-6 text-white"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
									/>
								</svg>
							</a>
						{/if}
					</div>
				</div>
			</div>

			<!-- Project metadata -->
			<div class="flex flex-wrap gap-2 text-xs">
				<span class="rounded-full px-2 py-1 {getProjectStatusColor(validatedProjectStatus)}">
					{validatedProjectStatus}
				</span>
				<span class="rounded-full px-2 py-1 {getDeploymentStatusColor(validatedDeploymentStatus)}">
					{validatedDeploymentStatus}
				</span>
				{#if !dateError}
					<span class="rounded-full bg-purple-500/20 px-2 py-1 text-purple-300">
						{#if isLoadingDate}
							Loading update date...
						{:else}
							Updated {formatDate(lastUpdated)}
						{/if}
					</span>
				{/if}
			</div>

			<!-- Description -->
			<p class="text-sm text-gray-300">{description}</p>

			<!-- Technologies -->
			<div class="space-y-2">
				<h4 class="text-sm font-semibold text-gray-400">Technologies</h4>
				<div class="flex flex-wrap gap-2">
					{#each technologies as tech}
						<span class="rounded-full bg-zinc-700/50 px-2 py-1 text-xs text-white">
							{tech}
						</span>
					{/each}
				</div>
			</div>

			<!-- Key Features -->
			<div class="space-y-2">
				<h4 class="text-sm font-semibold text-gray-400">Key Features</h4>
				<ul class="list-inside list-disc space-y-1 text-sm text-gray-300">
					{#each keyFeatures as feature}
						<li>{feature}</li>
					{/each}
				</ul>
			</div>

			<!-- Role information if provided -->
			{#if role}
				<div class="text-sm text-gray-400">
					<span class="font-semibold">Role:</span>
					{role}
				</div>
			{/if}
		</div>
	</div>
</article>

<style>
	article::after {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom, transparent 0%, rgba(0, 0, 0, 0.1) 100%);
		opacity: 0;
		transition: opacity 0.3s ease;
		pointer-events: none;
		border-radius: 1rem;
	}

	article:hover::after {
		opacity: 1;
	}
</style>
