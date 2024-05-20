//minimal CRUD rest API using axum
// programmer: Devon Bedenbaugh
// help credits go to chatgpt, rustdocs, axum docs, bastian gruber rust textbook, 
#![warn(
    clippy::all,
)]
mod questions;
//mod error;

use axum::{ routing::{get, post, put, delete}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::{Form,Extension, rejection::ExtensionRejection}, response::{Json,Html}, response::IntoResponse, extract::{path::Path,  FromRequest}};
use tokio::{net::TcpListener, sync::Mutex};
use std::{net::SocketAddr, sync::Arc};


//use std::io::{Error, ErrorKind};
//use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::to_string;
//use hyper::body::to_bytes;

//use tower_http::{trace::TraceLayer};
use tower_http::cors::{Any, CorsLayer, AllowMethods};
use tower::ServiceBuilder;
use std::collections::HashMap;
//CRUD, create, read, update, delete
use core::mem::size_of;

use crate::questions::{Question, QuestionId, Store, Answer, AnswerId};
use sqlx::{FromRow,postgres::PgPoolOptions};
use reqwest::{Client, Error as ReqwestError};
use reqwest_middleware::Error as MiddlewareReqwestError;
use tracing::{event, Level, instrument};
use thiserror::Error;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot parse parameter: {0}")]
    ParseError(std::num::ParseIntError),
    #[error("Missing parameter")]
    MissingParameters,
    #[error("Cannot update, invalid data.")]
    DatabaseQueryError,
    #[error("External API error: {0}")]
    ReqwestAPIError(ReqwestError),
    #[error("External Middleware API error: {0}")]
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    #[error("External Client error: {0}")]
    ClientError(APILayerError),
    #[error("External Server error: {0}")]
    ServerError(APILayerError),
}

async fn transform_error(
    res: reqwest::Response
) -> APILayerError{
    APILayerError{
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}


#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

//impl Reject for Error {}
//impl Reject for APILayerError {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::ParseError(_) => (
                StatusCode::BAD_REQUEST,
                Json(APIResponse { message: self.to_string() }),
            ).into_response(),
            Error::MissingParameters => (
                StatusCode::BAD_REQUEST,
                Json(APIResponse { message: self.to_string() }),
            ).into_response(),
            Error::DatabaseQueryError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(APIResponse { message: self.to_string() }),
            ).into_response(),
            Error::ReqwestAPIError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(APIResponse { message: "Internal Server Error".into() }),
            ).into_response(),
            Error::MiddlewareReqwestAPIError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(APIResponse { message: "Internal Server Error".into() }),
            ).into_response(),
            Error::ClientError(err) => (
                StatusCode::BAD_REQUEST,
                Json(APIResponse { message: err.to_string() }),
            ).into_response(),
            Error::ServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(APIResponse { message: err.to_string() }),
            ).into_response(),
        }
    }
}


#[instrument]
pub async fn return_error(err: Error) -> Result<impl IntoResponse, Error> {
    event!(Level::ERROR, "{}", err);
    Ok(err.into_response())
}






#[derive(Deserialize, Debug)]
struct FilterResponse{
    censored_content: String,
}

#[derive(Deserialize)]
struct TextContent {
    text: String,
}


async fn filter_bad_words(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("BAD_WORDS_API_KEY").expect("BAD_WORDS_API_KEY not set properly.");
    let client = Client::new();
    let response = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body(serde_json::json!({ "text": text }).to_string())
        //.json(&serde_json::json!({ "text": text }))
        .send()
        .await?;
    
    let filter_response: FilterResponse = response.json().await?;
    println!("bout to return {:?}", filter_response.censored_content);
    let inner_content: TextContent = serde_json::from_str(&filter_response.censored_content)?;
    Ok(inner_content.text)
}

#[derive(Debug)]
struct InvalidId;

#[derive(Deserialize)]
pub struct FormData {
    title: String,
    content: String,
    tags: String,
}

#[derive(Deserialize)]
pub struct AnswerFormData{
    content: String,
    question_id: i32,
}


async fn get_handler(
    Extension(store): Extension<Store>,
) -> impl IntoResponse {
    match store.get_questions().await {
        Ok(questions) => {
            let json = serde_json::to_string(&questions).unwrap_or_else(|_| "[]".to_string());

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(json)
                .unwrap()
        }
        Err(err) => {
            let error_message = format!("Error fetching questions: {}", err);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_message)
                .unwrap()
        }
    }
}

async fn form_handler() -> impl IntoResponse {
    let html_contents = tokio::fs::read_to_string("src/post.html").await.unwrap();
    Html(html_contents)
}



async fn post_handler(
    Form(form_data): Form<FormData>,
    Extension(store): Extension<Store>,
) -> impl IntoResponse {
    
    let filtered_content = match filter_bad_words(&form_data.content).await {
        Ok(clean_content) => clean_content,
        Err(_) => return Html("<h1>Error filtering content</h1>"),
    };
    
    
    let tags_vec: Vec<String> = form_data.tags.split(',').map(|s| s.trim().to_string()).collect();
    let new_question = Question {
        id: 1,
        title: form_data.title,
        content: filtered_content,
        tags: Some(tags_vec),
    };
    match store.add_question(new_question).await {
        Ok(_) => Html("<h1>Question added successfully!</h1>"),
        Err(_) => Html("<h1>Error adding question</h1>"),
    }
}


async fn update_handler(
    Form(form_data): Form<FormData>,
    Extension(store): Extension<Store>,
) -> impl IntoResponse{
    let tags_vec: Vec<String> = form_data.tags.split(',').map(|s| s.trim().to_string()).collect();
    let new_question = Question {
        id: 1,
        title: form_data.title,
        content: form_data.content,
        tags: Some(tags_vec),
    };
    match store.update_table(new_question).await {
        Ok(_) => Html("<h1>Question added successfully!</h1>"),
        Err(_) => Html("<h1>Error adding question</h1>"),
    }

}

async fn delete_handler(
    Form(form_data): Form<FormData>,
    Extension(store): Extension<Store>,
) -> impl IntoResponse{
    let tags_vec: Vec<String> = form_data.tags.split(',').map(|s| s.trim().to_string()).collect();
    let new_question = Question {
        id: 1,
        title: form_data.title,
        content: form_data.content,
        tags: Some(tags_vec),
    };
    match store.delete_question(new_question).await {
        Ok(_) => Html("<h1>Question added successfully!</h1>"),
        Err(_) => Html("<h1>Error adding question</h1>"),
    }

}

async fn answer_form_handler() -> impl IntoResponse {
    let html_contents = tokio::fs::read_to_string("src/answers_form.html").await.unwrap();
    Html(html_contents)
}

async fn answer_handler(
    Form(form_data): Form<AnswerFormData>,
    Extension(store): Extension<Store>,
) -> impl IntoResponse {
    let new_answer = Answer {
        id: 1,
        content: form_data.content,
        question_id: form_data.question_id,
    };
    match store.add_answer(new_answer).await {
        Ok(_) => Html("<h1>Answer added successfully!</h1>"),
        Err(_) => Html("<h1>Error adding answer</h1>"),
    }
}

#[tokio::main]
async fn main() {

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv::dotenv().ok();
    println!("Using database URL: {}", std::env::var("DATABASE_URL").unwrap());



    let store = Store::new().await.expect("Failed to create store");

    if let Err(e) = store.create_table_questions().await {
        eprintln!("Failed to create questions table: {}", e);
    }
    if let Err(e) = store.create_table_answers().await {
        eprintln!("Failed to create answers table: {}", e);
    }
   

    let app = Router::new()

    .route("/", get(form_handler).post(form_handler).put(form_handler).delete(form_handler))
    .route("/questions/submit", get(post_handler).post(post_handler))
    .route("/questions", get(get_handler))
    .route("/questions/update", get(update_handler).put(update_handler))
    .route("/questions/delete", get(delete_handler).delete(delete_handler))
    .route("/answers", get(answer_form_handler).post(answer_form_handler))
    .route("/answers/submit", get(answer_handler).post(answer_handler))

    
    .layer(
        ServiceBuilder::new()
            .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
            //.layer(Extension(store_filter.clone()))
            .layer(Extension(store))
    );



    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}/", addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
}
