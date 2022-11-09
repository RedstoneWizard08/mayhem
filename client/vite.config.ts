// Vite
import { defineConfig } from "vite";

// Plugins
import preact from "@preact/preset-vite";
import { VitePWA as pwa } from "vite-plugin-pwa";

export default defineConfig({
    plugins: [
        preact(),
        pwa(),
    ],

    server: {
        host: "0.0.0.0",
        port: 4000,
        strictPort: true,

        hmr: {
            clientPort: 4000,
            port: 4000,
            protocol: "ws",
            host: "0.0.0.0",
        },
    },
});
