// Vite
import { defineConfig } from "vite";

// Plugins
import { sveltekit } from "@sveltejs/kit/vite";
import { SvelteKitPWA } from "@vite-pwa/sveltekit";

export default defineConfig({
    plugins: [sveltekit(), SvelteKitPWA()],
    clearScreen: false,

    server: {
        port: 4001,
        strictPort: true,
        cors: true,

        hmr: {
            clientPort: 443,
            port: 4001,
            protocol: "wss",
            path: "/vite-hmr",
        },
    },
});
