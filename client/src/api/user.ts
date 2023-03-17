export interface CompleteRole {
    id: number,
    name: string,
    server_id: number,
    member_ids: number[],
}

export interface CompleteMember {
    id: number,
    name: string,
    nick: string,
    role_ids: number[],
    server_id: number,
}

export interface Channel {
    id: number,
    name: string,
    server_id: number,
    channel_type: string,
}

export interface CompleteServer {
    id: number,
    name: string,
    roles: CompleteRole[],
    members: CompleteMember[],
    channels: Channel[],
}

export interface UserSettings {
    id: number;
    user_id: number;
}

export interface UserInfo {
    id: number;
    first_name: string;
    last_name: string;
    email: string;
    username: string;
    servers: CompleteServer[];
    settings?: UserSettings;
}
