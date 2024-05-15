//minimal CRUD rest API using axum
// programmer: Devon Bedenbaugh
// help credits go to chatgpt, rustdocs, axum docs, bastian gruber rust textbook, 
#![warn(
    clippy::all,
)]
mod questions;
mod error;

use axum::{ routing::{get, post, put, delete}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::{Form,Extension}, response::{Json,Html}, response::IntoResponse, extract::{path::Path,  FromRequest}};
use tokio::{net::TcpListener, sync::Mutex};
use std::{net::SocketAddr, sync::Arc};


use std::io::{Error, ErrorKind};
use std::str::FromStr;

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



#[derive(Debug)]
struct InvalidId;

#[derive(Deserialize)]
struct FormData {
    title: String,
    content: String,
    tags: String,
}

#[derive(Deserialize)]
struct AnswerFormData{
    content: String,
    question_id: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
struct QuestionAnswer {
    question_id: Option<i32>,
    question_content: Option<String>,
    title: Option<String>,
    answer_content: Option<String>,
}

async fn create_db_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = "postgres://postgres:Password@localhost/final";
    PgPoolOptions::new()
        .max_connections(3)
        .connect(database_url)
        .await
}


async fn add_question(pool: &sqlx::Pool<sqlx::Postgres>, question: Question) -> Result<(), sqlx::Error>{
    let tags = question.tags.unwrap_or_else(Vec::new);
    sqlx::query!(
        "INSERT INTO questions (title, content, tags) 
        VALUES ($1, $2, $3)",
        question.title,
        question.content,
        &tags
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn get_questions(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> Result <Vec<QuestionAnswer>, sqlx::Error> {
    let result = sqlx::query_as!(
        QuestionAnswer,
        "
        SELECT 
            questions.id AS question_id, 
            questions.content AS question_content,
            questions.title, 
            answers.content AS answer_content
        FROM 
            questions 
        FULL JOIN 
            answers 
        ON 
            questions.id = answers.question_id
        "
        //Question,
        //"SELECT * FROM questions"
    )
    .fetch_all(&pool)
    .await?;

    Ok(result)
}

async fn create_table_questions(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS questions(
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            tags TEXT[]
        )"
    )
    .execute(&pool)
    .await?;
    Ok(())
}

async fn create_table_answers(
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS answers (
            id SERIAL PRIMARY KEY,
            content TEXT NOT NULL,
            question_id INTEGER,
            FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE
        )"
    )
    .execute(&pool)
    .await?;
    Ok(())
}

async fn add_answer(
    pool: &sqlx::Pool<sqlx::Postgres>, 
    answer: Answer
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO answers(id, content, question_id)
        VALUES ($1, $2, $3)",
        answer.id, answer.content, answer.question_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn delete_question(
    pool: &sqlx::Pool<sqlx::Postgres>,
    question: Question,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM questions WHERE id = $1",
        question.id
    )
    .execute(pool)
    .await?;
    Ok(())
}


async fn update_table(
    pool: &sqlx::Pool<sqlx::Postgres>,
    question: Question,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE questions
        SET title = $1, content = $2
        WHERE id = $3",
        question.title, question.content, question.id
    )
    .execute(pool)
    .await?;
    Ok(())
}


async fn get_handler(
    pool: Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    match get_questions(pool).await {
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
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let tags_vec: Vec<String> = form_data.tags.split(',').map(|s| s.trim().to_string()).collect();
    let new_question = Question {
        id: 1,
        title: form_data.title,
        content: form_data.content,
        tags: Some(tags_vec),
    };
    match add_question(&pool, new_question).await {
        Ok(_) => Html("<h1>Question added successfully!</h1>"),
        Err(_) => Html("<h1>Error adding question</h1>"),
    }
}


async fn update_handler(
    Form(form_data): Form<FormData>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse{
    let tags_vec: Vec<String> = form_data.tags.split(',').map(|s| s.trim().to_string()).collect();
    let new_question = Question {
        id: 1,
        title: form_data.title,
        content: form_data.content,
        tags: Some(tags_vec),
    };
    match update_table(&pool, new_question).await {
        Ok(_) => Html("<h1>Question added successfully!</h1>"),
        Err(_) => Html("<h1>Error adding question</h1>"),
    }

}

async fn delete_handler(
    Form(form_data): Form<FormData>,
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse{
    let tags_vec: Vec<String> = form_data.tags.split(',').map(|s| s.trim().to_string()).collect();
    let new_question = Question {
        id: 1,
        title: form_data.title,
        content: form_data.content,
        tags: Some(tags_vec),
    };
    match delete_question(&pool, new_question).await {
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
    Extension(pool): Extension<sqlx::Pool<sqlx::Postgres>>,
) -> impl IntoResponse {
    let new_answer = Answer {
        id: 1,
        content: form_data.content,
        question_id: form_data.question_id,
    };
    match add_answer(&pool, new_answer).await {
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

    let pool = match create_db_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            return;
        }
    };
    if let Err(e) = create_table_questions(Extension(pool.clone())).await {
        eprintln!("Failed to create questions table: {}", e);
    }
    if let Err(e) = create_table_answers(Extension(pool.clone())).await {
        eprintln!("Failed to create answers table: {}", e);
    }
   

    let app = Router::new()

    .route("/form", get(form_handler).post(form_handler).put(form_handler).delete(form_handler))
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
            .layer(Extension(pool.clone()))
    );



    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}/form", addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
}

/* 

///GET Handler


async fn get_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
) -> impl IntoResponse{
    let mut store = store.lock().await;

    println!("get_hr called");

    match store.get_questions_json().await {

        Ok(questions) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(questions)
                .unwrap()
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);

            let error_response = "Error fetching and parsing questions".to_string();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_response)
                .unwrap()
        }
    }
}



///POST Handler

 
async fn post_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    //Json(question): Json<Question>
)   -> impl IntoResponse {
    println!("post_hr called");
    let mut store = store.lock().await;
    let new_question = Question::new(
        QuestionId::from_str("3").expect("No id provided"),
        "3nd Question".to_string(),
        "content, question3".to_string(),
            Some(vec!("faq".to_string())),  //encapsulate and create a vector
        );
    match store.add_question(new_question).await{
    //(StatusCode::CREATED, "Question added successfully.")
        Ok(_) => {
        Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body("Created successfully".to_string())
                    .unwrap()   
            }
        Err(err) => {
            //eprintln!("Error posting JSON: ");

            let error_response = "Error creating and submitting question".to_string();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_response)
                .unwrap()
            }
        }
}




///PUT Handler
async fn update_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    //Json(question): Json<Question>
    Path(question_id): Path<QuestionId>,
) -> impl IntoResponse {
    println!("update_hr called");

    let mut store = store.lock().await;
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question UPDATED".to_string(),
        "Content of question".to_string(),
            Some(vec!("faq".to_string())),  //encapsulate and create a vector
        );


    match store.update_question(&question_id, question).await {
        Ok(_) => Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "application/json")
                        .body("Question updated successfully.".to_string())
                        .unwrap(),
        Err(error) => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("content-type", "application/json")
                        .body(error.to_string())
                        .unwrap(),
    }


}

///DELETE Handler

async fn delete_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Path(question_id): Path<QuestionId>
) -> impl IntoResponse {
    let mut store = store.lock().await;
    match store.delete_question(&question_id).await {
        Ok(_) => (StatusCode::OK, "Question deleted successfully.".to_string()),
        Err(error) => (StatusCode::NOT_FOUND, error),
    }
}

async fn answer_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    //Path(answer_id): Path<AnswerId>,
    

) -> impl IntoResponse{
    let mut store = store.lock().await;

    let answer= Answer::new(
        AnswerId("1".to_string()),
        "test".to_string(),
        //params.get("content").unwrap().to_string(),
        QuestionId("101".to_string()),
    );


    match store.add_answer(answer).await{
        Ok(response) => {
            (StatusCode::OK, "answer added successfully.".to_string())
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, e)        }
    }

}

*/