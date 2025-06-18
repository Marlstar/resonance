use diesel::SqliteConnection;

pub struct DBHandler {
    pub db: SqliteConnection,
}
impl DBHandler {
    pub fn new() -> crate::Result<Self> {
        return Ok(Self {
            db: crate::db::connect()?,
        });
    }
}

