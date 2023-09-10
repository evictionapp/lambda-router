use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod app_macro;
mod router_macro;

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

#[proc_macro]
pub fn app(input: TokenStream) -> TokenStream {
    let lambda_app = parse_macro_input!(input as app_macro::lambda_app::LambdaApp);

    quote! {
        #lambda_app
    }
    .into()
}
