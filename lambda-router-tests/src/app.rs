use lambda_router::{app, not_found};

use crate::router::my_route;

pub async fn my_app(
    event: lambda_http::Request,
) -> Result<lambda_http::Response<lambda_http::Body>, lambda_http::Error> {
    app! {
        event,
        my_route,
        #[default]
        not_found,
    }
}
