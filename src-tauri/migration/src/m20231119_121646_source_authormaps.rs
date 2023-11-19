use sea_orm_migration::prelude::*;

use crate::{m20231111_123534_sources::Source, m20231119_121615_authors::Author};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SourceAuthorMapping::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SourceAuthorMapping::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SourceAuthorMapping::SourceId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SourceAuthorMapping::AuthorId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_map_source")
                            .from(SourceAuthorMapping::Table, SourceAuthorMapping::SourceId)
                            .to(Source::Table, Source::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_map_tag")
                            .from(SourceAuthorMapping::Table, SourceAuthorMapping::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SourceAuthorMapping::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SourceAuthorMapping {
    Table,
    Id,
    SourceId,
    AuthorId,
}
