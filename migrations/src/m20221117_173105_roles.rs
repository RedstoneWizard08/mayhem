use sea_orm_migration::prelude::*;

use crate::m20221117_173054_servers::Servers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerRoles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServerRoles::Name).text().not_null())
                    .col(ColumnDef::new(ServerRoles::ServerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-server_roles-server_id")
                            .from(ServerRoles::Table, ServerRoles::ServerId)
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
            .drop_table(Table::drop().table(ServerRoles::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ServerRoles {
    #[iden = "server_roles"]
    Table,
    Id,
    Name,
    ServerId,
}
