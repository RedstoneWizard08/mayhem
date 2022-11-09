import { useState } from "preact/hooks";
import { ChannelGroupComponentProps, ChannelIcon, ChannelIconComponentProps } from "./ChannelIcon";
import "./ChannelList.scss";

export const ChannelList = () => {
    const channels: (ChannelIconComponentProps | ChannelGroupComponentProps)[] = [
        {
            id: "0",
            name: "Test Channel Group",
            type: "group",
            server: "1",

            children: [
                {
                    id: "1",
                    name: "Test Channel 1",
                    type: "channel",
                    server: "1",
                    channelType: "text",
                },
                {
                    id: "2",
                    name: "Test Channel 2",
                    type: "channel",
                    server: "1",
                    channelType: "voice",
                },
            ],
        },
        {
            id: "3",
            name: "Test Channel Group",
            type: "group",
            server: "1",

            children: [
                {
                    id: "4",
                    name: "Test Channel 3",
                    type: "channel",
                    server: "1",
                    channelType: "announcement",
                },
                {
                    id: "5",
                    name: "Test Channel 4",
                    type: "channel",
                    server: "1",
                    channelType: "events",
                },
            ],
        },
        {
            id: "6",
            name: "Test Channel 5",
            type: "channel",
            server: "1",
            channelType: "text",
        },
    ];

    return (
        <div className="channels">
            {channels.map((channel) => <ChannelIcon {...channel} />)}
        </div>
    );
};
