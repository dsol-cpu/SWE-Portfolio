<script lang="ts">
	export let name: string;
	export let icon: string;
</script>

<div class="tech-container">
	<img
		loading="lazy"
		src={icon}
		alt={name}
		class="tech-icon aspect-square w-8 object-contain sm:w-[42px]"
	/>
	<span class="tech-tooltip">{name}</span>
</div>

<style>
	.tech-container {
		position: relative;
		display: inline-block;
	}
	.tech-icon {
		transition:
			transform 0.1s ease-in-out,
			filter 0.1s ease-in-out;
	}
	.tech-container:hover .tech-icon {
		transform: translateY(-5px);
		scale: 1.1;
		filter: brightness(1.2);
	}
	.tech-tooltip {
		position: absolute;
		bottom: -25px;
		left: 50%;
		transform: translateX(-50%);
		background-color: #171717;
		color: white;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		white-space: nowrap;
		opacity: 0;
		transition: opacity 0.2s ease-in-out;
		pointer-events: none;
	}
	.tech-container:hover .tech-tooltip {
		opacity: 1;
	}
	@media (max-width: 640px) {
		.tech-container:hover .tech-icon {
			transform: translateY(-3px);
		}
	}
</style>
