import "./MainApp.scss";
import { FunctionComponent } from "preact";
import { DefaultComponentProps } from "../util";
import { ServerList } from "../components/server/ServerList";
import { ChannelList } from "../components/channel/ChannelList";

export const MainApp: FunctionComponent<DefaultComponentProps> = ({ children }) => {
    return (
        <div className="main-app">
            <ServerList />
            <ChannelList />
            {children}
        </div>
    );
};
