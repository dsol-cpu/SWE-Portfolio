<script lang="ts">
	import TechCategory from './TechCategory.svelte';
	export let categories: TechCategory[];
</script>

<section class="mx-auto flex w-full max-w-7xl flex-col items-center px-4">
	<h2
		class="mt-20 text-center text-lg font-semibold tracking-[2.4px] text-stone-300 sm:mt-24 sm:text-xl"
	>
		EXPERIENCE
	</h2>
	<div class="w-full max-w-6xl">
		<div class="mt-8 flex flex-col gap-12 sm:mt-12 md:flex-row md:justify-center md:gap-16">
			{#each categories as category}
				<TechCategory title={category.title} items={category.items} />
			{/each}
		</div>
	</div>
</section>
