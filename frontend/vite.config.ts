import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const env = (globalThis as typeof globalThis & {
	process?: { env?: Record<string, string | undefined> };
}).process?.env;
const API_PROXY_TARGET = env?.VITE_API_PROXY ?? 'http://127.0.0.1:8080';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: API_PROXY_TARGET,
				changeOrigin: true,
				secure: false
			}
		}
	}
});
