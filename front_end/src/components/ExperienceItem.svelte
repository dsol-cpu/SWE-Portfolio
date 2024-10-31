<script lang="ts">
	export let companyLogo: string;
	export let companyName: string;
	export let position: string;
	export let description: string;
	export let technologies: { icon: string; name: string }[] = []; // New prop for technology icons
	export let logoWidth = 34;
	export let logoAspectRatio = 1.06;
	export let gapSize = 5;

	$: logoHeight = logoWidth / logoAspectRatio;
	$: logoStyle = `width: ${logoWidth}px; height: ${logoHeight}px;`;
	$: gapStyle = `gap: ${gapSize}px`;
</script>

<article
	class="bg-gray-1000 mx-auto w-full max-w-3xl rounded-lg p-4 shadow-md transition-all duration-300 ease-in-out hover:-translate-y-2 hover:shadow-xl md:p-6"
>
	<div class="flex flex-col items-center gap-4 md:gap-5">
		<div
			class="flex flex-col items-center gap-3 text-xl font-bold text-stone-300 md:flex-row md:text-2xl"
			style={gapStyle}
		>
			<div class="shrink-0" style={logoStyle}>
				<img
					loading="lazy"
					src={companyLogo}
					alt="{companyName} Logo"
					style={logoStyle}
					class="object-contain"
				/>
			</div>
			<h3 class="text-center md:text-left">
				<span class="block text-stone-300 md:inline">{position}</span>
				<span class="md:before:content-[' at '] block text-stone-300 md:inline">{companyName}</span>
			</h3>
		</div>
	</div>

	<p class="mt-4 text-sm leading-6 text-stone-300 md:mt-6">
		{description}
	</p>

	{#if technologies.length > 0}
		<div class="mt-4 flex flex-wrap justify-center gap-4">
			{#each technologies as tech}
				<div class="flex flex-col items-center gap-1">
					<img
						src={tech.icon}
						alt="{tech.name} icon"
						class="h-6 w-6 object-contain"
						loading="lazy"
					/>
					<span class="text-xs text-stone-400">{tech.name}</span>
				</div>
			{/each}
		</div>
	{/if}
</article>

<style>
	article {
		transition: all 0.3s ease-in-out;
	}

	:global(.dark-theme) article {
		background-color: #2a2a2a;
		color: white;
	}
</style>
