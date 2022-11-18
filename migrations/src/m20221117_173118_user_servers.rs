use sea_orm_migration::prelude::*;

use crate::{m20221117_173031_users::Users, m20221117_173057_servers::Servers};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserServers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserServers::UserId).integer().not_null())
                    .col(ColumnDef::new(UserServers::ServerId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-user_servers")
                            .col(UserServers::UserId)
                            .col(UserServers::ServerId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_servers-user_id")
                            .from(UserServers::Table, UserServers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_servers-server_id")
                            .from(UserServers::Table, UserServers::ServerId)
                            .to(Servers::Table, Servers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserServers::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserServers {
    #[iden = "user_servers"]
    Table,

    #[iden = "user_id"]
    UserId,

    #[iden = "server_id"]
    ServerId,
}
