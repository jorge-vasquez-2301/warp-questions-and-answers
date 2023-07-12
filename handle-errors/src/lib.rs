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
    ExternalAPIError(reqwest::Error),
    ClientError(APILayerError),
    ServerError(APILayerError),
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
            Error::ExternalAPIError(err) => {
                write!(f, "Cannot execute: {}", err)
            }
            Error::ClientError(err) => {
                write!(f, "External Client error: {}", err)
            }
            Error::ServerError(err) => {
                write!(f, "External Server error: {}", err)
            }
        }
    }
}

impl Reject for Error {}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl Reject for APILayerError {}

#[tracing::instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError) = r.find() {
        tracing::event!(Level::ERROR, "Database query error");
        Ok(warp::reply::with_status(
            Error::DatabaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::ExternalAPIError(e)) = r.find() {
        tracing::event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ClientError(e)) = r.find() {
        tracing::event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ServerError(e)) = r.find() {
        tracing::event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
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
