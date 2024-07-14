use crate::{
    dao::{abstract_structures::paginated_query::PaginatedQuery, abstract_traits::db_methods::DbMethods}, models::camisas::camisa::Camisa, services::camisas::dao_camisa::get_dao_camisa
};


pub fn listar_camisas(per_page: i64, page: i64) -> PaginatedQuery<Camisa> {
    let dao = &mut get_dao_camisa();

    let result = dao.get_all(page, per_page).expect("Deu ruim");
    return result;
}