<script lang="ts">
	import ExperienceSection from './ExperienceSection.svelte';
	import SocialIcon from './SocialIcon.svelte';
	import EmailContact from './EmailContact.svelte';

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

	const socialLinks = [
		{
			href: 'https://github.com/dsol-cpu',
			icon: 'github_icon.svg',
			label: 'Github Profile',
			size: '30px'
		},
		{
			href: '#',
			icon: 'upwork_icon.svg',
			label: 'Upwork Profile',
			size: '30px'
		},
		{
			href: '#',
			icon: 'fiverr_icon.svg',
			label: 'Fiverr Profile',
			size: '30px'
		},
		{
			href: 'https://www.instagram.com/smep_itty/',
			icon: 'instagram_icon.svg',
			label: 'Instagram Profile',
			size: '30px'
		}
	];

	const emailData = {
		email: 'dsolinsky98@gmail.com',
		icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/a1164e8c39919f9a28435c6fa8d6973ddf84a873d3f806476e64af1f343553af'
	};
</script>

<section class="bg-[#1C1C1C] px-4 py-20 sm:px-8 lg:px-12 xl:px-16">
	<div class="mx-auto flex max-w-7xl items-center gap-12">
		<img
			loading="lazy"
			src="me.png"
			alt="Profile Avatar"
			class="h-[251px] max-w-full object-contain sm:h-[301px]"
		/>
		<div class="flex flex-col items-start">
			<h1 class="text-2xl font-semibold tracking-[2.4px] text-stone-300 sm:text-3xl">
				David Solinsky
			</h1>
			<p class="mt-4 max-w-[600px] text-base font-light tracking-wide text-stone-300 sm:text-lg">
				Full-stack and Embedded software engineer.<br />
			</p>
			<!-- Contact information -->
			<div class="mt-6">
				<EmailContact {...emailData} />
				<nav class="mt-4 flex items-center gap-7">
					{#each socialLinks as link}
						<SocialIcon {...link} />
					{/each}
				</nav>
			</div>
			<div class="mt-10 flex w-full max-w-xl flex-col justify-start gap-4 sm:flex-row sm:gap-6">
				<a href="David_Solinsky_resume.pdf" target="_blank">
					<button
						class="button-hover outline-button rounded-[50px] border-2 border-solid border-white px-6 py-3 text-white transition-colors duration-300 hover:bg-white hover:text-neutral-900 sm:px-7 sm:py-4"
					>
						Download Resume
					</button>
				</a>
			</div>
		</div>
	</div>
</section>

<style>
	.button-hover {
		transition: all 0.3s ease-in-out;
	}
	.white-button:hover {
		background-color: transparent;
		color: white;
	}
	.outline-button:hover {
		background-color: white;
		color: #171717;
	}
</style>
