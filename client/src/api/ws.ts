import { channels, currentChannel, messages } from "../stores/current";
import { get, type Writable } from "svelte/store";
import type { ChannelType } from "./channel";
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

                messages.update((m) => {
                    m.push({
                        timestamp: new Date(messageData.timestamp),
                        content: messageData.content,
                    });

                    return m;
                });

                if (get(currentChannel)?.type == "channel") {
                    (currentChannel as Writable<ChannelType>).update((c) => {
                        if (c)
                            c.messages = get(messages);

                        return c;
                    });

                    channels.update((cs) => {
                        const c = cs.find((c) => c.id == get(currentChannel)!.id)! as ChannelType;
                        
                        if (c)
                            c.messages = get(messages);

                        return cs;
                    });
                }
            } else if (data.action == "GetMessagesForChannel") {
                const messagesData = data.data.messages as IncomingChatMessage[];

                messages.update((m) => {
                    messagesData
                        .map((msg) => ({
                            timestamp: new Date(msg.timestamp),
                            content: msg.content,
                        }))
                        .forEach((msg) => m.push(msg));

                    return m;
                });

                if (get(currentChannel)?.type == "channel") {
                    (currentChannel as Writable<ChannelType>).update((c) => {
                        if (c)
                            c.messages = get(messages);

                        return c;
                    });

                    channels.update((cs) => {
                        const c = cs.find((c) => c.id == get(currentChannel)!.id)! as ChannelType;
                        
                        if (c)
                            c.messages = get(messages);

                        return cs;
                    });
                }
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
