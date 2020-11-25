#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        #[cfg(feature = "profile-with-puffin")]
        puffin::profile_scope!($name);

        #[cfg(feature = "profile-with-optick")]
        optick::event!($name);

        #[cfg(feature = "profile-with-tracing")]
        tracing::span!(tracing::Level::INFO, $name);
    };

    ($name:expr, $data:expr) => {
        #[cfg(feature = "profile-with-puffin")]
        puffin::profile_scope_data!($data);

        #[cfg(feature = "profile-with-optick")]
        optick::event!($name);
        #[cfg(feature = "profile-with-optick")]
        optick::tag!($data);

        #[cfg(feature = "profile-with-tracing")]
        tracing::span!(tracing::Level::INFO, $name, tag = $data);
    };
}

// This must be done as a proc macro because tracing requires a const string
pub use profiling_derive::function;
