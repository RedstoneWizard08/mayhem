use sea_orm_migration::prelude::*;

use crate::m20221117_173031_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserSettings::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_settings-user_id")
                            .from(UserSettings::Table, UserSettings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSettings::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserSettings {
    #[iden = "user_settings"]
    Table,
    Id,
    UserId,
}
