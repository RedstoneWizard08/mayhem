import { currentChannel, user } from "../stores/current";
import { get } from "svelte/store";
import { updateAllChannels } from "./channel";
import { joinServer } from "./ws/server";
import { setupMessageHandler } from "./ws/message";

export class WebSocketAPI {
    private ws?: WebSocket;

    public connect() {
        const url = new URL("/api/ws", window.location.href);

        url.protocol = url.protocol.replace("http", "ws");

        this.ws = new WebSocket(url.href);

        setupMessageHandler(this.ws!);

        this.ws.addEventListener("message", async (ev) => {
            const data =
                typeof JSON.parse(ev.data) == "string"
                    ? JSON.parse(JSON.parse(ev.data))
                    : JSON.parse(ev.data);

            if (data.action == "CreateServer") {
                const serverData = data.data as { id: number; name: string };

                await joinServer(get(user)!.id, serverData.id, this.ws!);
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
