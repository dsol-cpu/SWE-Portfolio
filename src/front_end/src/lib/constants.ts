export const ENV_CONFIG = {
	BACKEND_URL:
		import.meta.env.VITE_MODE === 'production'
			? import.meta.env.VITE_BACKEND_URL
			: 'http://localhost:10000',
	BACKEND_ROUTE: import.meta.env.VITE_BACKEND_ROUTE,
	GITHUB_API_ROUTE: import.meta.env.VITE_GITHUB_API_ROUTE,
	PAGE_VISIT_ROUTE_PATH: import.meta.env.VITE_PAGE_VISIT_ROUTE_PATH
} as const;

export const STATUS = {
	PROJECT: {
		COMPLETED: 'Completed',
		IN_PROGRESS: 'In Progress',
		MAINTAINED: 'Maintained'
	},
	DEPLOYMENT: {
		LIVE: 'Live',
		DEMO: 'Demo Available',
		LOCAL: 'Local Only'
	}
} as const;

const BASE_DEVICON_URL = 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons';
const BASE_YESICON_URL = 'https://api.iconify.design';

export const TECH_ICONS = {
	C: `${BASE_DEVICON_URL}/c/c-original.svg`,
	'C#': `${BASE_DEVICON_URL}/csharp/csharp-original.svg`,
	'C++': `${BASE_DEVICON_URL}/cplusplus/cplusplus-original.svg`,
	Rust: 'rust_icon.svg',
	Typescript: `${BASE_DEVICON_URL}/typescript/typescript-original.svg`,
	React: `${BASE_DEVICON_URL}/react/react-original.svg`,
	Svelte: `${BASE_DEVICON_URL}/svelte/svelte-original.svg`,
	Python: `${BASE_DEVICON_URL}/python/python-original.svg`,
	Java: `${BASE_DEVICON_URL}/java/java-original.svg`,
	Docker: `${BASE_DEVICON_URL}/docker/docker-original.svg`,
	AWS: `${BASE_DEVICON_URL}/amazonwebservices/amazonwebservices-original-wordmark.svg`,
	Ansible: `${BASE_DEVICON_URL}/ansible/ansible-original.svg`,
	Bash: 'bash_icon.svg',
	Bitbucket: `${BASE_DEVICON_URL}/bitbucket/bitbucket-original.svg`,
	XML: `${BASE_DEVICON_URL}/xml/xml-original.svg`,
	Eclipse: `${BASE_DEVICON_URL}/eclipse/eclipse-original.svg`,
	Git: `${BASE_DEVICON_URL}/git/git-original.svg`,
	Centos: `${BASE_DEVICON_URL}/centos/centos-original.svg`,
	RHEL: 'rhel_icon.svg',
	Actix: `${BASE_YESICON_URL}/simple-icons:actix.svg`,
	Blender: `${BASE_DEVICON_URL}/blender/blender-original.svg`,
	PostgreSQL: `${BASE_DEVICON_URL}/postgresql/postgresql-original.svg`,
	'Github Pages': 'github_icon.svg',
	Redis: `${BASE_DEVICON_URL}/redis/redis-original.svg`,
	MongoDB: `${BASE_DEVICON_URL}/mongodb/mongodb-original-wordmark.svg`,
	JSON: `${BASE_DEVICON_URL}/json/json-original.svg`,
	Unity: `${BASE_DEVICON_URL}/unity/unity-original.svg`,
	Perl: `${BASE_DEVICON_URL}/perl/perl-original.svg`
} as const;
