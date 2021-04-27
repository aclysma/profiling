extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn function(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let instrumented_function_name = function.sig.ident.to_string();

    let body = &function.block;
    let new_body: syn::Block = impl_block(body, &instrumented_function_name);

    function.block = Box::new(new_body);

    (quote! {
        #function
    })
    .into()
}

#[cfg(not(any(
    feature = "profile-with-puffin",
    feature = "profile-with-optick",
    feature = "profile-with-superluminal",
    feature = "profile-with-tracing",
    feature = "profile-with-tracy",
)))]
fn impl_block(
    body: &syn::Block,
    _instrumented_function_name: &str,
) -> syn::Block {
    parse_quote! {
        {
            #body
        }
    }
}

#[cfg(feature = "profile-with-puffin")]
fn impl_block(
    body: &syn::Block,
    _instrumented_function_name: &str,
) -> syn::Block {
    parse_quote! {
        {
            profiling::puffin::profile_function!();

            #body
        }
    }
}

#[cfg(feature = "profile-with-optick")]
fn impl_block(
    body: &syn::Block,
    _instrumented_function_name: &str,
) -> syn::Block {
    parse_quote! {
        {
            profiling::optick::event!();

            #body
        }
    }
}

#[cfg(feature = "profile-with-superluminal")]
fn impl_block(
    body: &syn::Block,
    instrumented_function_name: &str,
) -> syn::Block {
    parse_quote! {
        {
            let _superluminal_guard = profiling::superluminal::SuperluminalGuard::new(#instrumented_function_name);

            #body
        }
    }
}

#[cfg(feature = "profile-with-tracing")]
fn impl_block(
    body: &syn::Block,
    instrumented_function_name: &str,
) -> syn::Block {
    parse_quote! {
        {
            let _fn_span = profiling::tracing::span!(tracing::Level::INFO, #instrumented_function_name);
            let _fn_span_entered = _fn_span.enter();

            #body
        }
    }
}

#[cfg(feature = "profile-with-tracy")]
fn impl_block(
    body: &syn::Block,
    instrumented_function_name: &str,
) -> syn::Block {
    parse_quote! {
        {
            // Note: callstack_depth is 0 since this has significant overhead
            let _tracy_span = profiling::tracy_client::Span::new(#instrumented_function_name, "", file!(), line!(), 0);

            #body
        }
    }
}
