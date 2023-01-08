use deadpool_postgres::Pool;

pub struct AppState {
    pub pool: Pool,
}