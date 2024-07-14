use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::camisas;


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = camisas)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Camisa {
    pub id: i32,
    pub nome: String,
    pub data_fabricacao: String,
}
