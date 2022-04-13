use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "officers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub vrchat_name: String,
    pub vrchat_id: String,
    pub started_monitoring: DateTime,
    pub delete_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
