use yew::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QuestionAnswer {
    pub question_id: Option<i32>,
    pub question_content: Option<String>,
    pub title: Option<String>,
    pub answer_content: Option<String>,
}

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(Vec::new);

    {
        let questions = questions.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_questions: Vec<QuestionAnswer> = Request::get("http://127.0.0.1:3000/questions")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    questions.set(fetched_questions);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            <h1>{ "Questions and Answers" }</h1>
            <table>
                <thead>
                    <tr>
                        <th>{ "Question ID" }</th>
                        <th>{ "Title" }</th>
                        <th>{ "Question Content" }</th>
                        <th>{ "Answer Content" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for questions.iter().map(|qa| html! {
                        <tr>
                            <td>{ qa.question_id.unwrap_or_default() }</td>
                            <td>{ qa.title.clone().unwrap_or_default() }</td>
                            <td>{ qa.question_content.clone().unwrap_or_default() }</td>
                            <td>{ qa.answer_content.clone().unwrap_or_else(|| "No answer".to_string()) }</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>;
}
