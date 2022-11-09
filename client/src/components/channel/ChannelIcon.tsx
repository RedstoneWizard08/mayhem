import { FunctionComponent, JSX } from "preact";
import { route, useRouter } from "preact-router";
import { StateUpdater, useEffect, useState } from "preact/hooks";
import "./ChannelIcon.scss";

export interface ChannelIconComponentProps {
    id: string;
    name: string;
    type: "channel";
    channelType: "text" | "voice" | "announcement" | "events";
    server: string;

    open?: boolean;
};

export interface ChannelGroupComponentProps {
    id: string;
    name: string;
    type: "group";
    server: string;
    
    children?: ChannelIconComponentProps[];
};

export const ChannelIcon: FunctionComponent<ChannelGroupComponentProps | ChannelIconComponentProps> = (props) => {
    const [router] = useRouter();
    const [selected, setSelected] = useState(false);
    const [open, setOpen] = useState(true);

    useEffect(() => {
        const id = router.matches?.channel;
        
        if (id == props.id) setSelected(true);
        else setSelected(false);

        if (props.type == "group") setSelected(false);
    }, [router]);

    const handleClick = () => {
        if (props.type == "channel")
            route(`/servers/${props.server}/channels/${props.id}`);
        
        else setOpen(!open);
    };

    return (
        <>
            {
                (props.type == "channel" && !props.open) ?
                <></> :
                (
                    <>
                        <div
                            className={`channel${selected ? " selected" : ""} ${props.type}`}
                            onClick={handleClick}
                        >
                            {
                                props.type == "channel" ?
                                (
                                    props.channelType == "text" ?
                                    <i className="fa-solid fa-hashtag"></i> :
                                    
                                    props.channelType == "voice" ?
                                    <i className="fa-solid fa-microphone"></i> :
                                    
                                    props.channelType == "announcement" ?
                                    <i className="fa-solid fa-bullhorn"></i> :
                                    
                                    props.channelType == "events" ?
                                    <i className="fa-solid fa-calendar-days"></i> :
                                    
                                    ""
                                ) :
                                ""
                            }
                            
                            {props.name}

                            {
                                props.type == "group" ? (
                                    open ?
                                    <i className="fa-solid fa-caret-down" /> :
                                    <i className="fa-solid fa-caret-up" />
                                ) :
                                ""
                            }
                        </div>

                        {
                            props.type == "group" ?
                            props.children?.map((channel) => {
                                const { id, name, server, type, channelType } = channel;

                                return (
                                    <ChannelIcon
                                        id={id}
                                        name={name}
                                        server={server}
                                        type={type}
                                        open={open}
                                        channelType={channelType}
                                    />
                                );
                            }) :
                            <></>
                        }
                    </>
                )
            }
        </>
    );
};
