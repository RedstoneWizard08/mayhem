import type { ChannelType } from "./channel";

export interface ServerIconProps {
    id: string;
    name: string;
    type: "server";
    icon?: "discord" | string;
    channels: ChannelType[];
}

export interface ChannelIconProps {
    id: string;
    name: string;
    type: "channel";
    icon?: "discord" | string;
}
