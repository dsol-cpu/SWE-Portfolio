<script lang="ts">
	interface ProjectCardProps {
		title: string;
		imageUrl: string;
		imageAlt: string;
		arrowIconUrl: string;
		href: string;
	}

	export let title: string;
	export let imageUrl: string;
	export let imageAlt: string;
	export let arrowIconUrl: string;
	export let href: string;
</script>

<article class="flex w-6/12 flex-col max-md:ml-0 max-md:w-full">
	<div class="flex grow flex-col font-extrabold uppercase max-md:mt-8">
		<a {href} class="flex w-full flex-col rounded-2xl bg-zinc-800 pb-3.5">
			<img
				loading="lazy"
				src={imageUrl}
				alt={imageAlt}
				class="aspect-[1.66] w-full rounded-2xl object-contain"
			/>
			<div class="ml-6 mr-6 mt-3 flex justify-between gap-5 max-md:mx-2.5">
				<div class="flex flex-col">
					<span class="self-start text-xs leading-5 text-stone-300">CLICK HERE TO VISIT</span>
					<h3 class="text-xl leading-none text-white">{title}</h3>
				</div>
				<img
					loading="lazy"
					src={arrowIconUrl}
					alt=""
					class="my-auto aspect-[1.09] w-3 shrink-0 object-contain"
				/>
			</div>
		</a>
	</div>
</article>
