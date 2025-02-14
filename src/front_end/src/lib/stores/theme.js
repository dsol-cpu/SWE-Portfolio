import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const createThemeStore = () => {
	const defaultTheme = browser ? localStorage.getItem('theme') || 'light' : 'light';

	const store = writable(defaultTheme);

	const themeStore = {
		subscribe: store.subscribe,
		toggleTheme: () => {
			if (browser) {
				const currentTheme = localStorage.getItem('theme') || 'light';
				const newTheme = currentTheme === 'light' ? 'dark' : 'light';
				localStorage.setItem('theme', newTheme);
				document.documentElement.classList.toggle('dark');
				store.set(newTheme);
			}
		},
		setTheme: (theme) => {
			if (browser) {
				localStorage.setItem('theme', theme);
				document.documentElement.classList.toggle('dark', theme === 'dark');
				store.set(theme);
			}
		}
	};

	return themeStore;
};

export const theme = createThemeStore();
