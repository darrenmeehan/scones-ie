use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn connect() -> Pool<Manager<PgConnection>> {
    let connection_string = std::env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set.");

    let manager = deadpool_diesel::postgres::Manager::new(connection_string, deadpool_diesel::Runtime::Tokio1);
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
