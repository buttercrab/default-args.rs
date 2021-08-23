//! Enables default arguments in rust by macro.
//!
//! Just wrap function with `default_args!` and macro with name of function
//! would be automatically generated to be used with default argument
//!
//! ```
//! # extern crate default_args;
//! use default_args::default_args;
//!
//! // this would make a macro named `foo`
//! // and original function named `foo_`
//! default_args! {
//!     fn foo(important_arg: u32, optional: u32 = 100) -> String {
//!         // ...
//! #        format!("{}-{}", important_arg, optional)
//!     }
//! }
//!
//! // in other codes...
//! assert_eq!(foo!(1), "1-100"); // foo(1, 100)
//! assert_eq!(foo!(1, 3), "1-3"); // foo(1, 3)
//! assert_eq!(foo!(1, optional=5), "1-5"); // foo(1, 5)
//! assert_eq!(foo!(1, optional = 10), "1-10"); // foo(1, 10)
//! ```

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::iter::FromIterator;
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

fn require_args_def(count: usize) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::from_iter((0..count).map(|i| {
        let item = format_ident!("r{}", i);
        if i == 0 {
            quote! { $#item:expr }
        } else {
            quote! { , $#item:expr }
        }
    }))
}

fn require_args(count: usize) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::from_iter((0..count).map(|i| {
        let item = format_ident!("r{}", i);
        if i == 0 {
            quote! { $#item }
        } else {
            quote! { , $#item }
        }
    }))
}

fn generate_named_one(
    input: &DefaultArgs,
    check: &mut Vec<Option<usize>>,
    names: &mut Vec<(PatType, Expr)>,
) -> proc_macro2::TokenStream {
    let req_def = require_args_def(input.args.required);
    let req = require_args(input.args.required);
    let fn_name = format_ident!("{}_", input.fn_path.last().unwrap());

    let opt_def =
        proc_macro2::TokenStream::from_iter(names.iter().enumerate().map(|(i, (pat, _))| {
            let item = format_ident!("o{}", i);
            let pat = pat.pat.as_ref();
            if input.args.required == 0 && i == 0 {
                quote! { #pat = $#item:expr }
            } else {
                quote! { , #pat = $#item:expr }
            }
        }));

    let opt = proc_macro2::TokenStream::from_iter(check.iter().enumerate().map(|(i, j)| {
        let inner = if let Some(j) = *j {
            let item = format_ident!("o{}", j);
            quote! { $#item }
        } else {
            let (_, ref item) = input.args.optional[i];
            quote! { ( #item ) }
        };

        if input.args.required == 0 && i == 0 {
            quote! { #inner }
        } else {
            quote! { , #inner }
        }
    }));

    quote! {
        (#req_def#opt_def) => {
            #fn_name(#req#opt)
        };
    }
}

fn generate_named_inner(
    input: &DefaultArgs,
    check: &mut Vec<Option<usize>>,
    names: &mut Vec<(PatType, Expr)>,
) -> proc_macro2::TokenStream {
    let mut ret = generate_named_one(input, check, names);
    ret.append_all(
        check
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, b)| b.is_none())
            .map(|(i, _)| {
                check[i] = Some(names.len());
                names.push(input.args.optional[i].clone());
                let r = generate_named_inner(input, check, names);
                names.pop();
                check[i] = None;
                r
            }),
    );
    ret
}

fn generate_unnamed_inner(input: &DefaultArgs) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::from_iter((1..=input.args.optional.len()).map(|i| {
        let req_def = require_args_def(input.args.required);
        let req = require_args(input.args.required);
        let fn_name = format_ident!("{}_", input.fn_path.last().unwrap());

        let opt_def = proc_macro2::TokenStream::from_iter((0..i).map(|j| {
            let item = format_ident!("o{}", j);
            if input.args.required == 0 && j == 0 {
                quote! { $#item:expr }
            } else {
                quote! { , $#item:expr }
            }
        }));

        let opt = proc_macro2::TokenStream::from_iter((0..input.args.optional.len()).map(|j| {
            let inner = if j < i {
                let item = format_ident!("o{}", j);
                quote! { $#item }
            } else {
                let (_, ref item) = input.args.optional[j];
                quote! { ( #item ) }
            };

            if input.args.required == 0 && j == 0 {
                quote! { #inner }
            } else {
                quote! { , #inner }
            }
        }));

        quote! {
            (#req_def#opt_def) => {
                #fn_name(#req#opt)
            };
        }
    }))
}

fn generate_macro(input: &DefaultArgs) -> proc_macro2::TokenStream {
    let mut check = vec![None; input.args.optional.len()];
    let mut names = Vec::new();
    let named_inner = generate_named_inner(input, &mut check, &mut names);
    let unnamed_inner = generate_unnamed_inner(input);
    let name = input.fn_path.last().unwrap();
    let export = if input.export.is_some() {
        quote! { #[macro_export] }
    } else {
        quote! {}
    };

    quote! {
        #export
        macro_rules! #name {
            #named_inner
            #unnamed_inner
        }
    }
}

#[proc_macro]
pub fn default_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DefaultArgs);
    let generated_macro = generate_macro(&input);
    let output = quote! {
        #input

        #generated_macro
    };
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
