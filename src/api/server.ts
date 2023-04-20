import { page } from "$app/stores";
import axios from "axios";
import { get } from "svelte/store";
import { token } from "../stores/current";

export interface Server {
    id: number;
    name: string;
}

export interface ServerResponse {
    user_id: number;
    servers: Server[];
}

export const getServers = async (): Promise<ServerResponse> => {
    const url = new URL("/api/v1/servers", get(page).url.href);

    const result = await axios.get<ServerResponse>(url.toString(), {
        headers: {
            Authorization: "Bearer " + get(token),
        },
    });

    return result.data;
};
