<script lang="ts">
	import SocialIcon from '../components/SocialIcon.svelte';
	import EmailContact from '../components/EmailContact.svelte';

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
			href: 'https://www.fiverr.com/dsol_cpu?up_rollout=true',
			icon: 'fiverr_icon.svg',
			label: 'Fiverr Profile',
			size: '30px'
		},
		{
			href: 'https://www.linkedin.com/in/david-solinsky',
			icon: 'linkedin_icon.svg',
			label: 'Fiverr Profile',
			size: '30px'
		}
	];

	const emailData = {
		email: 'dsolinsky98@gmail.com',
		icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/a1164e8c39919f9a28435c6fa8d6973ddf84a873d3f806476e64af1f343553af'
	};
</script>

<section id="home" class="flex min-h-screen items-center px-4 sm:px-8 lg:px-12 xl:px-16">
	<div class="mx-auto w-full max-w-7xl py-12 sm:py-16 lg:py-20">
		<div class="flex flex-col items-center gap-8 lg:flex-row lg:items-center lg:gap-12">
			<div class="w-full max-w-[300px] flex-shrink-0 lg:w-auto">
				<img
					loading="lazy"
					src="me.png"
					alt="Profile Avatar"
					class="mx-auto h-[200px] w-[200px] rounded-full object-cover sm:h-[250px] sm:w-[250px] lg:h-[300px] lg:w-[300px]"
					style="border-radius: 50%;"
				/>
			</div>
			<div
				class="flex w-full flex-shrink flex-col items-center text-center lg:items-start lg:text-left"
			>
				<h1 class="text-2xl font-semibold tracking-[2.4px] text-stone-300 sm:text-3xl lg:text-4xl">
					David Solinsky
				</h1>
				<p class="mt-4 max-w-[600px] text-base font-light tracking-wide text-stone-300 sm:text-lg">
					Full-stack and Embedded software engineer.<br />
				</p>
				<!-- Updated contact information container -->
				<div class="mt-6 flex w-full flex-col items-center lg:items-start">
					<!-- Email contact with centered container -->
					<div class="flex w-full justify-center lg:justify-start">
						<div class="w-full max-w-md">
							<EmailContact {...emailData} />
						</div>
					</div>
					<!-- Social icons with centered container -->
					<nav class="mt-4 flex w-full justify-center gap-7 lg:justify-start">
						{#each socialLinks as link}
							<SocialIcon {...link} />
						{/each}
					</nav>
				</div>
				<div class="mt-8 w-full sm:mt-10">
					<a href="David_Solinsky_resume.pdf" target="_blank" class="inline-block w-full sm:w-auto">
						<button
							class="outline-button w-full border-2 border-solid border-white px-6 py-3 text-white transition-colors duration-300 sm:px-7 sm:py-4"
						>
							Download Resume
						</button>
					</a>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.outline-button {
		border-radius: 50px;
	}

	.outline-button:hover {
		background-color: white;
		color: #171717;
	}
</style>
