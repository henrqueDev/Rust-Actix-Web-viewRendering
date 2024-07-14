use std::fmt::Error;


use crate::dao::abstract_structures::paginated_query::PaginatedQuery;



pub trait DbMethods<T, U> {
    fn insert(self: &mut Self, obj: U) -> Result<usize, Error>;
    /*fn update<T, U>(self: &Self, obj: Entity<T, U>, new_obj: Entity<T, U>) -> Result<Entity<T, U>, Error>;
    fn remove<T, U>(self: &Self, obj: Entity<T, U>) -> Result<bool , Error>;
    fn get_by_id<T, U>(self: &Self, id: u64) -> Result<Entity<T, U>, Error>;*/
    fn get_all(self: &mut Self, page: i64, per_page: i64) -> Result<PaginatedQuery<T>, Error>;
    
    fn count(self: &mut Self) -> Result<usize, Error>;
}