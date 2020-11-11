#![feature(trace_macros)]

trace_macros!(false);

use quote::quote;
use syn::parse_macro_input;
use syn::Token;

fn parse_path(tokens: syn::parse::ParseStream<'_>) -> syn::parse::Result<Vec<syn::Ident>> {
    let mut vars: Vec<syn::Ident> = vec![];

    tokens.parse::<Token![/]>()?; // remove leading slash

    // let path : syn::punctuated::Punctuated<, syn::token::Div> = tokens.parse_terminated(parser)?; // either a pair
    while !tokens.is_empty() {
        let lookahead = tokens.lookahead1();
        if lookahead.peek(Token![/]) {
            tokens.parse::<Token![/]>()?; // remove forward slashes
        } else if lookahead.peek(Token![:]) {
            // if the next thing is an ident we add it to vars
            tokens.parse::<Token![:]>()?; // the token
            let lookahead = tokens.lookahead1();
            if lookahead.peek(syn::Ident) {
                vars.push(tokens.parse()?);
            }
        } else if lookahead.peek(syn::Ident) {
            tokens.parse::<syn::Ident>()?;
        } else {
            return Err(syn::Error::new(tokens.span(), "Invalid path"));
        }
    }

    Ok(vars)
}

struct Route {
    // the connection or thing that evals to connection
    conn: syn::Expr,
    // method or thing that evals to a Method
    method: syn::Expr,
    // path that is a string literal
    path: syn::LitStr,
    // the variables in the path
    vars: Vec<syn::Ident>,
    // function thing to call
    func: syn::Expr, // might have to do better than this but will test later
}

impl syn::parse::Parse for Route {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let conn: syn::Expr = input.parse()?;
        input.parse::<syn::token::Comma>()?;

        let method: syn::Expr = input.parse()?;
        input.parse::<syn::token::Comma>()?;

        let path: syn::LitStr = input.parse()?;
        input.parse::<syn::token::Comma>()?;

        let func: syn::Expr = input.parse()?;

        let vars = path.parse_with(parse_path)?;

        Ok(Self {
            conn,
            method,
            path,
            vars,
            func,
        })
    }
}

impl Into<proc_macro::TokenStream> for Route {
    fn into(self) -> proc_macro::TokenStream {
        let Self {
            conn,
            method,
            path,
            vars,
            func,
        } = self;
        // This is where the output happens
        quote!({
            if #conn.match_method(#method) {
                if let Some(mut vars) = #conn.match_path(#path) {
                    #(let #vars = vars.next().unwrap().as_str().parse();)* // TODO: try_into is an issue from_str is probably a better choice
                    if [#(&#vars)*].iter().all(|f| f.is_ok()) {
                        let func_expr = #func;
                        func_expr(#conn, #(#vars.unwrap())*);
                        return
                    }
                }
            }
        })
        .into()
    }
}

#[proc_macro]
pub fn route(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: Route = parse_macro_input!(input);
    input.into()
}
