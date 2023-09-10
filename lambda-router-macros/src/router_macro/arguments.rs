use syn::{
    parse::{Parse, ParseStream},
    Result,
};

use super::{method::Method, route::Route};

#[derive(Debug)]
pub struct Arguments {
    pub method: Method,
    pub route: Route,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let method: Method = input.parse()?;
        let route: Route = input.parse()?;

        route.lint()?;

        Ok(Self { method, route })
    }
}
