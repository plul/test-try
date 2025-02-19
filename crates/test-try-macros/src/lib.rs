//! test-try.

use proc_macro::TokenStream;

/// Alternative to the `#[test]` macro.
///
/// - Use `?` to return errors without specifying a return type or ending the test with `Ok(())`.
/// - On error, the test prints out a report including the chain of source errors.
#[proc_macro_attribute]
pub fn test_try(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = input_fn.attrs;
    let vis = input_fn.vis;
    let sig = input_fn.sig;
    let block = input_fn.block;
    let stmts = block.stmts;

    assert!(matches!(sig.output, syn::ReturnType::Default), "Don't specify a return type");

    let expanded = quote::quote! {
        #(#attrs)*
        #[test]
        #vis #sig -> ::std::result::Result<(), ::test_try::report::Report> {
            #(#stmts)*
            Ok(())
        }
    };

    TokenStream::from(expanded)
}
