use sea_orm_migration::prelude::*;

use crate::m20221117_173057_servers::Servers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerChannels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerChannels::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServerChannels::Name).text().not_null())
                    .col(
                        ColumnDef::new(ServerChannels::ServerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-server_channels-server_id")
                            .from(ServerChannels::Table, ServerChannels::ServerId)
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
            .drop_table(Table::drop().table(ServerChannels::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ServerChannels {
    #[iden = "server_channels"]
    Table,
    Id,
    Name,
    ServerId,
}
