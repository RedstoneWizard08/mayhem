import "./ChatMessage.scss";
import moment from "moment";
import { FunctionComponent } from "preact";
import { useState } from "preact/hooks";

export interface ChatMessageProps {
    avatar: string;
    name: string;
    timestamp: string | Date;
    content: string;
}

export const fillMessageProps = (partial?: Partial<ChatMessageProps>): ChatMessageProps => {
    const props: Partial<ChatMessageProps> = {};

    props.avatar = partial?.avatar || "https://ui-avatars.com/api/?rounded=true&name=John+Doe";
    props.name = partial?.name || "John Doe";
    props.timestamp = partial?.timestamp || new Date();
    props.content = partial?.content || "Unknown Message Content";

    return props as ChatMessageProps;
};

export const ChatMessage: FunctionComponent<Partial<ChatMessageProps> | undefined> = (props) => {
    const message = fillMessageProps(props);
    const [time, setTime] = useState(moment(message.timestamp).fromNow());

    setInterval(() => setTime(moment(message.timestamp).fromNow()), 1000);

    return (
        <div className="chat-message">
            <img src={message.avatar} />

            <div className="content">
                <div className="info">
                    <p className="author">{message.name}</p>
                    <p className="timestamp">{time}</p>
                </div>

                <p className="message">{message.content}</p>
            </div>
        </div>
    );
};
