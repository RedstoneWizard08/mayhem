// Vite
import { defineConfig } from "vite";

// Plugins
import { sveltekit } from "@sveltejs/kit/vite";
import { SvelteKitPWA } from "@vite-pwa/sveltekit";

export default defineConfig({
    plugins: [sveltekit(), SvelteKitPWA()],
    clearScreen: false,

    server: {
        port: 4000,
        strictPort: true,

        hmr: {
            clientPort: 443,
            port: 4000,
            protocol: "wss",
        },

        proxy: {
            "/api": {
                target: "http://localhost:4001",
                changeOrigin: true,
                ws: true,
            },
        },
    },
});
