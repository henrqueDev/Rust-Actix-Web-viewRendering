use diesel::{SqliteConnection, Table};

pub struct DAO<Tabela: Table> {
    pub connection: SqliteConnection,
    pub logs: Option<Vec<String>>,
    pub table: Tabela
}

