//minimal web server using axum, help credits go to chatgpt and some axum documentation
use axum::{routing::get, Router};
use std::net::SocketAddr;
//use axum::Server;

#[tokio::main]
async fn main() {
    // Define a route handler for GET requests to "/"
    async fn hello_world() -> &'static str {
        "Hello, World!"
    }

    // Create an Axum router with a single route
    let app = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // Specify the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    // Start the Axum server
    axum::serve(listener, app).await.unwrap();
}

/* 
use warp::Filter;
//A minimal Rust HTTP server with Warp
#[tokio::main]
async fn main() {
    let hello = warp::get()
        .map(|| format!("Hello, World!"));
 
    warp::serve(hello)
        .run(([127, 0, 0, 1], 1337))
        .await;
}

//Example async HTTP call
//requires Tokio and Reqwest crates added to cargo toml

use std::collections::HashMap;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https:/ /httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

*/