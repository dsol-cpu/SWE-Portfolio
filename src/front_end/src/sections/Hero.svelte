<script lang="ts">
	import SocialIcon from '../components/SocialIcon.svelte';
	import EmailContact from '../components/EmailContact.svelte';
	import ResumeButton from '../components/ResumeButton.svelte';
	const socialLinks = [
		{
			href: 'https://github.com/dsol-cpu',
			icon: 'github_icon.svg',
			label: 'Github Profile',
			size: '30px'
		},
		{
			href: 'https://www.upwork.com/freelancers/~0179a1a2a2df5212fd',
			icon: 'upwork_icon.svg',
			label: 'Upwork Profile',
			size: '30px'
		},
		{
			href: 'https://www.linkedin.com/in/david-solinsky',
			icon: 'linkedin_icon.svg',
			label: 'Linkedin Profile',
			size: '30px'
		}
	];

	const emailData = {
		email: 'solinsky.david@gmail.com',
		icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/a1164e8c39919f9a28435c6fa8d6973ddf84a873d3f806476e64af1f343553af'
	};
</script>

<section id="home" class="styled-section flex items-center px-4 sm:px-8 lg:px-12 xl:px-16">
	<div class="mx-auto w-full max-w-7xl py-12 sm:py-16 lg:py-20">
		<div class="flex flex-col items-center gap-8 lg:flex-row lg:items-center lg:gap-12">
			<div class="w-full max-w-[300px] flex-shrink-0 lg:w-auto">
				<img
					loading="lazy"
					src="me.png"
					alt="Profile Avatar"
					class="mx-auto h-[200px] w-[200px] rounded-full object-cover sm:h-[250px] sm:w-[250px] lg:h-[300px] lg:w-[300px]"
				/>
			</div>
			<div
				class="flex w-full flex-shrink flex-col items-center text-center lg:items-start lg:text-left"
			>
				<h1 class="text-2xl font-semibold tracking-[2.4px] text-stone-300 sm:text-3xl lg:text-4xl">
					David Solinsky
				</h1>
				<p class="mt-4 max-w-[600px] text-base font-light tracking-wide text-stone-300 sm:text-lg">
					Software Engineer and Game Developer.<br />
				</p>
				<div class="mt-6 flex w-full flex-col items-center lg:items-start">
					<div class="flex w-full justify-center lg:justify-start">
						<div class="w-full max-w-md">
							<EmailContact {...emailData} />
						</div>
					</div>
					<nav class="mt-4 flex w-full justify-center gap-7 lg:justify-start">
						{#each socialLinks as link}
							<SocialIcon {...link} />
						{/each}
					</nav>
				</div>
				<div class="mt-8 w-full sm:mt-10">
					<ResumeButton />
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	@import '../lib/styles/styles.css';
</style>
