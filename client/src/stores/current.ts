import type { ServerIconProps } from "../api/icon";
import { writable } from "svelte/store";
import type { ChatMessageProps } from "../api/message";
import type { ChannelGroupType, ChannelType } from "src/api/channel";

export const currentServer = writable<ServerIconProps | null>(null);
export const currentChannel = writable<(ChannelType | ChannelGroupType) | null>(null);
export const messages = writable<Partial<ChatMessageProps>[]>([]);