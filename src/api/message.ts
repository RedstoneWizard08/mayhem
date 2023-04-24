import { UserCache } from "./cache";
import { getDefaultProfilePic } from "./icon";
export interface ChatMessageProps {
    avatar: string;
    name: string;
    timestamp: string | Date;
    content: string;
    user_id: number;
}

const cache = new UserCache();

export const getSender = async (user_id: number) => {
    return await cache.get(user_id);
};

export const fillMessageProps = async (
    partial?: Partial<ChatMessageProps>
): Promise<ChatMessageProps> => {
    const props: Partial<ChatMessageProps> = {};

    const sender = await getSender(partial!.user_id!);

    props.avatar = partial?.avatar || getDefaultProfilePic(sender?.username);
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
