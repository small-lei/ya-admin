use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub customer_name: String,
    pub phone: String,
    #[sea_orm(column_type = "Text")]
    pub prescription: String,
    pub frame_type: String,
    pub lens_type: String,
    pub total_amount: Decimal,
    pub status: String,
    pub created_by: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relation for Relation::{:?}", self)
    }
}

impl ActiveModelBehavior for ActiveModel {}