import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    
    compilerOptions: {
        format: "cjs",
        enableSourcemap: true,
    },

    kit: {
        adapter: adapter({
            polyfill: true,
            precompress: true,
        }),
    },
};

export default config;
