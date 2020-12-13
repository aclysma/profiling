#[doc(hidden)]
pub mod guard;

#[cfg(feature = "profile-with-puffin")]
pub use puffin;

#[cfg(feature = "profile-with-optick")]
pub use optick;

#[cfg(feature = "profile-with-superluminal")]
pub use superluminal_perf;

#[cfg(feature = "profile-with-tracy")]
pub use tracy_client;

#[cfg(feature = "profile-with-tracing")]
pub use tracing;

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
        $crate::puffin::profile_scope!($name);

        #[cfg(feature = "profile-with-optick")]
        $crate::optick::event!($name);

        #[cfg(feature = "profile-with-superluminal")]
        let _superluminal_guard = $crate::guard::superluminal::SuperluminalGuard::new($name);

        #[cfg(feature = "profile-with-tracy")]
        // Note: callstack_depth is 0 since this has significant overhead
        let _tracy_span = $crate::tracy_client::Span::new($name, "", file!(), line!(), 0);

        #[cfg(feature = "profile-with-tracing")]
        let _span = $crate::tracing::span!(tracing::Level::INFO, $name);
        #[cfg(feature = "profile-with-tracing")]
        let _span_entered = _span.enter();
    };
    // NOTE: I've not been able to get attached data to work with optick
    ($name:expr, $data:expr) => {
        #[cfg(feature = "profile-with-puffin")]
        $crate::puffin::profile_scope_data!($name, $data);

        #[cfg(feature = "profile-with-optick")]
        $crate::optick::event!($name);
        #[cfg(feature = "profile-with-optick")]
        $crate::optick::tag!("tag", $data);

        #[cfg(feature = "profile-with-superluminal")]
        let _superluminal_guard =
            $crate::guard::superluminal::SuperluminalGuard::new_with_data($name, $data);

        #[cfg(feature = "profile-with-tracy")]
        let _tracy_span = $crate::tracy_client::Span::new($name, "", file!(), line!(), 0);
        #[cfg(feature = "profile-with-tracy")]
        _tracy_span.emit_text($data);

        #[cfg(feature = "profile-with-tracing")]
        let _span = $crate::tracing::span!(tracing::Level::INFO, $name, tag = $data);
        #[cfg(feature = "profile-with-tracing")]
        let _span_entered = _span.enter();
    };
}

/// Opens a scope with a named binding for the guard allowing it to be manually managed. This is useful if you want to break up a function into multiple sections in the profiling output without increasing the indentation levels. Two variants:
///  - profiling::manual_scope!(guard, name: &str) - Opens a scope with the given name
///  - profiling::manual_scope!(guard, name: &str, data: &str) - Opens a scope with the given name and an extra
///    datafield. Details of this depend on the API, but it should be a &str. If the extra data is
///    named, it will be named "tag". Some APIs support adding more data (for example, `optic::tag!`)
///
/// ```
/// profiling::manual_scope!(guard, "outer");
/// for _ in 0..10 {
///     profiling::manual_scope!(_guard, "inner", format!("iteration {}").as_str());
/// }
/// drop(guard);
/// ```
#[macro_export]
macro_rules! manual_scope {
    ($guard:tt, $name:expr) => {
        let $guard = $crate::guard::Guard {
            #[cfg(feature = "profile-with-puffin")]
            puffin: if $crate::puffin::are_scopes_on() {
                Some($crate::puffin::ProfilerScope::new(
                    $name,
                    $crate::puffin::current_file_name!(),
                    "",
                ))
            } else {
                None
            },

            // Note: optick unsupported
            #[cfg(feature = "profile-with-superluminal")]
            superluminal: $crate::guard::superluminal::SuperluminalGuard::new($name),

            #[cfg(feature = "profile-with-tracy")]
            tracy: $crate::tracy_client::Span::new($name, "", file!(), line!(), 0),

            #[cfg(feature = "profile-with-tracing")]
            tracing: $crate::guard::tracing::TracingGuard::new($crate::tracing::span!(
                tracing::Level::INFO,
                $name
            )),
        };
    };
    // NOTE: I've not been able to get attached data to work with optick
    ($guard:tt, $name:expr, $data:expr) => {
        let $guard = $crate::guard::Guard {
            #[cfg(feature = "profile-with-puffin")]
            puffin: if $crate::puffin::are_scopes_on() {
                Some($crate::puffin::ProfilerScope::new(
                    $name,
                    $crate::puffin::current_file_name!(),
                    $data,
                ))
            } else {
                None
            },

            // Note: optick unsupported
            #[cfg(feature = "profile-with-superluminal")]
            superluminal: $crate::superluminal::SuperluminalGuard::new_with_data($name, $data),

            #[cfg(feature = "profile-with-tracy")]
            tracy: {
                let span = $crate::tracy_client::Span::new($name, "", file!(), line!(), 0);
                span.emit_text($data);
                span
            },

            #[cfg(feature = "profile-with-tracing")]
            tracing: $crate::guard::tracing::TracingGuard::new($crate::tracing::span!(
                tracing::Level::INFO,
                $name,
                tag = $data
            )),
        };
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

/// Registers a thread with the profiler API(s). This is usually setting a name for the thread.
/// Two variants:
///  - register_thread!() - Tries to get the name of the thread, or an ID if no name is set
///  - register_thread!(name: &str) - Registers the thread using the given name
#[macro_export]
macro_rules! register_thread {
    () => {
        let thread_name = std::thread::current()
            .name()
            .map(|x| x.to_string())
            .unwrap_or_else(|| format!("Thread {:?}", std::thread::current().id()));

        $crate::register_thread!(&thread_name);
    };
    ($name:expr) => {
        // puffin uses the thread name

        #[cfg(feature = "profile-with-optick")]
        $crate::optick::register_thread($name);

        #[cfg(feature = "profile-with-superluminal")]
        $crate::superluminal_perf::set_current_thread_name($name);

        #[cfg(feature = "profile-with-tracy")]
        $crate::tracy_client::set_thread_name($name);
    };
}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
macro_rules! finish_frame {
    () => {
        #[cfg(feature = "profile-with-puffin")]
        $crate::puffin::GlobalProfiler::lock().new_frame();

        #[cfg(feature = "profile-with-optick")]
        $crate::optick::next_frame();

        // superluminal does not have a frame end function

        #[cfg(feature = "profile-with-tracy")]
        $crate::tracy_client::finish_continuous_frame!();
    };
}

// Maintain current public API
#[doc(hidden)]
#[cfg(feature = "profile-with-superluminal")]
pub use guard::superluminal;
