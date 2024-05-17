use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    Client,
    Database
};

// Database Configuration
pub async fn database_config() -> Database {
    dotenv().ok();

    // Get Mongo URI
    let uri: String = match env::var("MONGO_URI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable")
    };

    // Get Mongo DBNAME
    let db_name: String = match env::var("MONGO_DBNAME") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable")
    };

    // Client
    let client = Client::with_uri_str(uri)
        .await
        .expect("error connecting to database");

    // DB Connection
    return client.database(db_name.as_str());
}
