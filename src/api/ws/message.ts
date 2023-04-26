import { currentChannel } from "../../stores/current";
import { fillMessageProps, type ChatMessageProps, type IncomingChatMessage } from "../message";
import { RECIEVE_MESSAGE } from "./keys";

export const setupMessageHandler = (ws: WebSocket) => {
    ws.addEventListener("message", async (ev) => {
        const data =
            typeof JSON.parse(ev.data) == "string"
                ? JSON.parse(JSON.parse(ev.data))
                : JSON.parse(ev.data);

        if (data.action == RECIEVE_MESSAGE) {
            const messageData = data.data as IncomingChatMessage;

            const _mData = {
                timestamp: new Date(messageData.timestamp),
                content: messageData.content,
                user_id: messageData.user_id,
            };

            const mData = await fillMessageProps(_mData);

            currentChannel.update((c) => {
                c?.messages.push(mData);

                return c;
            });
        } else if (data.action == "GetMessagesForChannel") {
            const messagesData = data.data.messages as IncomingChatMessage[];

            const messages: ChatMessageProps[] = [];

            await Promise.all(
                messagesData
                    .map((msg) => ({
                        timestamp: new Date(msg.timestamp),
                        content: msg.content,
                        user_id: msg.user_id,
                    }))
                    .map(async (msg: Partial<ChatMessageProps>) =>
                        messages.push(await fillMessageProps(msg))
                    )
            );

            currentChannel.update((c) => {
                if (c) c.messages = messages;

                return c;
            });
        }
    });
};
