use lambda_http::{Body, Error, Request, Response};

/// a simple 404 response when no routes are matched. provided as a convenience
/// you can see how to implement your own in the docs
pub async fn not_found(_: Request) -> Result<Response<Body>, Error> {
    let resp = lambda_http::Response::builder()
        .status(404)
        .body(().into())
        .map_err(Box::new)?;

    Ok(resp)
}
