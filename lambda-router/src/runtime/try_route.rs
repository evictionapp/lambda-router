//! # try_route
//! "tries" to match a route and method to a request, also tries to deserialize inputs if the routes and methods check out.

use lambda_http::{http::Method, RequestExt, RequestPayloadExt};
use serde::de::DeserializeOwned;

use super::error::Error;

/// attemps to match an inbound route to a route given by a handler
/// it is the runtime version of routing for the app! macro
/// it simply checks if the routes are equal as str
/// if they match it then checks that the methods are the same
/// from there it desererializes data either from json or url encoded
/// based on the method, if all goes well, it returns the deserialized type "T"
/// which is later passed to the handler you write
pub fn try_route<T: DeserializeOwned>(
    event: &lambda_http::Request,
    method: lambda_http::http::Method,
    route: &str,
    inbound: Option<&str>,
) -> Result<T, Error> {
    if inbound.map(|el| el != route).unwrap_or_default() {
        return Err(Error::NotFound);
    }
    if event.method() != method {
        return Err(Error::MethodNotAllowed);
    }

    match event.method() {
        &Method::GET | &Method::DELETE => {
            let qs = event.query_string_parameters().to_query_string();
            Ok(serde_urlencoded::from_str(&qs)?)
        }
        _ => match event.payload() {
            Ok(Some(payload)) => Ok(payload),
            Ok(None) => Err(Error::NoPayload),
            Err(err) => Err(Error::Payload(err)),
        },
    }
}
