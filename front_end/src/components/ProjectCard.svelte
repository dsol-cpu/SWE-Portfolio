<script lang="ts">
	export let title: string;
	export let imageUrl: string;
	export let imageAlt: string;
	export let githubUrl: string;
	export let href: string;
	export let description: string; // New export variable for description
</script>

<article
	class="group relative h-96 max-h-96 w-full overflow-hidden rounded-2xl bg-zinc-800 transition-all duration-300 hover:shadow-xl"
>
	<a {href} target="_blank" rel="noopener noreferrer" class="block h-2/3 overflow-hidden">
		<img
			loading="lazy"
			src={imageUrl}
			alt={imageAlt}
			class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
		/>
	</a>
	<div class="flex h-1/3 flex-col justify-between p-4 sm:p-6">
		<div class="flex items-start gap-4">
			<h3 class=" line-clamp-2 flex-1 overflow-hidden text-lg font-bold text-white sm:text-xl">
				{title}
			</h3>
			<a
				href={githubUrl}
				target="_blank"
				rel="noopener noreferrer"
				class="relative flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-zinc-700/50 p-2 transition-all duration-300 hover:bg-zinc-600/50"
				aria-label="View source on GitHub"
			>
				<img
					src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/github/github-original.svg"
					alt="GitHub"
					class="h-6 w-6 transition-all duration-300 hover:scale-[1.02] group-hover:brightness-125"
				/>
				<div
					class="absolute -top-8 left-1/2 -translate-x-1/2 whitespace-nowrap rounded bg-zinc-900 px-2 py-1 text-xs text-white opacity-0 transition-opacity group-hover:opacity-100"
				>
					View Source
				</div>
			</a>
		</div>
		<p class="mt-2 text-sm text-gray-300">{description}</p>
	</div>
</article>

<style>
	/* Optional: Add a subtle gradient overlay on hover */
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
