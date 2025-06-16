use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::embed_migrations;
use diesel::Connection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations() {
    let url = format!("{}", crate::dirs::DATABASE.display());
    let mut connection = diesel::sqlite::SqliteConnection::establish(&url)
        .expect("failed to establish database connection");
    connection.run_pending_migrations(MIGRATIONS)
        .expect("failed to run database migrations");
}
