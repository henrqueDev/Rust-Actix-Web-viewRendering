use crate::{
    services::camisas::dao_camisa::get_dao_camisa, dao::abstract_traits::db_methods::DbMethods
};


pub fn count_camisas() -> usize{
    let dao = &mut get_dao_camisa();
    return dao.count().expect("count_camisas");
}