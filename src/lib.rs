mod core;

use crate::core::{generate_macro, DefaultArgs};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

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
    /// error: `self in default_args! is not support in this version`
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

    /// using `self` in argument is compile error for now
    ///
    /// error: `self in default_args! is not support in this version`
    ///
    /// ```compile_fail
    /// # extern crate default_args;
    /// use default_args::default_args_attribute;
    ///
    /// struct A {}
    ///
    /// impl A {
    ///     #[default_args_attribute]
    ///     fn foo(&self, a: usize, #[default(0)] b: usize) -> usize {
    ///         a + b
    ///     }
    /// }
    /// ```
    fn using_self_attr() {}

    /// having required argument after optional argument is an error
    ///
    /// error: `required argument cannot come after optional argument`
    ///
    /// ```compile_fail
    /// # extern crate default_args;
    /// use default_args::default_args_attribute;
    ///
    /// #[default_args_attribute]
    /// fn foo(#[default(0)] a: usize, b: usize) -> usize {
    ///     a + b
    /// }
    /// ```
    fn required_after_optional_attr() {}
}

#[proc_macro_attribute]
pub fn default_args_attribute(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
