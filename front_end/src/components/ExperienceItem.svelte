<script lang="ts">
	interface ExperienceItemProps {
		companyLogo: string;
		companyName: string;
		position: string;
		dateRange: string;
		description: string;
		logoWidth?: string;
		logoAspectRatio?: string;
		gapSize?: string;
	}

	export let companyLogo: string;
	export let companyName: string;
	export let position: string;
	export let dateRange: string;
	export let description: string;
	export let logoWidth: string = '34px';
	export let logoAspectRatio: string = '1.06';
	export let gapSize: string = '5';
</script>

<article class="w-[801px] max-w-full text-center">
	<div class="flex items-center justify-between gap-5">
		<div class="flex gap-{gapSize} items-center text-2xl font-bold text-white">
			<img
				loading="lazy"
				src={companyLogo}
				alt={`${companyName} Logo`}
				class="aspect-[{logoAspectRatio}] w-[{logoWidth}] shrink-0 object-contain"
			/>
			<h3>{position} at {companyName}</h3>
		</div>
		<time class="text-sm text-gray-400">{dateRange}</time>
	</div>
	<p class="mt-6 text-center text-sm leading-6 text-gray-400 max-md:max-w-full">
		{description}
	</p>
</article>
