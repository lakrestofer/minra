use sea_orm_migration::prelude::*;

use crate::{m20231111_123534_sources::Source, m20231119_121558_tags::Tag};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SourceTagMapping::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SourceTagMapping::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SourceTagMapping::SourceId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SourceTagMapping::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_map_source")
                            .from(SourceTagMapping::Table, SourceTagMapping::SourceId)
                            .to(Source::Table, Source::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_map_tag")
                            .from(SourceTagMapping::Table, SourceTagMapping::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SourceTagMapping::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SourceTagMapping {
    Table,
    Id,
    SourceId,
    TagId,
}
