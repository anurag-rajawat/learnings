use crate::types::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
