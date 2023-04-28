import { defineManifest } from "@crxjs/vite-plugin";

export default defineManifest({
    name: "mayhem-education-chrome",
    description: "Mayhem's education module for Chrome and Chromium-based browsers.",
    version: "0.1.0",
    manifest_version: 3,
    icons: {
        "16": "img/logo-16.png",
        "32": "img/logo-34.png",
        "48": "img/logo-48.png",
        "128": "img/logo-128.png",
    },
    background: {
        service_worker: "src/background/index.ts",
        type: "module",
    },
    web_accessible_resources: [
        {
            resources: [
                "img/logo-16.png",
                "img/logo-34.png",
                "img/logo-48.png",
                "img/logo-128.png",
            ],
            matches: [],
        },
    ],
    permissions: [],
});
