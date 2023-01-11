use sea_orm_migration::prelude::*;

use crate::{m20221117_173031_users::Users, m20221117_173054_channels::ServerChannels};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChatMessages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatMessages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChatMessages::UserId).integer().not_null())
                    .col(ColumnDef::new(ChatMessages::ChannelId).integer().not_null())
                    .col(ColumnDef::new(ChatMessages::Timestamp).text().not_null())
                    .col(ColumnDef::new(ChatMessages::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-user_id")
                            .from(ChatMessages::Table, ChatMessages::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-channel_id")
                            .from(ChatMessages::Table, ChatMessages::ChannelId)
                            .to(ServerChannels::Table, ServerChannels::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChatMessages::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ChatMessages {
    #[iden = "messages"]
    Table,

    Id,

    #[iden = "user_id"]
    UserId,

    #[iden = "channel_id"]
    ChannelId,
    Timestamp,
    Content,
}
