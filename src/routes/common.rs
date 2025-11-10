pub mod pool {
    use crate::db::connection::DbPool;
    use warp::Filter;

    /// Clone DbPool cho mỗi request
    pub fn filter(pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pool.clone())
    }
}