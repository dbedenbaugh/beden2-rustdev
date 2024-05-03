//minimal CRUD rest API using axum
// programmer: Devon Bedenbaugh
// help credits go to chatgpt, rustdocs, axum docs, bastian gruber rust textbook, 
#![warn(
    clippy::all,
)]
mod questions;
mod error;

use axum::{ routing::{get, post, put, delete}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::{path::Path,  FromRequest}};
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

#[derive(Debug)]
struct InvalidId;




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
        QuestionId::from_str("2").expect("No id provided"),
        "4nd Question".to_string(),
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
            eprintln!("Error posting JSON: ");

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
    Path(answer_id): Path<AnswerId>,
    

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


#[tokio::main]
async fn main() {


    let store = Store::load();
    let store_filter = Arc::new(Mutex::new(store));


    let app = Router::new()

    .route("/questionspost", get(post_hr).post(post_hr))
    .route("/questions", get(get_hr))
    .route("/questions/:questionId", put(update_hr))
    .route("/questions/:questionId", delete(delete_hr))
    .route("/answers", post(answer_hr))

    
    .layer(
        ServiceBuilder::new()
            .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
            .layer(Extension(store_filter.clone()))
    );



    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);


    if let Err(e) = axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await{
        eprintln!("failed to start server: {}", e);
    }
}

    /* 
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    */

/* 

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

async fn set_questions(
    req: Request<Body>) -> Result<Response<Body>, hyper::Error>{
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
*/


/*

    let question = Question::new(
        QuestionId::from_str("4").expect("No id provided"),
        "4nd Question".to_string(),
        "content, question4".to_string(),
            Some(vec!("new".to_string())),  //encapsulate and create a vector
        );

    let question = Question::new(
        QuestionId::from_str("2").expect("No id provided"),
        "4nd Question".to_string(),
        "content, question3".to_string(),
            Some(vec!("faq".to_string())),  //encapsulate and create a vector
        );

        let question = Question::new(
        QuestionId::from_str("3").expect("No id provided"),
        "3nd Question".to_string(),
        "content, question3".to_string(),
            Some(vec!("new".to_string())),  //encapsulate and create a vector
        );

*/