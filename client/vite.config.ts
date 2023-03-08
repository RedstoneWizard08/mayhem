// Vite
import { defineConfig } from "vite";

// Plugins
import { sveltekit } from "@sveltejs/kit/vite";
import { SvelteKitPWA } from "@vite-pwa/sveltekit";

export default defineConfig({
    plugins: [sveltekit(), SvelteKitPWA()],

    server: {
        port: 4000,
        strictPort: true,

        hmr: {
            clientPort: 4000,
            port: 4000,
            protocol: "ws",
        },
    },
});
