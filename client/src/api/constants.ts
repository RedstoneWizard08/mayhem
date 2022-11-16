import { AxiosRequestConfig } from "axios";

export interface APIRoute {
    /**
     * The path of the route. Example: "/servers"
     */
    path: string;

    /**
     * The method to use for the API route.
     *
     * - GET: Mainly used for retrieving/viewing information.
     * - POST: Mainly used for updating/changing information.
     * - PUT: Mainly used for creating/adding information.
     * - DELETE: Mainly used for removing/deleting information.
     */
    method: "GET" | "POST" | "PUT" | "DELETE";

    /**
     * Whether or not the route requires an API key or not.
     *
     * __NOTICE!__
     * This is not a token! Tokens are not API keys!
     * API keys are used for OAuth (authentication)
     *    or general non-user operations.
     * Tokens are used to access information as a user
     *    or a bot.
     *
     * @default false
     */
    requiresKey?: boolean;

    /**
     * Whether or not the route requires a token or not.
     *
     * __NOTICE!__
     * This is not an API key! Tokens are not API keys!
     * API keys are used for OAuth (authentication)
     *    or general non-user operations.
     * Tokens are used to access information as a user
     *    or a bot.
     *
     * @default false
     */
    requiresToken?: boolean;
};

export interface APIRouteList {
    [key: string]: APIRouteListInner;
};

export interface APIRouteListInner {
    [key: string]: APIRoute;
};

export const API_BASE: string = import.meta.env.VITE_API_BASE || "http://dev.local.host:4001/api/v1";

export const _ROUTES = {
    servers: {
        list: {
            path: "/servers",
            method: "GET",

            requiresToken: true,
        },

        info: {
            path: "/servers/{id}",
            method: "GET",

            requiresToken: true,
        },

        delete: {
            path: "/servers/{id}",
            method: "DELETE",

            requiresToken: true,
        },

        publicInfo: {
            path: "/servers/{id}/public_info",
            method: "GET",

            requiresToken: true,
        },

        updatePublicInfo: {
            path: "/servers/{id}/public_info",
            method: "POST",

            requiresToken: true,
        },

        updateInfo: {
            path: "/servers/{id}",
            method: "POST",

            requiresToken: true,
        },
        
        members: {
            path: "/servers/{id}/members",
            method: "GET",

            requiresToken: true,
        },

        join: {
            path: "/servers/{id}/members",
            method: "PUT",

            requiresToken: true,
        },

        leave: {
            path: "/servers/{id}/members",
            method: "DELETE",

            requiresToken: true,
        },
        
        roles: {
            path: "/servers/{id}/roles",
            method: "GET",

            requiresToken: true,
        },

        createRole: {
            path: "/servers/{id}/roles",
            method: "PUT",

            requiresToken: true,
        },

        updateRole: {
            path: "/servers/{id}/roles",
            method: "POST",

            requiresToken: true,
        },

        deleteRole: {
            path: "/servers/{id}/roles",
            method: "DELETE",

            requiresToken: true,
        },

        channels: {
            path: "/servers/{id}/channels",
            method: "GET",

            requiresToken: true,
        },

        createChannel: {
            path: "/servers/{id}/channel",
            method: "PUT",

            requiresToken: true,
        },

        updateChannel: {
            path: "/servers/{id}/channel",
            method: "POST",

            requiresToken: true,
        },

        deleteChannel: {
            path: "/servers/{id}/channel",
            method: "DELETE",

            requiresToken: true,
        },

        listPublic: {
            path: "/servers/public",
            method: "GET",
        },
    },
};

export const ROUTES = _ROUTES as APIRouteList;

export const buildRequest = (request: APIRoute, token?: string) => {
    let config: AxiosRequestConfig = {
        url: request.path,
        baseURL: API_BASE,

        method: request.method,
    };

    if (request.requiresKey) {
        if (!config.headers) config.headers = {};

        config.headers.Authorization = "Basic " + token;
    }

    if (request.requiresToken) {
        if (!config.headers) config.headers = {};

        config.headers.Authorization = "Bearer " + token;
    }

    return config;
};
