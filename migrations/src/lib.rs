pub use sea_orm_migration::prelude::*;

mod m20221117_173031_users;
mod m20221117_173054_channels;
mod m20221117_173057_servers;
mod m20221117_173102_member_roles;
mod m20221117_173105_members;
mod m20221117_173108_roles;
mod m20221117_173116_user_settings;
mod m20221117_173118_user_servers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Mother tables (many-to-many bases).
            Box::new(m20221117_173031_users::Migration),
            Box::new(m20221117_173057_servers::Migration),
            // One-to-many relations.
            Box::new(m20221117_173054_channels::Migration),
            Box::new(m20221117_173105_members::Migration),
            Box::new(m20221117_173108_roles::Migration),
            Box::new(m20221117_173116_user_settings::Migration),
            // Many-to-many attachers.
            Box::new(m20221117_173118_user_servers::Migration),
            Box::new(m20221117_173102_member_roles::Migration),
        ]
    }
}
