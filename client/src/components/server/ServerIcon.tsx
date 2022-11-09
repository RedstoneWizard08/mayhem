import { FunctionComponent, JSX } from "preact";
import { route, useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";
import { trim } from "../../util";
import "./ServerIcon.scss";

export interface ServerIconComponentProps {
    id: string;
    name: string;
    type: "server";
    icon?: string | JSX.Element;
};

export interface ChannelIconComponentProps {
    id: string;
    name: string;
    type: "channel";
    icon?: string | JSX.Element;
};

export const ServerIcon: FunctionComponent<ServerIconComponentProps | ChannelIconComponentProps> = (props) => {
    const [router] = useRouter();
    const [selected, setSelected] = useState(false);

    useEffect(() => {
        const id = props.type == "server" ? router.matches?.server : router.matches?.channel;

        if (id == props.id) setSelected(true);
        else setSelected(false);

        if (props.id == "0" && router.path?.startsWith("/channels/")) setSelected(true);
    }, [router]);

    const handleClick = () => {
        if (props.type == "channel") route("/channels/" + props.id);
        else route("/servers/" + props.id);
    };

    return (
        <div className={`server${selected ? " selected" : ""}`} onClick={handleClick}>
            {
                props.icon ? (
                    typeof props.icon == "string" ?
                    <img src={props.icon} /> :
                    props.icon
                ) : trim(
                    props.name.split(" ")
                        .filter((w) => w.toLowerCase() != "server")
                        .map((w) => w.charAt(0))
                        .join(""),
                    2,
                )
            }
        </div>
    );
};
