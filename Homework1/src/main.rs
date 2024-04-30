//minimal CRUD rest API using axum
// programmer: Devon Bedenbaugh
// help credits go to chatgpt, rustdocs, axum docs, bastian gruber rust textbook, 


use axum::{ routing::{get, post, put}, body::{Body,Bytes},Router,http::{Request,Method, Response, StatusCode}, extract::Extension, response::Json, response::IntoResponse, extract::{path::Path,  FromRequest}};
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


///Structure for loading and saving database.
/// #handles basic CRUD components
/// 
#[derive(Clone)]
struct Store{
    questions: HashMap<QuestionId, Question>,
}

impl Store {

    fn init()-> HashMap<QuestionId, Question>{
        let file = include_str!("questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }

    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }
    
    fn load()-> Self {

        let file_contents = std::fs::read_to_string("src/questions.json").unwrap_or_else(|_| "{}".to_string());
        let questions: HashMap<QuestionId, Question> = serde_json::from_str(&file_contents).expect("Failed to parse JSON.");
        Store { questions }
    }
    fn save(&self) {
        let json = serde_json::to_string(&self.questions).expect("Failed to serialize questions.");
        std::fs::write("src/questions.json", json).expect("Failed to write to file.");
    }

    fn add_question(&mut self, question: Question)  {
        println!("{:?} {:?}", question.id, question);
        println!("{:?}", self.questions);
        self.questions.insert(question.id.clone(), question);
        self.save();
    }

    fn update_question(&mut self, question_id: &QuestionId, new_question: Question) -> Result<(), String> {
        if self.questions.contains_key(question_id) {
            self.questions.insert(question_id.clone(), new_question);
            self.save();
            Ok(())
        } else {
            Err("Question not found".to_string())
        }
    }

    fn delete_question(&mut self, question_id: &QuestionId) -> Result<(), String> {
        if self.questions.remove(question_id).is_some() {
            self.save();
            Ok(())
        } else {
            Err("Question not found".to_string())
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



///GET Handler


async fn get_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
) -> impl IntoResponse{
    let mut store = store.lock().await;

    println!("get_hr called");

    match to_string(&store.questions){

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
    let question = Question::new(
        QuestionId::from_str("2").expect("No id provided"),
        "4nd Question".to_string(),
        "content, question3".to_string(),
            Some(vec!("faq".to_string())),  //encapsulate and create a vector
        );
    store.add_question(question);
    //(StatusCode::CREATED, "Question added successfully.")
    Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body("Created successfully".to_string())
                .unwrap()    
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
    match store.update_question(&question_id, question) {
        Ok(_) =>(StatusCode::OK, "Question updated successfully.".to_string()),
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()),

    }


}

///DELETE Handler

async fn delete_hr(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Path(question_id): Path<QuestionId>
) -> impl IntoResponse {
    let mut store = store.lock().await;
    match store.delete_question(&question_id) {
        Ok(_) => (StatusCode::OK, "Question deleted successfully.".to_string()),
        Err(error) => (StatusCode::NOT_FOUND, error),
    }
}

#[tokio::main]
async fn main() {


    let store = Store::load();
    let store_filter = Arc::new(Mutex::new(store));


    let app = Router::new()

    .route("/questionspost", get(post_hr).post(post_hr))
    .route("/questions", get(get_hr))
    .route("/questionsup", put(update_hr).delete(delete_hr))
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