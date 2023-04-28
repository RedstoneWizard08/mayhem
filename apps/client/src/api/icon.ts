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

export const getDefaultProfilePic = (username?: string) => {
    if (username == "JohnDoe26") username = "John Dorian";

    return `https://ui-avatars.com/api/?background=random&rounded=true&name=${username}`;
};
