//minimal web server using axum, help credits go to chatgpt and some axum documentation
use axum::{ routing::{get, post}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::FromRequest};
use tokio::net::TcpListener;
use std::{net::SocketAddr, sync::Arc};
//mod question;

use std::io::{Error, ErrorKind};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use hyper::body::to_bytes;

//use tower_http::{trace::TraceLayer};
use tower_http::cors::{Any, CorsLayer, AllowMethods};
use tower::ServiceBuilder;
use std::collections::HashMap;
//CRUD, create, read, update, delete
use core::mem::size_of;

#[derive(Debug)]
struct InvalidId;

#[derive(Deserialize, Serialize, Debug, Clone)] //use the derive macro to implement the debug trait
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)] //use the derive macro to implement the debug trait0
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
#[derive(Clone)]
struct Store{
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    /* 
    fn init(self) -> Self {
        let question = Question::new(
            QuestionId::from_str("1").expect("Id not set"),
            "How?".to_string(),
            "Please help!".to_string(),
            Some(vec!["general".to_string()])
        );
        self.add_question(question)
    }
    */
    fn init()-> HashMap<QuestionId, Question>{
        let file = include_str!("questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }

    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }


    fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
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

    let file = include_str!("questions.json");
    println!("Contents of questions.json:");
    println!("{}", file);

    //let questions = serde_json::from_str(file).expect("Can't read questions.json");
    match serde_json::from_str::<Vec<Question>>(file){
        Ok(questions) => {
            let json_response = serde_json::to_string(&questions).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(json_response)
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

async fn set_questions(req: Request<Body>) -> Result<Response<Body>, hyper::Error>{
    println!("Set_questions");
    match(req.method(), req.uri().path()){
        (&Method::GET, "/set") => {
            //Serve the HTML form page
            println!("Set_questions, GET");

            let html_content = include_str!("set.html");
            //let html_content = "<form action='/set' method='post'><button type='submit'>Submit</button></form>";

            //println!("{}", html_content);
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type","text/html")
                .body(Body::from(html_content))
                .unwrap();
            Ok(response)

        }
        (&Method::POST, "/set") => {
            println!("Set_questions, POST");

            //let body_bytes = to_bytes(req.into_body()).await?;
            //let form_data: Result<Question, _> = serde_urlencoded::from_bytes(&body_bytes);
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = std::str::from_utf8(&whole_body).unwrap();
            let form_data: HashMap<String, String> = url::form_urlencoded::parse(body_str.as_bytes())
            .into_owned()
            .collect();

        // Printing out the form data to console
            for (key, value) in form_data.iter() {
                println!("{}: {}", key, value);
            }

            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("Form submitted successfully!"))
                .unwrap();

            Ok(response)
        }
            _ => {
                println!("Set_questions, _");

                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap())

            }
        }
    }




async fn get_hr() -> impl IntoResponse {
    //let response = get_questions().await;
    //Ok(response)
    println!("get_hr called");
    let response = get_questions().await;
    //Ok(response)
}

/* 
async fn set_hr(req: Request<Body>) -> impl IntoResponse {
    println!("set_hr called");

    set_questions(req).await
}*/


async fn set_hr(req: Request<Body>) -> Response<Body> {
    println!("set_hr called");
    match set_questions(req).await {
        Ok(response) => response,
        Err(e) => {
            // Log the error or handle it as needed
            eprintln!("Error handling request: {}", e);

            // Create an error response
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    }
}

#[tokio::main]
async fn main() {


    let store = Store::new();
    //let store_filter = warp::any().map(move || store.clone()
    let store_filter = axum::Extension(Arc::new(store.clone()));


    let app = Router::new()
    .route("/", get(get_hr))
    .route("/set", get(set_hr).post(set_hr))
    .layer(
        CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );


    //let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    //axum::serve(listener, app_with_cors)
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}


