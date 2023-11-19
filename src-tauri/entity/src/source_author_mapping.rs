//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "source_author_mapping")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub source_id: i32,
    pub author_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::author::Entity",
        from = "Column::AuthorId",
        to = "super::author::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Author,
    #[sea_orm(
        belongs_to = "super::source::Entity",
        from = "Column::SourceId",
        to = "super::source::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Source,
}

impl Related<super::author::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl Related<super::source::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Source.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}