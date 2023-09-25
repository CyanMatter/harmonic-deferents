import { defineConfig } from 'astro/config';
import { ViteRsw } from 'vite-plugin-rsw';

// https://astro.build/config
export default defineConfig({
	srcDir: './astro/src',

	publicDir: './astro/public',

	outDir: './astro/dist',

	site: 'https://cyanmatter.github.io/',

  base: '/harmonic-deferents',

  vite: {
    plugins: [ViteRsw()],
  },
});
