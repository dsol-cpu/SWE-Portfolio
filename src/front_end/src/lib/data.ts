import { TECH_ICONS } from './constants';
import type { Technology, ExperienceItem, LanguageCategory } from './types';

const createTech = (name: string, iconKey: keyof typeof TECH_ICONS): Technology => ({
	name: name,
	icon: TECH_ICONS[iconKey]
});

const EXPERIENCE_DATA = {
	SWIFT: {
		technologies: ['C++', 'Bash', 'Perl', 'XML', 'Ansible', 'Bitbucket'] as const,
		description: 'Contributing to the development of a new financial messaging format.'
	},
	GEOST: {
		technologies: ['C', 'Python', 'Typescript', 'Docker', 'AWS', 'Bitbucket'] as const,
		description:
			"I developed satellite payload test software with NASA's Core Flight System (cFS) framework for controlling a camera payload in orbit as well as a ground interface to interface with and command it. Split into multiple dockerized microservices to process different Space Packet Protocol (SPP) data packet types (Commands, Command Responses, Telemetry)."
	},
	RAYTHEON: {
		technologies: ['Java', 'Python', 'XML', 'Eclipse', 'Git', 'Centos', 'RHEL'] as const,
		description:
			'I maintained the weather hazards alert system (AWIPS2) software in communication with National Weather Service (NWS) offices and meteorologists in the area.'
	}
} as const;

export const experienceItems: ExperienceItem[] = [
	{
		companyLogo: 'SWIFT_logo.svg',
		companyName: 'SWIFT',
		position: 'Associate Developer/Systems Engineer',
		dateRange: 'January 2025 - Present',
		description: EXPERIENCE_DATA.SWIFT.description,
		technologies: EXPERIENCE_DATA.SWIFT.technologies.map((tech) =>
			createTech(tech === 'Bash' ? 'Kornshell' : tech, tech)
		)
	},
	{
		companyLogo: 'geost_icon.svg',
		companyName: 'GEOST',
		position: 'Mission Systems Software Engineer',
		dateRange: 'April 2023 - August 2024',
		description: EXPERIENCE_DATA.GEOST.description,
		technologies: EXPERIENCE_DATA.GEOST.technologies.map((tech) => createTech(tech, tech))
	},
	{
		companyLogo: 'raytheon_icon.svg',
		companyName: 'Raytheon Technologies',
		position: 'Software Engineer',
		dateRange: 'August 2022 - April 2023',
		description: EXPERIENCE_DATA.RAYTHEON.description,
		technologies: EXPERIENCE_DATA.RAYTHEON.technologies.map((tech) => createTech(tech, tech))
	}
];

const CATEGORY_DATA = {
	SYSTEMS: ['C', 'C++', 'Rust'],
	FRONTEND: ['Typescript', 'React', 'Svelte'],
	BACKEND: ['Python', 'Rust']
} as const;

export const languageCategories: LanguageCategory[] = [
	{
		title: 'Systems & Embedded',
		items: CATEGORY_DATA.SYSTEMS.map((tech) => createTech(tech, tech))
	},
	{
		title: 'Frontend',
		items: CATEGORY_DATA.FRONTEND.map((tech) => createTech(tech, tech))
	},
	{
		title: 'Backend',
		items: CATEGORY_DATA.BACKEND.map((tech) => createTech(tech, tech))
	}
];

const PROJECTS = {
	PORTFOLIO: {
		title: 'Portfolio Website',
		imageUrl: 'portfolio_website_icon.svg',
		imageAlt: 'Portfolio Website',
		arrowIconUrl:
			'https://cdn.builder.io/api/v1/image/assets/TEMP/836107239b15f597b011636150c87b1fc753035a1ed4748bf7f038fd561d8903',
		githubUrl: 'https://github.com/dsol-cpu/SWE-Portfolio',
		description:
			"You're here! I wanted to better organize my portfolio website from the one I did in ReactJS a couple years ago, so here we are!",
		projectStatus: 'In Progress',
		deploymentStatus: 'Live',
		keyFeatures: ['Front-end', 'Back-end', 'RESTful API', 'Serverless'],
		technologies: ['Svelte', 'Rust', 'Actix', 'Github Pages', 'Redis', 'MongoDB'],
		demoUrl: 'https://dsol-cpu.github.io/SWE-Portfolio'
	},
	GEOSPATIAL: {
		title: 'Geospatial Mapping Challenge',
		imageUrl: 'https://img.youtube.com/vi/coCoAvOaSBM/0.jpg',
		imageAlt: '',
		arrowIconUrl:
			'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
		githubUrl: 'https://github.com/dsol-cpu/GeospatialDataVisualization-Aug2024',
		description:
			'Parses a json file for geospatial data and uses a regex to deal with dirty/corrupted/invalid data. That data is correlated with each pin instantiated on the globe, allowing the user to mouse hover and display the corresponding location data.',
		projectStatus: 'Completed',
		deploymentStatus: 'Live',
		keyFeatures: ['Data Visualization', 'Game Development'],
		technologies: ['Unity', 'C#', 'JSON', 'Blender'],
		youtubeUrl: 'https://youtu.be/coCoAvOaSBM',
		demoUrl: 'https://dsol-cpu.github.io/GeospatialDataVisualization-Aug2024/'
	},
	UFO: {
		title: 'UFO Sightings Challenge',
		imageUrl: 'alien_search_icon.svg',
		imageAlt: '',
		arrowIconUrl:
			'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
		githubUrl: 'https://github.com/dsol-cpu/alien-search',
		description:
			'This was a challenge to gameify ufo sightings data. Extracts a number of random pieces of data and displays on UI near screens. Intended behavior was that all screens would amount to a password using the data and the ship would take that password, ending the game.',
		projectStatus: 'Completed',
		deploymentStatus: 'Live',
		keyFeatures: ['Data Visualization', 'Game Development'],
		technologies: ['Unity', 'C#', 'JSON', 'Blender']
	},
	BLENDER_ADDON: {
		title: 'Moving Terrain Editor Addon (Mario Kart)',
		imageUrl: 'https://img.youtube.com/vi/QDt2I_BZKrI/0.jpg',
		imageAlt: 'Mario Kart Blender Addon',
		arrowIconUrl:
			'https://cdn.builder.io/api/v1/image/assets/TEMP/887e7f1dfa4b2c0692cecd3fa55a4372355bafc71ae7d9ba75414ed4744a4ad4',
		githubUrl: 'https://github.com/dsol-cpu/MK-AREAS-plugin',
		description:
			"Allows the user to import .kmp files and represent data as cube primitives, manipulate primitives' transform in Blender, and export transforms to manipulate their corresponding moving terrain (Conveyor belts, water current surfaces)",
		projectStatus: 'Completed',
		deploymentStatus: 'Local Only',
		keyFeatures: ['Data Visualization', 'Blender Addon'],
		technologies: ['Blender', 'Python'],
		youtubeUrl: 'https://www.youtube.com/watch?v=QDt2I_BZKrI'
	},
	MMORPG: {
		title: 'MMORPG Database & Interface',
		imageUrl: 'db_icon.svg',
		imageAlt: 'MMORPG Database and Interface',
		arrowIconUrl: '',
		githubUrl: 'https://github.com/dsol-cpu/MMORG-DB',
		description:
			'A small python CLI interface made for a mock MMORPG relational database postgreSQL',
		projectStatus: 'Completed',
		deploymentStatus: 'Local Only',
		keyFeatures: ['CLI', 'Database'],
		technologies: ['Python', 'PostgreSQL']
	}
} as const;

export const projects = [
	{
		...PROJECTS.PORTFOLIO,
		technologies: PROJECTS.PORTFOLIO.technologies.map((tech) => createTech(tech, tech))
	},
	{
		...PROJECTS.GEOSPATIAL,
		technologies: PROJECTS.GEOSPATIAL.technologies.map((tech) => createTech(tech, tech))
	},
	{
		...PROJECTS.UFO,
		technologies: PROJECTS.UFO.technologies.map((tech) => createTech(tech, tech))
	},
	{
		...PROJECTS.BLENDER_ADDON,
		technologies: PROJECTS.BLENDER_ADDON.technologies.map((tech) =>
			createTech(tech === 'Blender' ? 'Blender API' : tech, tech)
		)
	},
	{
		...PROJECTS.MMORPG,
		technologies: PROJECTS.MMORPG.technologies.map((tech) => createTech(tech, tech))
	}
];
