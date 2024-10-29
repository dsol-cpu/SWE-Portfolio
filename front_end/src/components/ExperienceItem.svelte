<script lang="ts">
	// In Svelte, we typically don't need to declare an interface for props
	// unless we're using TypeScript for external consumption/documentation
	export let companyLogo: string;
	export let companyName: string;
	export let position: string;
	export let description: string;
	export let logoWidth = 34; // Default values can be set directly
	export let logoAspectRatio = 1.06;
	export let gapSize = 5;

	// Reactive declarations in Svelte use $: syntax
	$: logoHeight = logoWidth / logoAspectRatio;
	$: logoStyle = `width: ${logoWidth}px; height: ${logoHeight}px;`;
	$: gapStyle = `gap: ${gapSize}px`;
</script>

<article
	class="w-[801px] max-w-full rounded-lg bg-white p-6 shadow-md transition-all duration-300 ease-in-out hover:-translate-y-2 hover:shadow-xl"
>
	<div class="flex flex-col items-center gap-5">
		<div class="flex items-center text-2xl font-bold text-gray-900" style={gapStyle}>
			<!-- In Svelte, we can use bind:clientWidth if we need to dynamically adjust size -->
			<div class="shrink-0" style={logoStyle}>
				<img
					loading="lazy"
					src={companyLogo}
					alt="{companyName} Logo"
					style={logoStyle}
					class="object-contain"
				/>
			</div>
			<h3 class="text-center">{position} at {companyName}</h3>
		</div>
	</div>
	<p class="mt-6 text-sm leading-6 text-gray-600 max-md:max-w-full">
		{description}
	</p>
</article>

<!-- Svelte allows styles to be scoped by default -->
<style>
	/* We could move transition styles here */
	article {
		transition: all 0.3s ease-in-out;
	}

	/* Example of using Svelte's :global modifier if needed */
	:global(.dark-theme) article {
		background-color: #2a2a2a;
		color: white;
	}
</style>
