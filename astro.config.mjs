import { defineConfig } from 'astro/config';
import { ViteRsw } from 'vite-plugin-rsw';

// https://astro.build/config
export default defineConfig({
	srcDir: './astro/src',

	publicDir: './astro/public',

	outDir: './astro/dist',

	site: 'http://127.0.0.1:3000/',

  base: '/harmonic-deferents',

  vite: {
    plugins: [ViteRsw()],
  },
});
