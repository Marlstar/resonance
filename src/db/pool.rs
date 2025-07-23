use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::LazyLock;
use super::DBConn;

pub static POOL: LazyLock<Pool<ConnectionManager<SqliteConnection>>> = LazyLock::new(super::connect::pool);

pub fn get() -> DBConn {
    POOL.get().expect("failed to get DB connection from pool")
}
