<script lang="ts">
	import ExperienceItem from '../components/ExperienceItem.svelte';
	import ExperienceSection from '../components/ExperienceSection.svelte';

	const experienceItems = [
		{
			companyLogo: 'geost_icon.svg',
			companyName: 'GEOST',
			position: 'Mission Systems Software Engineer',
			dateRange: 'April 2023 - August 2024',
			description:
				'I developed satellite payload test software for controlling a camera payload in orbit as well as its corresponding ground interface.'
		},
		{
			companyLogo: 'raytheon_icon.svg',
			companyName: 'Raytheon Technologies',
			position: 'Software Engineer',
			dateRange: 'August 2022 - April 2023',
			description: '(Subcontracted by ISYS Technologies) I maintained software via'
		}
	];

	const languageExperiences = [
		{
			title: 'Systems & Embedded',
			items: [
				{
					name: 'C',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/c/c-original.svg'
				},
				{
					name: 'C++',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/cplusplus/cplusplus-original.svg'
				},
				{
					name: 'Rust',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/rust/rust-original.svg'
				}
			]
		},
		{
			title: 'Frontend',
			items: [
				{
					name: 'TypeScript',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/typescript/typescript-original.svg'
				},
				{
					name: 'React',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/react/react-original.svg'
				},
				{
					name: 'Svelte',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/svelte/svelte-original.svg'
				}
			]
		},
		{
			title: 'Backend',
			items: [
				{
					name: 'Python',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/python/python-original.svg'
				},
				{
					name: 'Rust',
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/rust/rust-original.svg'
				}
			]
		}
	];
</script>

<section id="experience" class="flex flex-col items-center">
	<div class=" flex w-full flex-col items-center justify-center bg-[#1C1C1C]">
		<div class="w-full max-w-6xl">
			<ExperienceSection categories={languageExperiences} />
			<div class="flex flex-col items-center space-y-6 sm:mt-12">
				{#each experienceItems as item}
					<ExperienceItem {...item} />
				{/each}
			</div>
		</div>
	</div>
</section>
