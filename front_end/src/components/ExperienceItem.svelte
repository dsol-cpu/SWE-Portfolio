<script lang="ts">
	interface ExperienceItemProps {
		companyLogo: string;
		companyName: string;
		position: string;
		dateRange: string;
		description: string;
		logoWidth?: number;
		logoAspectRatio?: number;
		gapSize?: number;
	}

	export let companyLogo: string;
	export let companyName: string;
	export let position: string;
	export let dateRange: string;
	export let description: string;
	export let logoWidth: number = 34;
	export let logoAspectRatio: number = 1.06;
	export let gapSize: number = 5;

	// Calculate height based on aspect ratio
	$: logoHeight = logoWidth / logoAspectRatio;

	// Function to handle image load and resize if needed
	function handleImageLoad(event: Event) {
		const img = event.target as HTMLImageElement;
		if (img) {
			img.width = logoWidth;
			img.height = logoHeight;
		}
	}
</script>

<article
	class="w-[801px] max-w-full rounded-lg bg-white p-6 shadow-md transition-all duration-300 ease-in-out hover:-translate-y-2 hover:shadow-xl"
>
	<div class="flex items-center justify-between gap-5">
		<div class="flex items-center text-2xl font-bold text-gray-900" style="gap: {gapSize}px">
			<div style="width: {logoWidth}px; height: {logoHeight}px;" class="shrink-0">
				<img
					loading="lazy"
					src={companyLogo}
					alt="{companyName} Logo"
					on:load={handleImageLoad}
					style="width: {logoWidth}px; height: {logoHeight}px; object-fit: contain;"
				/>
			</div>
			<h3>{position} at {companyName}</h3>
		</div>
		<time class="text-sm text-gray-500">{dateRange}</time>
	</div>
	<p class="mt-6 text-sm leading-6 text-gray-600 max-md:max-w-full">
		{description}
	</p>
</article>
