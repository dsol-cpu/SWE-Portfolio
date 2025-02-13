export type Technology = {
	name: string;
	icon: string;
};

export type ExperienceItem = {
	companyLogo: string;
	companyName: string;
	logoStyle?: string;
	position: string;
	dateRange: string;
	description: string;
	technologies: Technology[];
};

export type LanguageCategory = {
	title: string;
	items: Technology[];
};

export type GitHubInfo = {
	name: string;
	last_updated_at: string;
};

export type Project = {
	title: string;
	imageUrl: string;
	imageAlt: string;
	arrowIconUrl: string;
	githubUrl: string;
	description: string;
	projectStatus: string;
	deploymentStatus: string;
	keyFeatures: string[];
	technologies: Technology[];
	youtubeUrl?: string;
	demoUrl?: string;
	last_updated_at?: string;
};
