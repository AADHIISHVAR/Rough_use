use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "register_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        panic!("No RelationDef for this entity")
    }
}

impl ActiveModelBehavior for ActiveModel {}
