
//use axum::{ routing::{get, post, put}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::{path::Path,  FromRequest}};
//use axum::{ routing::{get, post, put}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::{path::Path,  FromRequest}};
use axum::Extension;
use tokio::{ sync::Mutex};
//use std::{net::SocketAddr, sync::Arc};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
//use serde_json::to_string;
//use tower_http::cors::{Any, CorsLayer, AllowMethods};
//use tower::ServiceBuilder;
use std::collections::HashMap;
//use core::mem::size_of;
use sqlx::postgres::PgPoolOptions;





#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);
 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub content: String,
    pub question_id: i32,
}

impl Answer{
    pub fn new(
        id: i32,
        content: String,
        question_id: i32,
    
    )-> Self{
        Answer {
            id,
            content,
            question_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)] //use the derive macro to implement the debug trait
pub struct Question {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)] //use the derive macro to implement the debug trait0
pub struct QuestionId(pub i32);
 
impl Question {
    pub fn new(
        id: i32, 
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


///Structure for loading and saving database.
/// #handles basic CRUD components
/// 
//#[derive(Clone)]
pub struct Store{
    questions: Mutex<HashMap<QuestionId, Question>>,
    answers: Mutex<HashMap<AnswerId, Answer>>,
}



    
/* 
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

 */
/* 
    pub async fn add_answer(
        store: Store,
        params: HashMap<String, String>,
    )-> Result<String, Error> {
        let answer = Answer {
            id: AnswerId("1".to_string()),
            content: params.get("content").unwrap().to_string(),
            question_id: QuestionId(
                params.get("questionId").unwrap().to_string()
            ),
        };

        store.answers.insert(answer.id.clone(), answer);
        

    }

*/