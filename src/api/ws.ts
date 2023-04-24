import { currentChannel, servers, user } from "../stores/current";
import { get } from "svelte/store";
import { fillMessageProps, type ChatMessageProps, type IncomingChatMessage } from "./message";
import { getServers } from "./server";
import { updateAllChannels } from "./channel";

export class WebSocketAPI {
    private ws?: WebSocket;

    public connect() {
        const url = new URL("/api/ws", window.location.href);

        url.protocol = url.protocol.replace("http", "ws");

        this.ws = new WebSocket(url.href);

        this.ws.addEventListener("message", async (ev) => {
            const data =
                typeof JSON.parse(ev.data) == "string"
                    ? JSON.parse(JSON.parse(ev.data))
                    : JSON.parse(ev.data);

            if (data.action == "RecieveMessage") {
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
            } else if (data.action == "CreateServer") {
                const serverData = data.data as { id: number; name: string };

                this.send(
                    JSON.stringify({
                        action: "JoinServer",
                        data: {
                            user_id: get(user)?.id,
                            server_id: serverData.id,
                        },
                    })
                );
            } else if (data.action == "JoinServer") {
                servers.set(
                    (await getServers()).servers.map((s) => ({
                        id: s.id.toString(),
                        name: s.name,
                        type: "server",
                        channels: [],
                    }))
                );
            } else if (data.action == "CreateChannel") {
                await updateAllChannels();
            }
        });

        this.ws.addEventListener("open", this.getAll.bind(this));
    }

    public send(data: string) {
        this.ws?.send(data);
    }

    public getAll() {
        this.ws?.send(
            JSON.stringify({
                action: "GetMessagesForChannel",
                data: parseInt(get(currentChannel)?.id || "-1"),
            })
        );
    }

    public close() {
        this.ws?.close();
    }
}
