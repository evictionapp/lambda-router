use lambda_http::ext::PayloadError;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

#[serde_as]
#[derive(Debug, Error, Serialize)]
pub enum Error {
    /// an error occurred when trying to serialize from a json body payload
    #[error("{0}")]
    SerializeJSON(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        serde_json::Error,
    ),

    /// an error occurred when trying to deserialize from a url encoded query string
    #[error("{0}")]
    DeserializeURLEncoded(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        serde_urlencoded::de::Error,
    ),

    /// an error occurred when trying to deserialize from the request body
    #[error("{0}")]
    Payload(#[serde_as(as = "DisplayFromStr")] PayloadError),

    /// the route did not match what was expected
    #[error("not found")]
    NotFound,

    /// the method did not match what was expected
    #[error("method not allowed")]
    MethodNotAllowed,

    /// there was no payload sent bt all handlers are expected to get data, if you want to get no data, use the "unit" type of that data format, i.e in json this is null or use an empty struct
    #[error("no payload")]
    NoPayload,
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            _ => 400,
        }
    }

    pub fn json(&self) -> Result<String, Self> {
        let err: Result<(), &Self> = Err(self);
        Ok(serde_json::to_string(&err)?)
    }
}
