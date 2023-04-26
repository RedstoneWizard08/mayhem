import { dev } from "$app/environment";
import type { Handle } from "@sveltejs/kit";

const getEruda = () => {
    if (dev)
        return `
            <script src="https://cdn.jsdelivr.net/npm/eruda"></script>
            <script>
                eruda.init();
            </script>
        `;

    return "";
};

export const handle: Handle = async ({ event, resolve }) => {
    const resp = await resolve(event, {
        transformPageChunk: ({ html }) => html.replace("%eruda%", getEruda()),
    });

    return resp;
};
