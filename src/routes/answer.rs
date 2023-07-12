use warp::hyper::StatusCode;

use crate::{profanity::check_profanity, store::Store, types::NewAnswer};

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let content = check_profanity(new_answer.content).await?;

    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    Ok(store
        .add_answer(answer)
        .await
        .map(|_| warp::reply::with_status("Answer added", StatusCode::OK))?)
}
