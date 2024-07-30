import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import svg from '@poppanator/sveltekit-svg';

export default defineConfig({
	plugins: [sveltekit(), svg({
	
	})],
	resolve: {
		alias: {
			// "$icon": '@ajwdmedia/svelterial-icons-light/SharpFilled'
		}
	}
});