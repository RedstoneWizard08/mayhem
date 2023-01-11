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
