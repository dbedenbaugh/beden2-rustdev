
//use axum::{ routing::{get, post, put}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::{path::Path,  FromRequest}};
//use axum::{ routing::{get, post, put}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::{path::Path,  FromRequest}};

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




#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);
 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

impl Answer{
    pub fn new(
        id: AnswerId,
        content: String,
        question_id: QuestionId,
    
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
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)] //use the derive macro to implement the debug trait0
pub struct QuestionId(pub String);
 
impl Question {
    pub fn new(
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


///Structure for loading and saving database.
/// #handles basic CRUD components
/// 
//#[derive(Clone)]
pub struct Store{
    questions: Mutex<HashMap<QuestionId, Question>>,
    answers: Mutex<HashMap<AnswerId, Answer>>,
}

impl Store {

    pub fn init()-> HashMap<QuestionId, Question>{
        let file = include_str!("questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }

    pub fn new() -> Self {
        Store {
            questions: Mutex::new(HashMap::new()),
            answers: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_questions_json(&self) -> Result<String, serde_json::Error> {

        let questions = self.questions.lock().await;
        serde_json::to_string(&*questions)
    }
    
   
    pub fn load() -> Self {
        let file_contents = std::fs::read_to_string("src/questions.json")
            .unwrap_or_else(|_| "{}".to_string());
        let questions: HashMap<QuestionId, Question> = serde_json::from_str(&file_contents)
            .expect("Failed to parse JSON.");
        
        let answer_contents = std::fs::read_to_string("src/answers.json")
            .unwrap_or_else(|_| "{}".to_string());
        let answers: HashMap<AnswerId, Answer> = serde_json::from_str(&answer_contents)
            .expect("Failed to parse JSON.");
        
        
        Store {
            questions: Mutex::new(questions),
            answers: Mutex::new(answers),
        }
    }

    pub async fn save(&self)  {
        let questions = self.questions.lock().await;
        let json = serde_json::to_string(&*questions).expect("Failed to serialize questions");
        //let json = serde_json::to_string(&self.questions).expect("Failed to serialize questions.");
        std::fs::write("src/questions.json", json).expect("Failed to write to file.");
        
    }

    pub async fn add_question(&mut self, question: Question) -> Result<(), String> {
        let mut questions = self.questions.lock().await;
        println!("{:?} {:?}", question.id, question);
        println!("{:?}", *questions);

        if questions.contains_key(&question.id) {
            Err("Question with this ID already exists".to_string())
        } else {
            questions.insert(question.id.clone(), question);
            self.save().await;
            "Successfully added question to db".to_string();
            Ok(())
        }

       // questions.insert(question.id.clone(), question);
      //  self.save().await;
    }

    pub async fn update_question(&mut self, question_id: &QuestionId, new_question: Question) -> Result<(), String> {
        let mut questions = self.questions.lock().await;

        if questions.contains_key(question_id) {
            questions.insert(question_id.clone(), new_question);
            self.save().await;
            Ok(())
        } else {
            Err("Question not found".to_string())
        }
    }

    pub async fn delete_question(&mut self, question_id: &QuestionId) -> Result<(), String> {
        let mut questions = self.questions.lock().await;
        if questions.remove(question_id).is_some() {
            self.save();
            Ok(())
        } else {
            Err("Question not found".to_string())
        }
    }

    pub async fn add_answer(
        &mut self,
        answer: Answer,
        
    )-> Result<String, String> {

        let mut answers = self.answers.lock().await;
        match answers.insert(answer.id.clone(), answer) {
            Some(_) => {
                // If Some, it means an old value was replaced.
                let json = serde_json::to_string(&*answers).expect("Failed to serialize answers");
                std::fs::write("src/answers.json", json).expect("Failed to write to file.");
                Ok("Answer updated successfully.".to_string())
            },
            None => {
                // If None, it means no value was previously associated with this key.
                let json = serde_json::to_string(&*answers).expect("Failed to serialize answers");
                std::fs::write("src/answers.json", json).expect("Failed to write to file.");
                Ok("Answer inserted successfully.".to_string())
            },
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



    
}
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