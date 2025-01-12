use std::env;

use dotenv::dotenv;
use mongodb::Database;
use tokio::sync::OnceCell;
use wither::mongodb;

static CONNECTION: OnceCell<Database> = OnceCell::const_new();

pub async fn connection() -> &'static Database {
    CONNECTION
        .get_or_init(|| async {
            dotenv().ok();

            let db_uri = env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");
            let db_name = env::var("DATABASE_NAME").expect("missing DATABASE_NAME env variable");

            mongodb::Client::with_uri_str(&db_uri)
                .await
                .expect("Failed to initialize MongoDB connection")
                .database(&db_name)
        })
        .await
}
