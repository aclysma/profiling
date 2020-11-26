#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        #[cfg(feature = "profile-with-puffin")]
        puffin::profile_scope!($name);

        #[cfg(feature = "profile-with-optick")]
        optick::event!($name);

        #[cfg(feature = "profile-with-tracing")]
        let _span = tracing::span!(tracing::Level::INFO, $name);
        #[cfg(feature = "profile-with-tracing")]
        let _span_entered = _span.enter();
    };
    // NOTE: I've not been able to get attached data to work with optick and tracy
    ($name:expr, $data:expr) => {
        #[cfg(feature = "profile-with-puffin")]
        puffin::profile_scope_data!($name, $data);

        #[cfg(feature = "profile-with-optick")]
        optick::event!($name);
        #[cfg(feature = "profile-with-optick")]
        optick::tag!("scope_tag", $data);

        #[cfg(feature = "profile-with-tracing")]
        let _span = tracing::span!(tracing::Level::INFO, $name, tag = $data);
        #[cfg(feature = "profile-with-tracing")]
        let _span_entered = _span.enter();

    };
}

// This must be done as a proc macro because tracing requires a const string
pub use profiling_procmacros::function;
