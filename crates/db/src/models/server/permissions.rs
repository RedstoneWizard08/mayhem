use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, FromPrimitive, PartialEq, Copy)]
pub enum ServerPermission {
    SendMessage,
    ReadMessage,
    EditMessage,
    DeleteMessage,

    LoadOldMessages,
    AtMention,
    AtEveryone,

    JoinVoice,
    VoiceUnmute,
    VoiceUndeafen,

    VoiceMuteOther,
    VoiceDeafenOther,
    TextMuteOther,
    TextDeafenOther,

    DeleteOtherMessage,
    UseCommands,
    UseAdminCommands,

    Administrator,
    EditServer,
    DeleteServer,

    AddChannel,
    DeleteChannel,
    EditChannel,

    EditRole,
    AddRole,
    DeleteRole,

    CreateInvite,
    DeleteInvite,
    EditInvite,

    AddBot,
    AddIntegration,
    AddPlugin,

    AddRoleMembers,
    DeleteRoleMembers,
    EditRoleMembers,

    BulkActions,
    EnableFeatures,
}

#[derive(Deserialize, Serialize, Debug, Clone, FromPrimitive, PartialEq, Copy)]
pub enum PermissionAccessType {
    GRANTED,
    NONE,
    DENIED,
}

pub fn deserialize_permission(string: String) -> (ServerPermission, PermissionAccessType) {
    let mut split = string.split(':');

    let permission_id = split.next().unwrap().parse::<u32>().unwrap();
    let access = split.nth(1).unwrap().parse::<u32>().unwrap();

    let permission: ServerPermission = num::FromPrimitive::from_u32(permission_id).unwrap();
    let access_type: PermissionAccessType = num::FromPrimitive::from_u32(access).unwrap();

    return (permission, access_type);
}

pub fn deserialize_permissions(
    strings: Vec<String>,
) -> Vec<(ServerPermission, PermissionAccessType)> {
    let mut vec: Vec<(ServerPermission, PermissionAccessType)> = Vec::new();

    for string in strings {
        vec.push(deserialize_permission(string));
    }

    return vec;
}
