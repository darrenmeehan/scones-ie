use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::configuration::get_configuration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn connect() -> Pool<Manager<PgConnection>> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        configuration.database.username,
        configuration.database.password,
        configuration.database.host,
        configuration.database.port,
        configuration.database.database_name
    );
    let manager = deadpool_diesel::postgres::Manager::new(
        connection_string,
        deadpool_diesel::Runtime::Tokio1,
    );
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }
    pool
}
