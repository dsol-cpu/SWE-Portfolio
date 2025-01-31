import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [sveltekit()],

	server: {
		proxy: {
			'/api': {
				changeOrigin: true, // Ensures correct headers are sent to the target
				rewrite: (path) => path.replace(/^\/api/, '') // Optional: rewrite path if needed
			}
		}
	},

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'jsdom', // Use jsdom for browser-like environment
		globals: true, // Allow global variables in tests
		setupFiles: './src/test/setup.js', // Optional: Setup file for tests
		coverage: {
			reporter: ['text', 'html'], // Report coverage to console and generate HTML report
			exclude: ['node_modules/*', 'src/**/*.{test,spec}.{js,ts}'] // Exclude node_modules and test files from coverage
		}
	},

	// Optimize build for production
	build: {
		sourcemap: process.env.NODE_ENV === 'production' ? false : true, // Disable in production
		minify: 'terser', // Use terser for efficient minification
		target: 'es2020', // Target modern browsers for smaller bundle size
		rollupOptions: {
			output: {
				chunkFileNames: 'assets/[name]-[hash].js' // Use hash for cache busting
			}
		},
		outDir: 'src/front_end/build' // Specify the output directory
	},

	// Optimize for development
	preview: {
		port: 3000, // Specify port for preview server
		host: 'localhost' // Host for preview server
	}
});
