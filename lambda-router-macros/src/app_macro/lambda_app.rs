use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Result, Token,
};

use super::handler::HandlerSet;

pub struct LambdaApp {
    event: Ident,
    handlers: Vec<Expr>,
    default_handler: Expr,
}

impl Parse for LambdaApp {
    fn parse(input: ParseStream) -> Result<Self> {
        let event: Ident = input.parse()?;
        let _: Token!(,) = input.parse()?;

        let mut handler_set = HandlerSet::new();
        while !input.is_empty() {
            let handler = input.parse()?;
            handler_set.push(handler);
        }

        let default_handler = handler_set.default_handler(input.span())?;
        let handlers = handler_set.into_expr();

        Ok(Self {
            event,
            handlers,
            default_handler,
        })
    }
}

impl ToTokens for LambdaApp {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let event = &self.event;
        let handlers = &self.handlers;
        let default_handler = &self.default_handler;

        let output = quote! {
            #( if let Some(r) = #handlers(& #event).await? {
                return Ok(r);
            })*
            #default_handler (#event) .await
        };

        tokens.append_all(output);
    }
}
