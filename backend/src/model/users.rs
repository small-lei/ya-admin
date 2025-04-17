use sea_orm::entity::prelude::*;
use sea_orm::Iterable;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Orders
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Orders => Entity::has_many(super::orders::Entity).into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}