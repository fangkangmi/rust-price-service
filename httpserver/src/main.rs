mod handler;
mod router;
mod server;
use server::Server;
use postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let conn_str = env::var("DB_IP").unwrap_or_else(|_| "postgres://postgres:mysecretpassword@localhost:5432/test_db".to_string());

    println!("Attempting to connect to the database...");

    let client = match Client::connect(&conn_str, NoTls) {
        Ok(client) => {
            println!("Successfully connected to the database.");
            client
        },
        Err(e) => {
            println!("Failed to connect to the database: {}", e);
            return Err(Box::new(e));
        }
    };

    let server = Server::new("localhost:8081");
    server.run();

    // Close the database connection
    client.close()?;
    println!("Database connection closed.");

    Ok(())
}