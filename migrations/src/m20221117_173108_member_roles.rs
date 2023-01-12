use sea_orm_migration::prelude::*;

use crate::{m20221117_173102_members::ServerMembers, m20221117_173105_roles::ServerRoles};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MemberRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MemberRoles::MemberId).integer().not_null())
                    .col(ColumnDef::new(MemberRoles::RoleId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-member_roles")
                            .col(MemberRoles::MemberId)
                            .col(MemberRoles::RoleId)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member_roles-member_id")
                            .from(MemberRoles::Table, MemberRoles::MemberId)
                            .to(ServerMembers::Table, ServerMembers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member_roles-role_id")
                            .from(MemberRoles::Table, MemberRoles::RoleId)
                            .to(ServerRoles::Table, ServerRoles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MemberRoles::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum MemberRoles {
    #[iden = "member_roles"]
    Table,

    #[iden = "member_id"]
    MemberId,

    #[iden = "role_id"]
    RoleId,
}
