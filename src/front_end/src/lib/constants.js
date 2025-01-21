export const BACKEND_URL =
	import.meta.env.VITE_MODE === 'production'
		? import.meta.env.VITE_BACKEND_URL
		: 'http://localhost:10000';
export const BACKEND_ROUTE = import.meta.env.VITE_BACKEND_ROUTE;
export const GITHUB_API_ROUTE = import.meta.env.VITE_GITHUB_API_ROUTE;
export const PAGE_VISIT_ROUTE_PATH = import.meta.env.VITE_PAGE_VISIT_ROUTE_PATH;

export const PROJECT_STATUS = {
	COMPLETED: 'Completed',
	IN_PROGRESS: 'In Progress',
	MAINTAINED: 'Maintained'
};

export const DEPLOYMENT_STATUS = {
	LIVE: 'Live',
	DEMO: 'Demo Available',
	LOCAL: 'Local Only'
};
