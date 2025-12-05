import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()],
    server: {
        proxy: {
            '/kernel': {
                target: 'http://localhost:2022',
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/kernel/, '')
            }
        }
    }
});
