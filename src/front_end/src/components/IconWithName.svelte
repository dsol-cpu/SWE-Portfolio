<script lang="ts">
	export let icon: string; // The URL of the icon image
	export let name: string; // The name to display underneath the icon
	export let iconSize: number = 40; // New export variable for icon size
	export let nameSize: number = 16;
	export let invertFlag: boolean = false;
</script>

<div class="icon-container">
	<img
		loading="lazy"
		src={icon}
		alt={name}
		class="icon"
		style="width: {iconSize}px; height: {iconSize}px; {invertFlag ? 'filter: invert(1);' : ''}"
	/>
	<span class="icon-name" style="font-size: {nameSize}px">{name}</span>
</div>

<style>
	.icon-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center; /* Center the name under the icon */
	}

	.icon {
		object-fit: contain; /* Ensure the icon maintains aspect ratio */
		transition: transform 0.1s ease-in-out;
	}

	.icon-container:hover .icon {
		transform: scale(1.1); /* Scale effect on hover */
	}

	.icon-name {
		margin-top: 4px; /* Space between icon and name */
		color: white; /* Text color */
		font-size: 12px; /* Font size for the name */
	}
</style>
