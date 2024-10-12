import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [sveltekit(), wasmPack('./wasm-lib')],
	server: {
        fs: {
            allow: [
                // Allow serving files from the wasm-lib/pkg directory
                './wasm-lib/pkg'
            ]
        }
    }
});
