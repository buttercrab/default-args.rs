//! Enables default arguments in rust by macro.
//!
//! Just wrap function with `default_args!` and macro with name of function
//! would be automatically generated to be used with default argument
//!
//! ```no_run
//! # extern crate default_args;
//! use default_args::default_args;
//!
//! // this would make a macro named `foo`
//! // and original function named `foo_`
//! default_args! {
//!     #[some_attribute]
//!     fn foo(important_arg: u32, optional: u32 = 100) -> String {
//!         // ...
//! #        format!("{}-{}", important_arg, optional)
//!     }
//! }
//!
//! // in other codes...
//! foo!(1); // foo(1, 100)
//! foo!(1, 3); // foo(1, 3)
//! foo!(1, optional=5); // foo(1, 5)
//! foo!(1, optional = 10); // foo(1, 10)
//! ```

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, token, Abi, Attribute, Block, Expr, FnArg, Generics, PatType,
    ReturnType, Token, Visibility,
};

struct Arg {
    pat: PatType,
    default: Option<(Token![=], Expr)>,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            pat: match input.parse::<FnArg>()? {
                FnArg::Receiver(r) => {
                    return Err(syn::Error::new(
                        r.span(),
                        "self in default_args! is not supported in this version",
                    ));
                }
                FnArg::Typed(pat) => pat,
            },
            default: {
                let equal  = input.parse()?;
                match equal {
                    Some(equal) => Some((equal, input.parse()?)),
                    None => None,
                }
            },
        })
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}

struct DefaultArgs {
    attrs: Vec<Attribute>,
    vis: Visibility,
    constness: Option<Token![const]>,
    asyncness: Option<Token![async]>,
    unsafety: Option<Token![unsafe]>,
    abi: Option<Abi>,
    fn_token: Token![fn],
    fn_name: Ident,
    generics: Generics,
    paren_token: token::Paren,
    args: Punctuated<Arg, Token![,]>,
    ret: ReturnType,
    body: Block,
}

impl Parse for DefaultArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let constness = input.parse()?;
        let asyncness = input.parse()?;
        let unsafety = input.parse()?;
        let abi = input.parse()?;
        let fn_token = input.parse()?;
        let fn_name = input.parse()?;


        let mut generics: Generics = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        let args = content.parse_terminated(Arg::parse)?;
        let ret = input.parse()?;
        generics.where_clause = input.parse()?;
        let body = input.parse()?;

        Ok(DefaultArgs {
            attrs,
            vis,
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            fn_name,
            generics,
            paren_token,
            args,
            ret,
            body,
        })
    }
}

impl ToTokens for DefaultArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}

#[proc_macro]
pub fn default_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DefaultArgs);
    let output = quote! { };
    output.into()
}

#[doc(hidden)]
mod compile_fail_test {
    /// using `self` in param is compile error for now
    ///
    /// error: `self in default_args! is not supported in this version`
    ///
    /// ```compile_fail
    /// # extern crate default_args;
    /// use default_args::default_args;
    ///
    /// struct A {}
    ///
    /// impl A {
    ///     default_args! {
    ///         fn hello(&self, a: usize, b: usize = 0) -> usize {
    ///             a + b
    ///         }
    ///     }
    /// }
    /// ```
    fn self_compile_error() {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {}
}
