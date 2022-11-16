import { createRef } from "preact";
import { useState } from "preact/hooks";
import { ChatMessage, ChatMessageProps } from "./ChatMessage";
import "./ChatWindow.scss";

export const ChatWindow = () => {
    const [messages, setMessages] = useState<Partial<ChatMessageProps>[]>([]);
    const [message, setMessage] = useState("");
    const messagesRef = createRef<HTMLDivElement>();

    const update = (e: KeyboardEvent) => {
        setMessage((e.target as HTMLInputElement)?.value);
    };

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key == "Enter") {
            if (message.replace(/\s/gm, "") == "") return;

            messages.push({ content: message, timestamp: new Date() });
            
            setMessage("");
            setMessages(messages);

            if (messagesRef.current) {
                messagesRef.current.scrollTo({
                    top: messagesRef.current.scrollHeight,
                });
            }
        } else update(e);
    };

    return (
        <div className="chat-window">
            <div className="messages" ref={messagesRef}>
                {messages.map((m) => <ChatMessage {...m} />)}
            </div>

            <div class="message-input">
                <input type="text" placeholder="Type a message..." onKeyDown={onKeyDown} onKeyUp={update} value={message} />
            </div>
        </div>
    );
};
