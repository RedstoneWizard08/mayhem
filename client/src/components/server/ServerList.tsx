import { ServerIcon, ServerIconComponentProps } from "./ServerIcon";
import { DiscordIcon } from "../../icons/DiscordIcon";
import "./ServerList.scss";

export const ServerList = () => {
    const servers: ServerIconComponentProps[] = [
        {
            id: "1",
            name: "Test Server 1",
            type: "server",
        },
        {
            id: "2",
            name: "Test Server 2",
            type: "server",
        },
        {
            id: "3",
            name: "Test Server 3",
            type: "server",
        },
        {
            id: "4",
            name: "Test Server 4",
            type: "server",
        },
    ];

    return (
        <div className="servers">
            <ServerIcon type="channel" id="@me" name="DMs" icon={<DiscordIcon />} />
            {servers.map((server) => <ServerIcon {...server} />)}
        </div>
    );
};
