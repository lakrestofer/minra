pub use sea_orm_migration::prelude::*;

mod m20231111_123534_sources;
mod m20231119_121558_tags;
mod m20231119_121615_authors;
mod m20231119_121637_source_tagmaps;
mod m20231119_121646_source_authormaps;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231111_123534_sources::Migration),
            Box::new(m20231119_121558_tags::Migration),
            Box::new(m20231119_121615_authors::Migration),
            Box::new(m20231119_121637_source_tagmaps::Migration),
            Box::new(m20231119_121646_source_authormaps::Migration),
        ]
    }
}
