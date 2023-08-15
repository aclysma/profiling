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

#[proc_macro_attribute]
pub fn skip(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut content = parse_macro_input!(item as ItemImpl);
    'func_loop: for block in &mut content.items {
        match block {
            ImplItem::Fn(ref mut func) => {
                for func_attr in &func.attrs {
                    match func_attr.meta {
                        syn::Meta::Path(ref func_path) => {
                            let path_seg = func_path.segments.last().unwrap();
                            if path_seg.ident.to_string() == "skip".to_string() {
                                continue 'func_loop;
                            }
                        }
                        _ => {}
                    }
                }
                let prev_block = &func.block;
                let func_name = func.sig.ident.to_string();
                func.block = impl_block(prev_block, &func_name);
            }
            _ => {}
        }
    }
    // println!("item: \"{:?}\"", content.attrs.);

    (quote!(
        #content
    ))
    .into()
}

#[cfg(not(any(
    feature = "profile-with-puffin",
    feature = "profile-with-optick",
    feature = "profile-with-superluminal",
    feature = "profile-with-tracing",
    feature = "profile-with-tracy"
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
            let _fn_span = profiling::tracing::span!(profiling::tracing::Level::INFO, #instrumented_function_name);
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
            let _tracy_span = profiling::tracy_client::span!(#instrumented_function_name, 0);

            #body
        }
    }
}
