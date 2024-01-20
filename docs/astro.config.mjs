import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	integrations: [
		starlight({
			title: 'OOXML Tools',
			social: {
				github: 'https://github.com/scuderia-fe/ooxml-tools',
			},
      logo: {
        src: './src/assets/logo.webp',
        replacesTitle: false,
      },
      favicon: '/favicon.ico',
      description: 'OOXML Tools is a collection of tools for working with OOXML files.',
      defaultLocale: 'en',
		}),
	],
});
