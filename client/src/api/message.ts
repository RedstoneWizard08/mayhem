import { page } from "$app/stores";
import axios from "axios";
import { get } from "svelte/store";
import type { UserInfo } from "./user";

export interface ChatMessageProps {
    avatar: string;
    name: string;
    timestamp: string | Date;
    content: string;
    user_id: number;
}

const cache = new Map<number, UserInfo>();

export const getSender = async (user_id: number) => {
    if (cache.has(user_id)) return cache.get(user_id);

    const uri = new URL("/api/users/" + user_id, get(page).url.href);

    const resp = await axios.get<UserInfo>(uri.toString());

    cache.set(user_id, resp.data);

    return resp.data;
};

export const fillMessageProps = async (partial?: Partial<ChatMessageProps>): Promise<ChatMessageProps> => {
    const props: Partial<ChatMessageProps> = {};

    const sender = await getSender(partial!.user_id!);

    props.avatar = partial?.avatar || "https://ui-avatars.com/api/?background=random&rounded=true&name=" + sender?.username;
    props.name = partial?.name || sender?.username;
    props.timestamp = partial?.timestamp || new Date();
    props.content = partial?.content || "Unknown Message Content";

    return props as ChatMessageProps;
};

export const fixContent = (content: string) => {
    let real = content;

    real = real.replaceAll("<", "&lt;");
    real = real.replaceAll(">", "&gt;");

    real = real.replace(/\*\*\*([^\*]+)\*\*\*/gm, "<b><i>$1</i></b>");
    real = real.replace(/\*\*([^\*]+)\*\*/gm, "<b>$1</b>");
    real = real.replace(/\*([^\*]+)\*/gm, "<i>$1</i>");

    real = real.replace(/```([^\n]+)?\n(.+?(?=```))```/gims, '<code lang="$1">$1</code>');
    real = real.replace(/`([^`]+)`/gm, "<code>$1</code>");

    real = real.replace(
        /(https?):\/\/([^\s]+)/gims,
        '<a href="$1://$2" target="_blank" rel="noreferrer">$1://$2</a>'
    );

    real = real.replace(
        /([^@:\/\s\!\?]+)@([^\.\s\!\?]+)\.([^\s\!\?]+)/gims,
        '<a href="mailto:$1@$2.$3" target="_blank" rel="noreferrer">$1@$2.$3</a>'
    );

    real = real.replaceAll("\n", "<br />");

    return real;
};

export interface IncomingChatMessage {
    id: number;
    user_id: number;
    channel_id: number;
    timestamp: string;
    content: string;
}
