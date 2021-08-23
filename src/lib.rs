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
use proc_macro2::Punct;
use proc_macro2::{Ident, Spacing, Span};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, token, Abi, Attribute, Block, Expr, FnArg, Generics, PatType,
    ReturnType, Token, Visibility,
};

struct Args {
    parsed: Punctuated<PatType, Token![,]>,
    required: usize,
    optional: Vec<(PatType, Expr)>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Punctuated::new();
        let mut has_optional = false;
        let mut required = 0;
        let mut optional = Vec::new();

        while !input.is_empty() {
            let fn_arg = input.parse::<FnArg>()?;

            let pat = match fn_arg {
                FnArg::Receiver(r) => {
                    return Err(syn::Error::new(
                        r.span(),
                        "self in default_args! is not support in this version",
                    ));
                }
                FnArg::Typed(pat) => pat,
            };

            if input.parse::<Option<Token![=]>>()?.is_some() {
                has_optional = true;
                optional.push((pat.clone(), input.parse()?));
            } else if has_optional {
                return Err(syn::Error::new(
                    pat.span(),
                    "required argument cannot come after optional argument",
                ));
            } else {
                required += 1;
            }

            args.push_value(pat);

            if input.is_empty() {
                break;
            }

            args.push_punct(input.parse()?);
        }

        Ok(Args {
            parsed: args,
            required,
            optional,
        })
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.parsed.to_tokens(tokens)
    }
}

mod export {
    use syn::custom_keyword;

    custom_keyword!(export);
}

struct DefaultArgs {
    attrs: Vec<Attribute>,
    export: Option<export::export>,
    vis: Visibility,
    constness: Option<Token![const]>,
    asyncness: Option<Token![async]>,
    unsafety: Option<Token![unsafe]>,
    abi: Option<Abi>,
    fn_token: Token![fn],
    fn_path: Punctuated<Ident, Token![::]>,
    generics: Generics,
    paren_token: token::Paren,
    args: Args,
    ret: ReturnType,
    body: Block,
}

impl Parse for DefaultArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let export = input.parse()?;
        let vis = input.parse()?;
        let constness = input.parse()?;
        let asyncness = input.parse()?;
        let unsafety = input.parse()?;
        let abi = input.parse()?;
        let fn_token = input.parse()?;

        let mut fn_path: Punctuated<Ident, Token![::]> = Punctuated::new();
        loop {
            fn_path.push_value(input.parse()?);
            if input.peek(Token![::]) {
                fn_path.push_punct(input.parse()?);
            } else {
                break;
            }
        }

        if fn_path.len() > 1 && fn_path.first().unwrap() != "crate" {
            return Err(syn::Error::new(
                fn_path.first().unwrap().span(),
                "path should start with crate",
            ));
        }

        let mut generics: Generics = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        let args = content.parse()?;
        let ret = input.parse()?;
        generics.where_clause = input.parse()?;
        let body = input.parse()?;

        Ok(DefaultArgs {
            attrs,
            export,
            vis,
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            fn_path,
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
        for i in &self.attrs {
            i.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        self.constness.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        let name = self.fn_path.last().unwrap();
        format_ident!("{}_", name).to_tokens(tokens);
        self.generics.gt_token.to_tokens(tokens);
        self.generics.params.to_tokens(tokens);
        self.generics.lt_token.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
        self.ret.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn default_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DefaultArgs);
    let output = quote! { #input };
    output.into()
}

#[allow(dead_code)]
mod compile_fail_test {
    /// using `self` in argument is compile error for now
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
    ///         fn foo(&self, a: usize, b: usize = 0) -> usize {
    ///             a + b
    ///         }
    ///     }
    /// }
    /// ```
    fn using_self() {}

    /// having required argument after optional argument is an error
    ///
    /// error: `required argument cannot come after optional argument`
    ///
    /// ```compile_fail
    /// # extern crate default_args;
    /// use default_args::default_args;
    ///
    /// default_args! {
    ///     fn foo(a: usize = 0, b: usize) -> usize {
    ///         a + b
    ///     }
    /// }
    /// ```
    fn required_after_optional() {}

    /// if path is used in function name, it should start with crate
    ///
    /// error: `path should start with crate`
    ///
    /// ```compile_fail
    /// # extern crate default_args;
    /// mod foo {
    ///     use default_args::default_args;
    ///
    ///     default_args! {
    ///         fn foo::bar() {}
    ///     }
    /// }
    /// ```
    fn path_not_starting_with_crate() {}
}
