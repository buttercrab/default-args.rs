//! Enables default arguments in rust by macro in zero cost.
//!
//! Just wrap function with `default_args!` and macro with name of function
//! would be automatically generated to be used with default argument.
//!
//! See below for usage
//!
//! ```
//! # extern crate default_args;
//! use default_args::default_args;
//!
//! // this would make a macro named `foo`
//! // and original function named `foo_`
//! default_args! {
//!     fn foo(important_arg: u32, optional: u32 = 100) -> String {
//!         format!("{}, {}", important_arg, optional)
//!     }
//! }
//!
//! // in other codes...
//! assert_eq!(foo!(1), "1, 100"); // foo(1, 100)
//! assert_eq!(foo!(1, 3), "1, 3"); // foo(1, 3)
//! assert_eq!(foo!(1, optional=5), "1, 5"); // foo(1, 5)
//! assert_eq!(foo!(1, optional = 10), "1, 10"); // foo(1, 10)
//! ```
//!
//! # More Features
//!
//! ## Export
//!
//! Add export in the front of the function and the macro would be exported.
//! *(add pub to export function with macro)*
//!
//! ```
//! # extern crate default_args;
//! # use default_args::default_args;
//! #
//! default_args! {
//!     export pub fn foo() {}
//! }
//! ```
//!
//! ## Path of function
//!
//! Macro just call the function in name, so you should import both macro and the function to use it.
//! By writing the path of this function, you can just only import the macro.
//! *(path should start with `crate`)*
//!
//! ```ignore
//! # extern crate default_args;
//! #
//! #[macro_use]
//! pub mod foo {
//!     # use default_args::default_args;
//!     default_args! {
//!         pub fn crate::foo::bar() {}
//!     }
//! }
//!
//! // then it would create `bar!()`
//! bar!();
//! ```
//!
//! ## *Why do we have to write module?*
//!
//! > `std::module_path!` can resolve the module path of the function where it is declared.
//! > However, it can be resolved in runtime, not compile-time.
//! > I couldn't find a way to get module path in compile-time.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, token, Abi, Attribute, Block, Expr, FnArg, Generics, PatType,
    ReturnType, Token, Visibility,
};

/// Structure for arguments
///
/// This contains arguments of function and default values like: `a: u32, b: u32 = 0`
struct Args {
    parsed: Punctuated<PatType, Token![,]>,
    required: usize,
    optional: Vec<(PatType, Expr)>,
}

impl Parse for Args {
    /// Parse function for `Args`
    ///
    /// ## Errors
    ///
    /// - when self is the argument of the function: `self in default_args! is not support in this version`
    /// - when required argument came after any optional argument: `required argument cannot come after optional argument`
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
    /// This function changes to normal signature of function which is `self.parsed`
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.parsed.to_tokens(tokens)
    }
}

/// Module for export keyword
///
/// export keyword would make macro export (by adding `#[macro_export]`
mod export {
    use syn::custom_keyword;

    custom_keyword!(export);
}

/// Structure for Default Argument function
///
/// This contains the signature of function like
/// `#[hello] export pub const async unsafe extern "C" fn crate::foo::bar<T>(a: T, b: u32 = 0) -> String where T: Display { format!("{}, {}", a, b) }`
struct DefaultArgs {
    attrs: Vec<Attribute>,
    export: Option<export::export>,
    vis: Visibility,
    constness: Option<Token![const]>,
    asyncness: Option<Token![async]>,
    unsafety: Option<Token![unsafe]>,
    abi: Option<Abi>,
    fn_token: Token![fn],
    crate_path: Option<(Token![crate], Token![::])>,
    fn_path: Punctuated<Ident, Token![::]>,
    fn_name: Ident,
    generics: Generics,
    paren_token: token::Paren,
    args: Args,
    ret: ReturnType,
    body: Block,
}

impl Parse for DefaultArgs {
    /// Parse function for `DefaultArgs`
    ///
    /// ## Errors
    ///
    /// - when path don't start with `crate`: `path should start with crate`
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
        let crate_token = input.parse::<Option<Token![crate]>>()?;
        let crate_path = if let Some(token) = crate_token {
            let crate_colon_token = input.parse::<Token![::]>()?;
            Some((token, crate_colon_token))
        } else {
            None
        };

        loop {
            fn_path.push_value(input.parse()?);
            if input.peek(Token![::]) {
                fn_path.push_punct(input.parse()?);
            } else {
                break;
            }
        }

        if crate_path.is_none() && fn_path.len() > 1 {
            return Err(syn::Error::new(
                fn_path.first().unwrap().span(),
                "path should start with crate",
            ));
        }
        let fn_name = fn_path.pop().unwrap().into_value();

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
            crate_path,
            fn_path,
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
    /// This function changes to normal signature of function
    /// It would not print `export` and change the name with under bar attached
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
        format_ident!("{}_", &self.fn_name).to_tokens(tokens);
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

/// Make unnamed arguments in macro
/// - `count`: how many arguments
/// - `def`: if it would be used in macro definition (will add `expr`)
fn unnamed_args(count: usize, def: bool) -> proc_macro2::TokenStream {
    (0..count)
        .map(|i| {
            let item = format_ident!("u{}", i);
            if def {
                if i == 0 {
                    quote! { $#item:expr }
                } else {
                    quote! { , $#item:expr }
                }
            } else if i == 0 {
                quote! { $#item }
            } else {
                quote! { , $#item }
            }
        })
        .collect()
}

/// Make named arguments in definition of macro
/// - `front_comma`: if it needs a front comma
/// - `input`: default args
/// - `macro_index`: mapped index of argument in function from macro
fn named_args_def(
    front_comma: bool,
    input: &DefaultArgs,
    macro_index: &[usize],
) -> proc_macro2::TokenStream {
    macro_index
        .iter()
        .map(|i| {
            let item = format_ident!("n{}", i);
            let pat = &input.args.optional[*i].0.pat;
            if !front_comma && *i == 0 {
                quote! { #pat = $#item:expr }
            } else {
                quote! { , #pat = $#item:expr }
            }
        })
        .collect()
}

/// Make names arguments in macro
/// - `front_comma`: if it needs a front comma
/// - `input`: default args
/// - `offset`: offset of named argument
/// - `func_index`: whether if the function argument is provided
fn named_args(
    front_comma: bool,
    input: &DefaultArgs,
    offset: usize,
    func_index: &[bool],
) -> proc_macro2::TokenStream {
    func_index
        .iter()
        .enumerate()
        .map(|(i, provided)| {
            let inner = if *provided {
                let item = format_ident!("n{}", i + offset);
                quote! { $#item }
            } else {
                let item = &input.args.optional[i + offset].1;
                quote! { ( #item ) }
            };

            if !front_comma && i == 0 {
                quote! { #inner }
            } else {
                quote! { , #inner }
            }
        })
        .collect()
}

/// Generate one arm of macro
/// - `input`: default args
/// - `unnamed_cnt`: unnamed argument count
/// - `offset`: offset of named argument
/// - `macro_index`: mapped index of argument in function from macro
/// - `func_index`: whether if the function argument is provided
fn generate(
    input: &DefaultArgs,
    unnamed_cnt: usize,
    offset: usize,
    macro_index: &[usize],
    func_index: &[bool],
) -> proc_macro2::TokenStream {
    let fn_name = format_ident!("{}_", input.fn_name);

    let unnamed_def = unnamed_args(unnamed_cnt, true);
    let unnamed = unnamed_args(unnamed_cnt, false);

    let named_def = named_args_def(unnamed_cnt != 0, input, macro_index);
    let named = named_args(unnamed_cnt != 0, input, offset, func_index);

    if input.crate_path.is_some() {
        let fn_path = &input.fn_path;
        quote! {
            (#unnamed_def#named_def) => {
                $crate::#fn_path#fn_name(#unnamed#named)
            };
        }
    } else {
        quote! {
            (#unnamed_def#named_def) => {
                #fn_name(#unnamed#named)
            };
        }
    }
}

/// Generate macro arms recursively
/// - `input`: default args
/// - `unnamed_cnt`: unnamed argument count
/// - `offset`: offset of named argument
/// - `macro_index`: mapped index of argument in function from macro
/// - `func_index`: whether if the function argument is provided
/// - `stream`: token stream to append faster
fn generate_recursive(
    input: &DefaultArgs,
    unnamed_cnt: usize,
    offset: usize,
    macro_index: &mut Vec<usize>,
    func_index: &mut Vec<bool>,
    stream: &mut proc_macro2::TokenStream,
) {
    stream.append_all(generate(
        input,
        unnamed_cnt,
        offset,
        macro_index,
        func_index,
    ));

    for i in 0..func_index.len() {
        if func_index[i] {
            continue;
        }

        func_index[i] = true;
        macro_index.push(i + offset);
        generate_recursive(input, unnamed_cnt, offset, macro_index, func_index, stream);
        macro_index.pop();
        func_index[i] = false;
    }
}

/// Generates all macro arms
/// - `input`: default args
fn generate_macro(input: &DefaultArgs) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for i in 0..=input.args.optional.len() {
        let mut macro_index = Vec::new();
        let mut func_index = vec![false; input.args.optional.len() - i];
        generate_recursive(
            input,
            input.args.required + i,
            i,
            &mut macro_index,
            &mut func_index,
            &mut stream,
        );
    }

    stream
}

/// The main macro of this crate
///
/// This would generate the original function and the macro
#[proc_macro]
pub fn default_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DefaultArgs);

    let name = &input.fn_name;
    let export = if input.export.is_some() {
        quote! { #[macro_export] }
    } else {
        quote! {}
    };

    let inner = generate_macro(&input);

    let output = quote! {
        #input

        #export
        macro_rules! #name {
            #inner
        }
    };
    output.into()
}

/// This is a test for compile failure
/// This will check the error cases
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
