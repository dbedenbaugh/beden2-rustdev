//minimal web server using axum, help credits go to chatgpt and some axum documentation
use axum::{routing::get, Router, response::Json, response::IntoResponse, extract::FromRequest};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use serde_json::json;
//mod question;

use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::Serialize;



#[derive(Serialize, Debug)] //use the derive macro to implement the debug trait
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Debug)] //use the derive macro to implement the debug trait0
struct QuestionId(String);
 
impl Question {
    fn new(
        id: QuestionId, 
        title: String, 
        content: String, 
        tags: Option<Vec<String>>
     ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;
 
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
              Error::new(ErrorKind::InvalidInput, "No id provided")
            ),
        }
    }
}




async fn get_questions() -> impl IntoResponse{
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );

    //Serialize the question to JSON
    Json(question)
}


async fn handle_request() -> Result<impl IntoResponse, std::convert::Infallible> {
    let response = get_questions().await;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_request));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}



/* 
#[tokio::main]
async fn main() {
    // Create an Axum router with a single route
    let app = Router::new().route("/", get(get_questions));
    // Specify the address to listen on
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("Failed to bind to address");
    // Start the Axum server
    //axum::serve(listener, app).await.unwrap();
    axum_server::bind("127.0.0.1:3000".parse().unwrap())
    //axum_server::bind(listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


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

/* 
async fn get_questions() -> Result<Json<Question>, axum::Error>{
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );

    //Serialize the question to JSON
    Ok(Json(question))
}
*/