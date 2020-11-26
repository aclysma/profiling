/// Opens a scope. Two variants:
///  - profiling::scope!(name: &str) - Opens a scope with the given name
///  - profiling::scope!(name: &str, data: &str) - Opens a scope with the given name and an extra
///    datafield. Details of this depend on the API, but it should be a &str. If the extra data is
///    named, it will be named "tag". Some APIs support adding more data (for example, `optic::tag!`)
///
/// ```
/// profiling::scope!("outer");
/// for _ in 0..10 {
///     profiling::scope!("inner", format!("iteration {}").as_str());
/// }
/// ```
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
        optick::tag!("tag", $data);

        #[cfg(feature = "profile-with-tracing")]
        let _span = tracing::span!(tracing::Level::INFO, $name, tag = $data);
        #[cfg(feature = "profile-with-tracing")]
        let _span_entered = _span.enter();
    };
}

/// Registers a thread with the profiler API(s). This is usually setting a name for the thread.
#[macro_export]
macro_rules! register_thread {
    ($name:expr) => {
        #[cfg(feature = "profile-with-optick")]
        optick::register_thread($name);

        #[cfg(feature = "profile-with-tracy")]
        tracy_client::set_thread_name($name);
    };
}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
macro_rules! finish_frame {
    () => {
        #[cfg(feature = "profile-with-puffin")]
        puffin::GlobalProfiler::lock().new_frame();

        #[cfg(feature = "profile-with-optick")]
        optick::next_frame();

        #[cfg(feature = "profile-with-tracy")]
        tracy_client::finish_continuous_frame!();
    };
}

/// Proc macro for creating a scope around the function, using the name of the function for the
/// scope's name
///
/// This must be done as a proc macro because tracing requires a const string
///
/// ```
/// #[profiling::function]
/// fn my_function() {
///
/// }
/// ```
pub use profiling_procmacros::function;
