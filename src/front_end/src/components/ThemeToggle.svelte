<script lang="ts">
	import { theme } from '../lib/stores/theme';

	$: isDark = $theme === 'dark';
</script>

<button
	on:click={() => theme.toggleTheme()}
	class="relative h-10 w-10 rounded-lg p-2 hover:bg-gray-200/10 focus:outline-none focus:ring-2 focus:ring-blue-500"
	aria-label="Toggle theme"
>
	<div class="relative h-6 w-6 overflow-hidden">
		<!-- Sun icon -->
		<img
			src="/sun.svg"
			alt="Light mode"
			class="absolute h-6 w-6 transform transition-transform duration-500 {isDark
				? 'translate-y-6'
				: 'translate-y-0'}"
		/>
		<!-- Moon icon -->
		<img
			src="/moon.svg"
			alt="Dark mode"
			class="absolute h-6 w-6 transform transition-transform duration-500 {isDark
				? 'translate-y-0'
				: '-translate-y-6'}"
		/>
	</div>
</button>
