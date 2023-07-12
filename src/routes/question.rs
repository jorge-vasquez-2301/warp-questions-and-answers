use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::Level;
use warp::hyper::StatusCode;

use crate::{
    profanity::check_profanity,
    store::Store,
    types::{extract_pagination, NewQuestion, Pagination, Question},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

#[tracing::instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        tracing::event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    Ok(store
        .get_questions(pagination.limit, pagination.offset)
        .await
        .map(|questions| warp::reply::json(&questions))?)
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = check_profanity(new_question.title).await?;
    let content = check_profanity(new_question.content).await?;
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    Ok(store
        .add_question(question)
        .await
        .map(|question| warp::reply::json(&question))?)
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = check_profanity(question.title).await?;
    let content = check_profanity(question.content).await?;
    let question = Question {
        id: question.id,
        title,
        content,
        tags: question.tags,
    };

    Ok(store
        .update_question(question, id)
        .await
        .map(|question| warp::reply::json(&question))?)
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(store
        .delete_question(id)
        .await
        .map(|id| warp::reply::with_status(format!("Question {} deleted", id), StatusCode::OK))?)
}
