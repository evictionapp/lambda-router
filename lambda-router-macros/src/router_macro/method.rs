use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Error, Ident, Result,
};

#[derive(Debug)]
pub enum MethodType {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug)]
pub struct Method {
    method_type: MethodType,
    span: Span,
}

impl Parse for Method {
    fn parse(input: ParseStream) -> Result<Self> {
        let method_ident: Ident = input.parse()?;
        let span = method_ident.span();

        let method_type = match method_ident.to_string().as_str() {
            "GET" => MethodType::Get,
            "POST" => MethodType::Post,
            "PUT" => MethodType::Put,
            "DELETE" => MethodType::Delete,
            _ => {
                return Err(Error::new(
                    method_ident.span(),
                    "invalid method. only GET, POST, PUT, DELETE are supported.",
                ));
            }
        };

        Ok(Self { method_type, span })
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let upper = match self.method_type {
            MethodType::Get => "GET",
            MethodType::Post => "POST",
            MethodType::Put => "PUT",
            MethodType::Delete => "DELETE",
        };

        let ident = Ident::new(upper, self.span);

        tokens.append(ident);
    }
}
