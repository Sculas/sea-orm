//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "underscores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub a_b_c_d: i32,
    pub a_b_c_dd: i32,
    pub a_b_cc_d: i32,
    pub a_bb_c_d: i32,
    pub aa_b_c_d: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Iterable;

    #[test]
    fn column_names() {
        assert_eq!(
            Column::iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            vec!["id", "a_b_c_d", "a_b_c_dd", "a_b_cc_d", "a_bb_c_d", "aa_b_c_d"]
        )
    }
}
