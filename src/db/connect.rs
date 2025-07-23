use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

/// Connect to the local sqlite database
pub fn single() -> Result<SqliteConnection, diesel::ConnectionError> {
    let db_path = crate::dirs::DATABASE.display().to_string();
    return SqliteConnection::establish(&db_path);
}

pub fn pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let db_path = crate::dirs::DATABASE.display().to_string();
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager);
    match pool {
        Ok(a) => return a,
        Err(e) => {
            println!("[ERROR] could not build connection pool");
            println!("        {e:?}");
            println!("exiting");
            std::process::exit(2);
        }
    }
}
