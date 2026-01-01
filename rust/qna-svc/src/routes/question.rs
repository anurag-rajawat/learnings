use crate::error;
use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let start = pagination.start.min(res.len());
        let end = pagination.end.min(res.len());
        let res = &res[start..end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => {
            return Err(warp::reject::custom(error::Error::QuestionNotFound));
        }
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "Question deleted",
            StatusCode::NO_CONTENT,
        )),
        None => Err(warp::reject::custom(error::Error::QuestionNotFound)),
    }
}
