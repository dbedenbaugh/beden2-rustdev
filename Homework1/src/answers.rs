#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct AnswerId(String);
 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}