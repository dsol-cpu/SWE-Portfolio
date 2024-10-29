<script lang="ts">
	export let title: string;
	export let imageUrl: string;
	export let imageAlt: string;
	export let githubUrl: string;
	export let href: string;
</script>

<article class="flex w-6/12 flex-col max-md:ml-0 max-md:w-full">
	<div class="flex grow flex-col font-extrabold uppercase max-md:mt-8">
		<div class="flex w-full flex-col rounded-2xl bg-zinc-800 pb-3.5">
			<a {href} target="_blank" rel="noopener noreferrer">
				<img
					loading="lazy"
					src={imageUrl}
					alt={imageAlt}
					class="aspect-[1.66] w-full rounded-2xl object-contain"
				/>
			</a>
			<div class="ml-6 mr-6 mt-3 flex justify-between gap-5 max-md:mx-2.5">
				<div class="flex flex-col">
					<span class="self-start text-xs leading-5 text-stone-300">CLICK HERE TO VISIT</span>
					<h3 class="text-xl leading-none text-white">{title}</h3>
				</div>
				<a
					href={githubUrl}
					target="_blank"
					rel="noopener noreferrer"
					class="my-auto w-6 object-contain text-red-900 transition-colors hover:text-red-900"
				>
					<img
						src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/github/github-original.svg"
						alt="GitHub"
						class="h-full w-full"
					/>
				</a>
			</div>
		</div>
	</div>
</article>
