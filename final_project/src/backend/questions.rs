
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
use sqlx::{FromRow,postgres::PgPoolOptions};





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

#[derive(Deserialize, Serialize, FromRow)]
pub struct QuestionAnswer {
    question_id: Option<i32>,
    question_content: Option<String>,
    title: Option<String>,
    answer_content: Option<String>,
}


///Structure for loading and saving database.
/// #handles basic CRUD components
/// #handles the relationship between answers and questions
#[derive(Clone)]
pub struct Store{
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl Store{

    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = "postgres://postgres:Password@localhost/final";
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .connect(database_url)
            .await?;
        Ok(Store {pool})
    }

    pub async fn get_pool(&self) -> sqlx::Pool<sqlx::Postgres> {
        self.pool.clone()
    }

    pub async fn add_question(&self, question: Question) -> Result<(), sqlx::Error>{
        let tags = question.tags.unwrap_or_else(Vec::new);
        sqlx::query!(
            "INSERT INTO questions (title, content, tags) 
            VALUES ($1, $2, $3)",
            question.title,
            question.content,
            &tags
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_table(
        &self,
        question: Question,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE questions
            SET title = $1, content = $2
            WHERE id = $3",
            question.title, question.content, question.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

        
    pub async fn delete_question(
        &self,
        question: Question,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM questions WHERE id = $1",
            question.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }


    pub async fn add_answer(
        &self, 
        answer: Answer
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO answers(id, content, question_id)
            VALUES ($1, $2, $3)",
            answer.id, answer.content, answer.question_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    


    pub async fn get_questions(
        &self,
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
        .fetch_all(&self.pool)
        .await?;
    
        Ok(result)
    }








    pub async fn create_table_questions(
        &self,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS questions(
                id SERIAL PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                content TEXT NOT NULL,
                tags TEXT[]
            )"
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    
    pub async fn create_table_answers(
        &self,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS answers (
                id SERIAL PRIMARY KEY,
                content TEXT NOT NULL,
                question_id INTEGER,
                FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE
            )"
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
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