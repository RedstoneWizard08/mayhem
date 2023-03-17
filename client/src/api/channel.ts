import { page } from "$app/stores";
import axios from "axios";
import { currentServer, servers, token } from "../stores/current";
import { get } from "svelte/store";
import type { ChatMessageProps } from "./message";
export interface ChannelType {
    id: string;
    name: string;
    type: "channel";
    channelType: "text" | "voice" | "announcement" | "events";
    server: string;
    messages: Partial<ChatMessageProps>[];
}

export interface ChannelGroupType {
    id: string;
    name: string;
    type: "group";
    server: string;
    channels: ChannelType[];
}

export const testChannelData: (ChannelType | ChannelGroupType)[] = [];

export interface ChannelR {
    id: number;
    name: string;
    server_id: number;
    channel_type: string;
}

export interface ChannelResponse {
    server_id: number;
    channels: ChannelR[];
}

export const getChannels = async (serverId: number): Promise<ChannelResponse> => {
    const url = new URL(`/api/v1/servers/${serverId}/channels`, get(page).url.href);

    const result = await axios.get<ChannelResponse>(url.toString(), {
        headers: {
            Authorization: "Bearer " + get(token),
        },
    });

    return result.data;
};

export const updateChannels = async () => {
    if (get(currentServer)) {
        const resp = await getChannels(parseInt(get(currentServer)?.id || "-1"));

        const _cs = resp.channels.map(
            (c) =>
                ({
                    id: c.id.toString(),
                    messages: [],
                    name: c.name,
                    server: c.server_id.toString(),
                    type: "channel",
                    channelType: c.channel_type,
                } as ChannelType)
        );

        get(servers).find((s) => s.id == get(currentServer)?.id)!.channels = _cs;
        get(currentServer)!.channels = _cs;
    }
};

export const updateAllChannels = async () => {
    for (const server of get(servers)) {
        const resp = await getChannels(parseInt(server.id));

        const _cs = resp.channels.map(
            (c) =>
                ({
                    id: c.id.toString(),
                    messages: [],
                    name: c.name,
                    server: c.server_id.toString(),
                    type: "channel",
                    channelType: c.channel_type,
                } as ChannelType)
        );

        get(servers).find((s) => s.id == server.id)!.channels = _cs;

        if (get(currentServer)?.id == server.id) {
            get(currentServer)!.channels = _cs;

            currentServer.update((c) => {
                c!.channels = _cs;

                return c;
            });
        }
    }
};
