use crate::{
    dao::{
        abstract_structures::{dao::DAO, paginated_query::PaginatedQuery}, 
        abstract_traits::db_methods::DbMethods, db_connection::get_connection
    }, 
    models::camisas::camisa::Camisa, 
    models::camisas::camisa_dto::CamisaDTO
};

use crate::schema::camisas::dsl::*;
use std::fmt::Error;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};


pub fn get_dao_camisa() -> DAO<camisas> {
    let conexao = get_connection();
    DAO::<camisas> {connection: conexao, logs: None, table: camisas}
}



impl DbMethods<Camisa, CamisaDTO> for DAO<camisas>{
    
    fn insert(self: &mut Self, obj: CamisaDTO) -> Result<usize, Error> {

        let result = diesel::insert_into(self.table)
                .values(&obj)
                .execute(&mut self.connection)
                .expect("Error saving new post");
        Ok(result)
    }
    
    fn get_all(self: &mut Self, page: i64, per_page: i64) -> Result<PaginatedQuery<Camisa>, Error> {

        //let camisas_size = self.table.count().get_result(&mut self.connection).expect("Deu ruim");
        let per_page = per_page;
        let page = page;
        let off = (page - 1) * per_page;

        let results: Vec<Camisa> = self.table
            .limit(per_page)
            .offset(off)
            .select(Camisa::as_select())
            .load(&mut self.connection)
            .expect("Error loading camisas");

        let query_paginated = PaginatedQuery {
            data: Some(results),
            per_page,
            page,
        };

        return Ok(query_paginated);
    }

    fn count(self: &mut Self) -> Result<usize, Error> {
        let conn = &mut self.connection;
        
        match self.table.select(Camisa::as_select()).load(conn) {
            Err(why) => panic!("{}", why),
            Ok(count) => return Ok(count.len())
        };

    }
}