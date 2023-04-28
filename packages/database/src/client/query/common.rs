use sea_orm::DbConn;
use std::sync::Arc;

pub enum UserQueryType {
    FetchUser,
    FetchUserServers,
    FetchUserSettings,
}

pub enum ServerQueryType {
    FetchServer,

    FetchRoles,
    FetchMembers,
    FetchChannels,
}

pub trait ConnectionHolder {
    /// Returns the internal database connection as a
    /// reference to a [DbConn](sea_orm::DbConn) object.
    /// This actually returns a case of the internal
    /// [Arc](std::sync::Arc) reference, for ease of use.
    fn get_connection(&self) -> &DbConn;

    /// Returns the internal database connection. This
    /// will return the [Arc](std::sync::Arc) reference
    /// that contains the connection object or pool.
    fn get_connection_ref(&self) -> Arc<DbConn>;

    /// Returns a constant database connection. This is
    /// to enforce memory safety because sea_orm's native
    /// [DbConn](sea_orm::DbConn) does not implement
    /// [Send](core::marker::Send) or [Sync](core::marker::Sync).
    fn get_connection_raw(&self) -> *const DbConn;
}
