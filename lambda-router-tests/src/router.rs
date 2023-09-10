use lambda_router::router;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

#[derive(Serialize)]
pub enum Error {}

#[router(GET "/my_route")]
pub async fn my_route(input: Input) -> Result<Output, Error> {
    println!("testing! {:?}", input);

    Ok(Output {})
}
