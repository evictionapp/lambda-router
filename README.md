# lambda-router

`lambda-router` is a simple library to help with routing `http-api-gateway` requests to handlers in the same lambda that receive and return json.

# On json and url encoded data

- It is presumed that you are returning json in the form Result<T, E> where T & E impl Serialize.
- It is presumed that you are ok embracing `Externally tagged` representations for Result<T, E> from your api. [Read Serde Docs](https://serde.rs/enum-representations.html#externally-tagged).
- It is presumed that GET / DELETE requests get flattened data in the query string.
- It is presumed that POST / PUT requests get json or url encoded data in the body and the query string is ignored.
- It is presumed that you are ok explicitly specifying the unit type or some other type that has exactly one value as the input to your handler when you don't receive any data from the request.
- For now, you cannot access header values or other parts of the request. Feel free to make a pull request if you need a feature like this.

# Configuration on AWS

First make sure to create a `api gateway endpoint` that is **http**

> I am open to adding macro and library support for non http endpoints like lambda function urls but currently I do not use them.

# Adding your first handler

Get started by creating a handler using the `router` macro

`rust
use lambda_router::router;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Input {}

#[derive(Serialize)]
struct Output {}

// you may want to consider using "thiserror" and "serde_with" crate to handle errors
// and serialization when variants or other data structures in your error don't impl Serialize #[derive(Serialize)]
enum MyError {}

#[router(POST "/my_route")]
async fn my_route(input: Input) -> Result<Output, MyError> {
todo!();
}
`

# Creating the actual routing logic

inside your entry point to your lambda function that gets the request you use the `app` macro to automate writing the if statements that route requests to handlers
`rust
use lambda_http::{Body, Error, Request, Response};
// not_found is a fallback route that returns 404 and no body. it is provided for simple 404 responses, you can read about it below.
use lambda_router::{app, not_found};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
app! {
event,
my_route, #[default]
not_found,
}  
}
`

# 404 not found fallback

if you just want to return a simple 404 with no body when a request comes in that doesn't match anything, you can use the **not_found** pre-built default router used above. You can also impl your own like this:

`rust
use lambda_http::{Body, Error, Request, Response};

async fn my_custom_404(event: Request) -> Result<Response<Body>, Error> {
todo!();  
}
`
