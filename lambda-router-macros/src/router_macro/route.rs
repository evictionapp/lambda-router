use lazy_regex::regex;
use proc_macro2::Span;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Error, Lit, LitStr, Result,
};

#[derive(Debug)]
pub struct Route {
    raw_route: String,
    route_no_slash: String,
    span: Span,
}

impl Parse for Route {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: Lit = input.parse()?;

        let str_lit = match lit {
            Lit::Str(str_lit) => str_lit,
            _ => {
                return Err(Error::new(lit.span(), "expected string literal for route"));
            }
        };
        let route_string = str_lit.value();

        let re = regex!(r#"\/"#);
        let no_slashes = re.replace_all(route_string.as_str(), "");

        Ok(Self {
            route_no_slash: no_slashes.to_string(),
            raw_route: route_string,
            span: str_lit.span(),
        })
    }
}

impl Route {
    pub fn lint(&self) -> Result<()> {
        let re = regex!(r#"^\/[a-zA-Z0-9_\-]+$"#);

        let err_msg = "invalid route, routes must start with a leading slash \"/\" and only consist of 0-9, a-z, A-Z, _, - ";

        if !re.is_match(self.raw_route.as_str()) {
            return Err(Error::new(self.span, err_msg));
        }

        Ok(())
    }
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let str_lit = LitStr::new(self.route_no_slash.as_str(), self.span);

        let quoted = quote! {
            #str_lit
        };

        tokens.append_all(quoted);
    }
}
