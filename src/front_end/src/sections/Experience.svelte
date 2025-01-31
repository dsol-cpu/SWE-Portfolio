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
				"I developed satellite payload test software with NASA's Core Flight System (cFS) framework for controlling a camera payload in orbit as well as a ground interface to interface with and command it. Split into multiple dockerized microservices to process different Space Packet Protocol(SPP) data packet types (Commands, Command Repsonses, Telemetry).",
			technologies: [
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/c/c-original.svg',
					name: 'C'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/python/python-original.svg',
					name: 'Python'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/typescript/typescript-original.svg',
					name: 'TypeScript'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/docker/docker-original.svg',
					name: 'Docker'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/amazonwebservices/amazonwebservices-original-wordmark.svg',
					name: 'AWS'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/bitbucket/bitbucket-original.svg',
					name: 'BitBucket'
				}
			]
		},
		{
			companyLogo: 'raytheon_icon.svg',
			companyName: 'Raytheon Technologies',
			position: 'Software Engineer',
			dateRange: 'August 2022 - April 2023',
			description:
				'I maintained the weather hazards alert system (AWIPS2) software in communication with National Weather Service (NWS) offices and meteorologists in the area.',
			technologies: [
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/java/java-original.svg',
					name: 'Java'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/python/python-original.svg',
					name: 'Python'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/xml/xml-original.svg',
					name: 'XML'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/eclipse/eclipse-original.svg',
					name: 'Eclipse'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/git/git-original.svg',
					name: 'Git'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/centos/centos-original.svg',
					name: 'CentOS'
				},
				{
					icon: 'rhel_icon.svg',
					name: 'RHEL'
				}
			]
		}
	];

	const languageExperiences = [
		{
			title: 'Systems & Embedded',
			items: [
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/c/c-original.svg',
					name: 'C'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/cplusplus/cplusplus-original.svg',
					name: 'C++'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/rust/rust-original.svg',
					name: 'Rust'
				}
			]
		},
		{
			title: 'Frontend',
			items: [
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/typescript/typescript-original.svg',
					name: 'TypeScript'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/react/react-original.svg',
					name: 'React'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/svelte/svelte-original.svg',
					name: 'Svelte'
				}
			]
		},
		{
			title: 'Backend',
			items: [
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/python/python-original.svg',
					name: 'Python'
				},
				{
					icon: 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/rust/rust-original.svg',
					name: 'Rust'
				}
			]
		}
	];
</script>

<section id="experience" class="styled-section flex flex-col items-center">
	<div class="mt-7 flex w-full flex-col justify-center">
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

<style>
	@import '../lib/styles/styles.css';
</style>
