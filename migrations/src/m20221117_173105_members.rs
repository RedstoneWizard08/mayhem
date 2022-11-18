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
                    .table(ServerMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerMembers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServerMembers::Name).text().not_null())
                    .col(ColumnDef::new(ServerMembers::Nick).text().not_null())
                    .col(ColumnDef::new(ServerMembers::ServerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-server_members-server_id")
                            .from(ServerMembers::Table, ServerMembers::ServerId)
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
            .drop_table(Table::drop().table(ServerMembers::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ServerMembers {
    #[iden = "server_members"]
    Table,
    Id,
    Name,
    Nick,
    ServerId,
}
