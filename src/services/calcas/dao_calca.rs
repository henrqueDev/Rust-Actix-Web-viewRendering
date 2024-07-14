use crate::{
    dao::{
        abstract_structures::{
        dao::DAO, 
        paginated_query::PaginatedQuery},
        abstract_traits::db_methods::DbMethods, 
        db_connection::get_connection
    }, 
    models::calcas::calca::Calca, 
    models::calcas::calca_dto::CalcaDTO,
    schema::calcas::dsl::*
};

use std::fmt::Error;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};


pub fn get_dao_calca() -> DAO<calcas> {
    let conexao = get_connection();
    DAO::<calcas> {connection: conexao, logs: None, table: calcas }
}

impl DbMethods<Calca, CalcaDTO> for DAO<calcas>{

    fn insert(self: &mut Self, obj: CalcaDTO) -> Result<usize, Error> {
        let result = diesel::insert_into(self.table)
                .values(&obj)
                .execute(&mut self.connection)
                .expect("Error saving new post");
        Ok(result)
    }
    
    fn get_all(self: &mut Self, page: i64, per_page: i64) -> Result<PaginatedQuery<Calca>, Error> {

        let calcas_size = calcas::count(self.table).get_result(&mut self.connection).expect("Deu ruim");
        let per_page = per_page;
        let page = page;
        let off = (page - 1) * per_page;

        let results: Vec<Calca> = self.table
            .limit(calcas_size)
            .offset(off)
            .select(Calca::as_select())
            .load(&mut self.connection)
            .expect("Error loading calcas");

        let query_paginated = PaginatedQuery {
            data: Some(results),
            per_page,
            page,
        };

        return Ok(query_paginated);
    }
    
    fn count(self: &mut Self) -> Result<usize, Error> {
        let conn = &mut self.connection;
        
        match self.table.select(Calca::as_select()).load(conn) {
            Err(why) => panic!("{}", why),
            Ok(count) => return Ok(count.len())
        };
    }
    
}