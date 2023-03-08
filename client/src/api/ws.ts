import { currentChannel } from "../stores/current";
import { get } from "svelte/store";
import type { IncomingChatMessage } from "./message";

export class WebSocketAPI {
    private ws?: WebSocket;

    public connect() {
        const url = new URL("/api/ws", window.location.href);

        url.protocol = url.protocol.replace("http", "ws");

        this.ws = new WebSocket(url.href);

        this.ws.addEventListener("message", (ev) => {
            const data =
                typeof JSON.parse(ev.data) == "string"
                    ? JSON.parse(JSON.parse(ev.data))
                    : JSON.parse(ev.data);

            if (data.action == "RecieveMessage") {
                const messageData = data.data as IncomingChatMessage;

                currentChannel.update((c) => {
                    c?.messages.push({
                        timestamp: new Date(messageData.timestamp),
                        content: messageData.content,
                    });

                    return c;
                });
            } else if (data.action == "GetMessagesForChannel") {
                const messagesData = data.data.messages as IncomingChatMessage[];

                currentChannel.update((c) => {
                    if (c)
                        c.messages = [];

                    messagesData
                        .map((msg) => ({
                            timestamp: new Date(msg.timestamp),
                            content: msg.content,
                        }))
                        .forEach((msg) => c?.messages.push(msg));

                    return c;
                });
            }
        });

        this.ws.addEventListener("open", this.getAll.bind(this));
    }

    public send(data: string) {
        this.ws?.send(data);
    }

    public getAll() {
        this.ws?.send(JSON.stringify({ action: "GetMessagesForChannel", data: parseInt(get(currentChannel)?.id || "-1") }));
    }

    public close() {
        this.ws?.close();
    }
}
