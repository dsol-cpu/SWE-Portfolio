<script lang="ts">
	import ExperienceItem from './ExperienceItem.svelte';
	import ExperienceSection from './ExperienceSection.svelte';

	const experienceData = [
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

<ExperienceSection categories={experienceData} />
