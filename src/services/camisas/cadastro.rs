use crate::{dao::abstract_traits::db_methods::DbMethods, models::camisas::camisa_dto::CamisaDTO, services::camisas::dao_camisa::get_dao_camisa};

pub fn cadastrar(name: &str, data_de_fabricacao: &str) -> String {  
    let dao = &mut get_dao_camisa();

    let name = String::from(name);
    let data_de_fabricacao = String::from(data_de_fabricacao);

    let obj = CamisaDTO { nome: name, data_fabricacao: data_de_fabricacao.to_owned() };
    let result = dao.insert(obj).expect("Deu erro");
    println!("{:?}", result);
    format!("Sucess!")
}

