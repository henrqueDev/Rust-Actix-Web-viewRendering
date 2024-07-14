use actix_web::{get, web, Responder};

use crate::{
    dao::abstract_structures::paginated_query::Pagination, services::camisas::listagem::listar_camisas
};

#[get("/list")]
pub async fn list(filter: web::Query<Pagination>) -> impl Responder {
    let result  = listar_camisas(filter.per_page, filter.page);
    return web::Json(result);
}