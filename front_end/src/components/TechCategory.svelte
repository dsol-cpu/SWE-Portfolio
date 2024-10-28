<script lang="ts">
	import TechIcon from './TechIcon.svelte';

	export let title: string;
	export let items: Array<{ name: string; icon: string }>;
</script>

<div class="flex flex-col items-center">
	<h3 class="mb-4 text-center text-sm font-medium text-stone-400">
		{title}
	</h3>
	<div class="grid grid-cols-3 items-center justify-center gap-6 px-4 sm:flex sm:gap-10">
		{#each items as item}
			<TechIcon name={item.name} icon={item.icon} />
		{/each}
	</div>
</div>
