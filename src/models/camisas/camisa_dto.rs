use diesel::prelude::*;

use crate::schema::camisas;


#[derive(Insertable)]
#[diesel(table_name = camisas)]
pub struct CamisaDTO {
    pub nome: String,
    pub data_fabricacao: String
}
