use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod app_macro;
mod router_macro;

/// wraps your function in a new one that handles http requests and returns responses
///
/// # Syntax
///
/// the first thing to do is specify the method in all caps without quotes
/// #[router(GET)]
///
/// from there specify the path to the resource you want to bind to that is quoted & starts with a leading slash
/// #[router(GET "/my_route")]
///
///
/// # Examples
///
/// ```
/// #[router(GET "/my_route")]
/// pub async fn my_route(input: ()) -> Result<(), ()> {
///     todo!();
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn router(args: TokenStream, input: TokenStream) -> TokenStream {
    let arguments = parse_macro_input!(args as router_macro::arguments::Arguments);
    let item_fn = parse_macro_input!(input as ItemFn);

    let ast = router_macro::ast::Ast { item_fn, arguments };

    quote! {
        #ast
    }
    .into()
}

/// provides the entry point for lambdas to handle routing
///
/// # Syntax
/// app! { #EVENT,  ...#HANDLERS,  #\[default\] 404_HANDLER, ...#HANDLERS  }
///
/// you are not required to put #\[not_found\] for the 404 handler at the bottom but it is recommended
///
/// # Examples
///
/// ```
/// app! {
///     event,
///     my_route,
///     #[not_found]
///     not_found,
/// }
/// ```
#[proc_macro]
pub fn app(input: TokenStream) -> TokenStream {
    let lambda_app = parse_macro_input!(input as app_macro::lambda_app::LambdaApp);

    quote! {
        #lambda_app
    }
    .into()
}
