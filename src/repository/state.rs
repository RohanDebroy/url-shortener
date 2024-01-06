use migration::{
    sea_orm::{ConnectOptions, Database, DatabaseBackend, DatabaseConnection, Statement},
    ConnectionTrait,
};
use std::env;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let (url, db_name) = get_path_url(&db_url);

        let db = Database::connect(url)
            .await
            .expect("Failed to establish a connection");

        match db.get_database_backend() {
            DatabaseBackend::Postgres => {
                let _ = db
                    .execute(Statement::from_string(
                        db.get_database_backend(),
                        format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
                    ))
                    .await;
                let _ = db
                    .execute(Statement::from_string(
                        db.get_database_backend(),
                        format!("CREATE DATABASE \"{}\";", db_name),
                    ))
                    .await;
            }
            DatabaseBackend::MySql => {}
            DatabaseBackend::Sqlite => {}
        };

        let mut connect_options = ConnectOptions::new(db_url);
        connect_options
            .max_connections(100)
            .min_connections(5)
            .sqlx_logging(true);

        let db = Database::connect(connect_options)
            .await
            .expect("Failed to connect to the database");

        AppState { db }
    }
}

fn get_path_url(url: &str) -> (&str, &str) {
    let x: Vec<_> = url.rsplitn(2, "/").collect();
    (x[1], x[0])
}
