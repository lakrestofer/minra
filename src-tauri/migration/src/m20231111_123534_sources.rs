use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Source::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Source::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Source::CheckSum).string().not_null())
                    .col(ColumnDef::new(Source::Title).string().not_null())
                    .col(ColumnDef::new(Source::Description).string().not_null())
                    .col(ColumnDef::new(Source::Path).string().not_null())
                    .col(ColumnDef::new(Source::Mime).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Source::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Source {
    Table,
    Id,
    CheckSum,
    Title,
    Description,
    Path,
    Mime, // the type of the source (pdf, html)
}
