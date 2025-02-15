import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const createThemeStore = () => {
	// Default to 'light', but check localStorage in the browser environment
	const defaultTheme = browser ? localStorage.getItem('theme') || 'light' : 'light';

	// Create store with the default theme
	const store = writable(defaultTheme);

	const themeStore = {
		subscribe: store.subscribe,
		toggleTheme: () => {
			if (browser) {
				const currentTheme = localStorage.getItem('theme') || 'light';
				const newTheme = currentTheme === 'light' ? 'dark' : 'light';
				localStorage.setItem('theme', newTheme);
				document.documentElement.classList.toggle('dark', newTheme === 'dark');
				store.set(newTheme);
			}
		},
		setTheme: (theme) => {
			if (browser) {
				localStorage.setItem('theme', theme);
				// Apply 'dark' class to the document element immediately
				document.documentElement.classList.toggle('dark', theme === 'dark');
				store.set(theme);
			}
		}
	};

	// On initial load, apply the theme class immediately based on localStorage
	if (browser) {
		const initialTheme = localStorage.getItem('theme') || 'light';
		document.documentElement.classList.toggle('dark', initialTheme === 'dark');
	}

	return themeStore;
};

export const theme = createThemeStore();
