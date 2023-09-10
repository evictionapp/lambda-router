use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error, Expr, Result, Token,
};

pub struct HandlerSet {
    set: Vec<Handler>,
}

impl HandlerSet {
    pub fn new() -> Self {
        HandlerSet { set: Vec::new() }
    }

    pub fn push(&mut self, handler: Handler) {
        self.set.push(handler);
    }

    pub fn default_handler(&mut self, span: Span) -> Result<Expr> {
        let pos = self.set.iter().position(|el| el.default).ok_or(Error::new(
            span,
            "expected at least one handler for default 404 fallback",
        ))?;

        let el = self.set.remove(pos);
        Ok(el.expr)
    }

    pub fn into_expr(self) -> Vec<Expr> {
        self.set.into_iter().map(|el| el.expr).collect()
    }
}

pub struct Handler {
    expr: Expr,
    pub default: bool,
}

impl Parse for Handler {
    fn parse(input: ParseStream) -> Result<Self> {
        let default = input
            .call(Attribute::parse_outer)
            .map(|attrs| {
                attrs
                    .into_iter()
                    .any(|attr| attr.path().is_ident("default"))
            })
            .unwrap_or_default();

        let expr: Expr = input.parse()?;

        if !input.is_empty() {
            let _: Token!(,) = input.parse()?;
        }

        Ok(Handler { expr, default })
    }
}
