export interface ServerIconProps {
    id: string;
    name: string;
    type: "server";
    icon?: "discord" | string;
}

export interface ChannelIconProps {
    id: string;
    name: string;
    type: "channel";
    icon?: "discord" | string;
}
