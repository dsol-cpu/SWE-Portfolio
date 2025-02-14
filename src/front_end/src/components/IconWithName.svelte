<script lang="ts">
	import { theme } from '../lib/stores/theme';
	export let icon: string; // The URL of the icon image
	export let name: string; // The name to display underneath the icon
	export let iconSize: number = 40; // New export variable for icon size
	export let nameSize: number = 16;
	export let invertFlag: boolean = false;
	$: textClass = $theme === 'dark' ? 'text-white' : 'text-gray-900';
</script>

<div class="icon-container">
	<img
		loading="lazy"
		src={icon}
		alt={name}
		class="icon"
		style="width: {iconSize}px; height: {iconSize}px;' : ''}"
	/>
	<span class="icon-name {textClass}" style="font-size: {nameSize}px">{name}</span>
</div>

<style>
	.icon-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}
	.icon {
		object-fit: contain;
		transition: transform 0.1s ease-in-out;
	}
	.icon-container:hover .icon {
		transform: scale(1.1);
	}
	.icon-name {
		margin-top: 4px;
	}
</style>
