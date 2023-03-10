import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit'

const config: UserConfig = {
	plugins: [sveltekit(),
	SvelteKitPWA({
		includeAssets: ['favicon.png', 'apple-touch-icon.png', 'masked-icon.svg'],
		manifest: {
			name: 'The Aesthetics of Source Code',
			short_name: 'CodeAesthetics',
			description: 'A research work on the aesthetics of source codes.',
			theme_color: '#fafafa',
			icons: [
				{
					src: 'pwa-192x192.png',
					sizes: '192x192',
					type: 'image/png'
				},
				{
					src: 'pwa-512x512.png',
					sizes: '512x512',
					type: 'image/png'
				}
			]
		}
	})],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'jsdom'
	}
};

export default config;
