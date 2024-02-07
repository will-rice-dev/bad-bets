use std::error::Error;

use axum::{response::Html, routing::get, Router};
use mongodb::{Client, options::ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _client = setup_mongodb().await?;
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn setup_mongodb() -> Result<Client, Box<dyn Error>> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://mongo-db:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("bad-bets".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(client)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}