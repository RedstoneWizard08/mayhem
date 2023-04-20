#![allow(clippy::needless_return)]

pub use sea_orm_migration::prelude::*;

mod m20221117_173031_users;
mod m20221117_173054_servers;
mod m20221117_173057_channels;
mod m20221117_173102_members;
mod m20221117_173105_roles;
mod m20221117_173108_member_roles;
mod m20221117_173116_user_settings;
mod m20221117_173118_user_servers;
mod m20230110_235226_messages;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221117_173031_users::Migration),
            Box::new(m20221117_173054_servers::Migration),
            Box::new(m20221117_173057_channels::Migration),
            Box::new(m20221117_173102_members::Migration),
            Box::new(m20221117_173105_roles::Migration),
            Box::new(m20221117_173108_member_roles::Migration),
            Box::new(m20221117_173116_user_settings::Migration),
            Box::new(m20221117_173118_user_servers::Migration),
            Box::new(m20230110_235226_messages::Migration),
        ]
    }
}
