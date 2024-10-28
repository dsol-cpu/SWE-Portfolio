<script lang="ts">
	import ExperienceSection from './ExperienceSection.svelte';

	const experienceData = [
		{
			title: 'Systems & Embedded',
			items: [
				{ name: 'C++', icon: '/api/placeholder/42/42' },
				{ name: 'C', icon: '/api/placeholder/42/42' },
				{ name: 'Rust', icon: '/api/placeholder/42/42' }
			]
		},
		{
			title: 'Frontend',
			items: [
				{ name: 'TypeScript', icon: '/api/placeholder/42/42' },
				{ name: 'React', icon: '/api/placeholder/42/42' },
				{ name: 'Svelte', icon: '/api/placeholder/42/42' }
			]
		},
		{
			title: 'Backend',
			items: [
				{ name: 'Python', icon: '/api/placeholder/42/42' },
				{ name: 'Rust Backend', icon: '/api/placeholder/42/42' }
			]
		}
	];
</script>

<section class="mx-auto flex w-full max-w-7xl flex-col items-center px-4">
	<img
		loading="lazy"
		src="me.png"
		alt="Profile Avatar"
		class="hover-scale mt-24 aspect-[1.13] w-48 object-contain max-md:mt-10 sm:w-[251px]"
	/>
	<p
		class="mt-10 max-w-[800px] px-4 text-center text-base font-light tracking-wide text-stone-300 sm:text-lg"
	>
		I am a seasoned full-stack software engineer with over 8 years of professional experience,
		specializing in backend development. My expertise lies in crafting robust and scalable
		SaaS-based architectures on the Amazon AWS platform.
	</p>
	<div
		class="mt-14 flex w-full max-w-xl flex-col justify-center gap-4 px-4 text-lg font-semibold max-md:mt-10 sm:flex-row sm:gap-5 sm:text-xl"
	>
		<button
			class="button-hover white-button rounded-[50px] border border-solid border-white bg-white px-6 py-3 text-neutral-900 sm:px-8 sm:py-4"
		>
			Get In Touch
		</button>
		<button
			class="button-hover outline-button rounded-[50px] border-2 border-solid border-white px-6 py-3 text-white sm:px-7 sm:py-4"
		>
			Download CV
		</button>
	</div>
</section>

<style>
	.hover-scale {
		transition: transform 0.3s ease-in-out;
	}
	.hover-scale:hover {
		transform: scale(1.5);
	}
	.button-hover {
		transition: all 0.3s ease-in-out;
	}
	.white-button:hover {
		background-color: transparent;
		color: white;
	}
	.outline-button:hover {
		background-color: white;
		color: #171717; /* neutral-900 */
	}
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
