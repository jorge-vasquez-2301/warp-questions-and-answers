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

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .update_question(question, id)
        .await
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
