<script lang="ts">
	import { theme } from '../lib/stores/theme';
	import type { LanguageCategory } from '../lib/types';
	import IconWithName from './IconWithName.svelte';

	export let languageCategories: LanguageCategory[];

	$: bgClass = $theme === 'dark' ? 'bg-gray-800' : 'bg-gray-100';
	$: textClass = $theme === 'dark' ? 'text-white' : 'text-gray-900';
</script>

<div class="mb-12 flex justify-center">
	<div class="grid w-full max-w-4xl grid-cols-1 gap-8 md:grid-cols-3">
		{#each languageCategories as category}
			<div class="flex flex-col items-center gap-6 rounded-lg {bgClass} p-6 shadow-lg">
				<h3 class="text-center text-xl font-bold {textClass}">{category.title}</h3>
				<div class="flex flex-wrap justify-center gap-6">
					{#each category.items as tech}
						<IconWithName
							icon={tech.icon}
							name={tech.name}
							iconSize={48}
							invertFlag={$theme === 'light'}
						/>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
