use diesel::prelude::*;
use crate::schema::calcas;



#[derive(Insertable)]
#[diesel(belongs_to(Camisa))]
#[diesel(table_name = calcas)]
pub struct CalcaDTO {
    pub is_visually_malnourished: bool,
    pub has_gastro_intestinal_inconsistency: bool,
    pub has_malnutrition_patology: bool,
    pub camisa_id: i32,
}