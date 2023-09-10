use quote::{quote, ToTokens, TokenStreamExt};
use syn::ItemFn;

use super::arguments::Arguments;

#[derive(Debug)]
pub struct Ast {
    pub item_fn: ItemFn,
    pub arguments: Arguments,
}

impl ToTokens for Ast {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item_fn = &self.item_fn;
        let vis = &self.item_fn.vis;
        let ident = &self.item_fn.sig.ident;

        let method = &self.arguments.method;
        let route = &self.arguments.route;

        let wrapped_func = quote! {
            #vis async fn #ident(event: &lambda_http::Request) -> Result<Option<lambda_http::Response<lambda_http::Body>>,lambda_http::Error> {
                #item_fn;

                use lambda_http::RequestExt;
                let path_params = event.path_parameters();
                let inbound = path_params.first("route");

                let method = lambda_http::http::Method:: #method;
                let route = #route;

                let (message, status_code) = match lambda_router::runtime::try_route::try_route(event, method, route, inbound) {
                    Ok(payload) => {
                        let result = #ident(payload).await;
                        let status_code = match result.is_ok() {
                            true => 200,
                            false => 400,
                        };
                        let message = lambda_router::runtime::json::json(&result)?;
                        (message, status_code)
                    },
                    Err(err) => {
                        if err.is_not_found() {
                            return Ok(None);
                        }
                        let status_code = err.status_code();
                        let serialized = err.json()?;
                        (serialized, status_code)
                    },
                };

                let resp = lambda_http::Response::builder()
                    .status(status_code)
                    .header("content-type", "application/json")
                    .body(message.into())
                    .map_err(Box::new)?;

                Ok(Some(resp))
            }
        };

        tokens.append_all(wrapped_func);
    }
}
