import type { ServerIconProps } from "../api/icon";
import { writable } from "svelte/store";
import type { ChatMessageProps } from "../api/message";
import type { ChannelGroupType, ChannelType } from "../api/channel";
import type { WebSocketAPI } from "../api/ws";
import type { UserInfo } from "../api/user";

export const currentServer = writable<ServerIconProps | null>(null);
export const currentChannel = writable<ChannelType | null>(null);
export const messages = writable<Partial<ChatMessageProps>[]>([]);
export const channels = writable<(ChannelType | ChannelGroupType)[]>([]);
export const servers = writable<ServerIconProps[]>([]);
export const ws = writable<WebSocketAPI | null>(null);
export const token = writable<string | null>(null);
export const user = writable<UserInfo | null>(null);
