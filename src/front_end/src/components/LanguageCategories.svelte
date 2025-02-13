<script lang="ts">
	import type { LanguageCategory } from '../lib/types';
	import IconWithName from './IconWithName.svelte';
	export let languageCategories: LanguageCategory[];
</script>

<div class="mb-12 flex justify-center">
	<div class="grid w-full max-w-4xl grid-cols-1 gap-8 md:grid-cols-3">
		{#each languageCategories as category}
			<div class="flex flex-col items-center gap-6 rounded-lg bg-gray-800 p-6 shadow-lg">
				<h3 class="text-center text-xl font-bold">{category.title}</h3>
				<div class="flex flex-wrap justify-center gap-6">
					{#each category.items as tech}
						<IconWithName icon={tech.icon} name={tech.name} iconSize={48} />
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
