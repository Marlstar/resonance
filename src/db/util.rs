use diesel::prelude::*;

pub fn connect() -> Result<SqliteConnection, diesel::ConnectionError> {
    let db_path = crate::dirs().db().display().to_string();
    return SqliteConnection::establish(&db_path);
}
