use diesel::prelude::*;

use crate::schema::calcas;
use crate::models::camisas::camisa::Camisa;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Camisa))]
#[diesel(table_name = calcas)]
pub struct Calca {
    pub id: i32,
    pub is_visually_malnourished: bool,
    pub has_gastro_intestinal_inconsistency: bool,
    pub has_malnutrition_patology: bool,
    pub camisa_id: i32,
}