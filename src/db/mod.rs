pub type SingleDBConn = diesel::SqliteConnection;
pub type DBConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;
pub mod schema;

pub mod pool;
pub use pool::POOL;

mod setup;
pub use setup::run_migrations as setup;

pub mod connect;
