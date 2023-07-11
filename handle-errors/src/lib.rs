use tracing::Level;
use warp::reject::Reject;
use warp::{body::BodyDeserializeError, cors::CorsForbidden, hyper::StatusCode};
use warp::{Rejection, Reply};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::DatabaseQueryError => {
                write!(f, "Query could not be executed")
            }
        }
    }
}

impl Reject for Error {}

#[tracing::instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError) = r.find() {
        tracing::event!(Level::ERROR, "Database query error");
        Ok(warp::reply::with_status(
            Error::DatabaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        tracing::event!(Level::ERROR, "CORS forbidden error: {error}");
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        tracing::event!(Level::ERROR, "Cannot deserizalize request body: {error}");
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        tracing::event!(Level::ERROR, "{error}");
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        tracing::event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
