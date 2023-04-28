import { updateServers } from "../server";
import { JOIN_SERVER, LEAVE_SERVER } from "./keys";

export const serverAction = (
    action: string,
    user: number,
    server: number,
    ws: WebSocket
): Promise<boolean> => {
    const promise = new Promise<boolean>((resolve, reject) => {
        ws.addEventListener("message", async (ev) => {
            try {
                const data =
                    typeof JSON.parse(ev.data) == "string"
                        ? JSON.parse(JSON.parse(ev.data))
                        : JSON.parse(ev.data);

                if (data.action == action) {
                    await updateServers();

                    return resolve(true);
                }
            } catch (e) {
                reject(e);
            }
        });
    });

    ws.send(
        JSON.stringify({
            action: action,
            data: {
                user_id: user,
                server_id: server,
            },
        })
    );

    return promise;
};

export const joinServer = (user: number, server: number, ws: WebSocket): Promise<boolean> => {
    return serverAction(JOIN_SERVER, user, server, ws);
};

export const leaveServer = (user: number, server: number, ws: WebSocket): Promise<boolean> => {
    return serverAction(LEAVE_SERVER, user, server, ws);
};
