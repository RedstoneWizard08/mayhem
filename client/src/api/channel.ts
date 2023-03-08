import discord from "../assets/discord.png";
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

export const testChannelData: (ChannelType | ChannelGroupType)[] = [
    {
        id: "0",
        name: "Test Channel Group",
        type: "group",
        server: "1",

        channels: [
            {
                id: "1",
                name: "Test Channel 1",
                type: "channel",
                server: "1",
                channelType: "text",
                messages: [
                    {
                        content:
                            "Hello and welcome to the Mayhem open alpha!\n" +
                            "Please settle yourself in and get comfortable.\n" +
                            "If you have any questions, please contact us at\n" +
                            "mayhem@nosadnile.net!\n\n" +
                            "If you have any feedback or you have found any bugs,\n" +
                            "please create an issue on our issue tracker:\n" +
                            "https://github.com/RedstoneWizard08/Mayhem/issues\n\n" +
                            "Thanks, and have fun!",

                        name: "Mayhem",
                        avatar: discord,
                    },
                ],
            },
            {
                id: "2",
                name: "Test Channel 2",
                type: "channel",
                server: "1",
                channelType: "voice",
                messages: [],
            },
        ],
    },
    {
        id: "3",
        name: "Test Channel Group",
        type: "group",
        server: "1",

        channels: [
            {
                id: "4",
                name: "Test Channel 3",
                type: "channel",
                server: "1",
                channelType: "announcement",
                messages: [],
            },
            {
                id: "5",
                name: "Test Channel 4",
                type: "channel",
                server: "1",
                channelType: "events",
                messages: [],
            },
        ],
    },
    {
        id: "6",
        name: "Test Channel 5",
        type: "channel",
        server: "1",
        channelType: "text",
        messages: [],
    },
];
