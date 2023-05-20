import nodeAdapter from "@sveltejs/adapter-node";
import bunAdapter from "svelte-adapter-bun";

import { vitePreprocess } from "@sveltejs/kit/vite";

const adapter = process.env.ADAPTER == "bun" ? bunAdapter : nodeAdapter;

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

        files: {
            hooks: {
                server: "src/hooks.ts",
            },
        },
    },
};

export default config;
