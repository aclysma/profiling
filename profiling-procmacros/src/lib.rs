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
    let new_body: syn::Block = parse_quote! {
        {
            #[cfg(feature = "profile-with-puffin")]
            profiling::puffin::profile_function!();

            #[cfg(feature = "profile-with-optick")]
            profiling::optick::event!();

            #[cfg(feature = "profile-with-tracing")]
            let _fn_span = profiling::tracing::span!(tracing::Level::INFO, #instrumented_function_name);
            #[cfg(feature = "profile-with-tracing")]
            let _fn_span_entered = _fn_span.enter();

            #[cfg(feature = "profile-with-tracy")]
            // Note: callstack_depth is 0 since this has significant overhead
            let _tracy_span = profiling::tracy_client::Span::new(#instrumented_function_name, "", file!(), line!(), 0);

            #[cfg(feature = "profile-with-superluminal")]
            let _superluminal_guard = profiling::superluminal::SuperluminalGuard::new(#instrumented_function_name);

            #body
        }
    };

    function.block = Box::new(new_body);

    (quote! {
        #function
    })
    .into()
}
