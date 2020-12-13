pub struct Guard {
    #[cfg(feature = "profile-with-puffin")]
    pub puffin: Option<puffin::ProfilerScope>,
    //#[cfg(feature = "profile-with-optick")]
    // optick doesn't expose a covenient way to construct it's guard object and imitating its
    // `event!` macro would involve unsafe
    #[cfg(feature = "profile-with-superluminal")]
    pub superluminal: superluminal::SuperluminalGuard,
    #[cfg(feature = "profile-with-tracy")]
    pub tracy: tracy_client::Span,
    #[cfg(feature = "profile-with-tracing")]
    pub tracing: tracing::TracingGuard,
}

//
// RAII wrapper to support superluminal. This is public as they need to be callable from macros
// but are not intended for direct use.
//
#[cfg(feature = "profile-with-superluminal")]
#[doc(hidden)]
pub mod superluminal {
    pub struct SuperluminalGuard;

    // 0xFFFFFFFF means "use default color"
    const DEFAULT_SUPERLUMINAL_COLOR: u32 = 0xFFFFFFFF;

    impl SuperluminalGuard {
        pub fn new(name: &str) -> Self {
            superluminal_perf::begin_event(name);
            SuperluminalGuard
        }

        pub fn new_with_data(
            name: &str,
            data: &str,
        ) -> Self {
            superluminal_perf::begin_event_with_data(name, data, DEFAULT_SUPERLUMINAL_COLOR);
            SuperluminalGuard
        }
    }

    impl Drop for SuperluminalGuard {
        fn drop(&mut self) {
            superluminal_perf::end_event();
        }
    }
}

//
// RAII wrapper for tracing that doesn't involve having a guard that is tied to the lifetime of
// a reference. This is public as it needs to be callable from macros
// but is not intended for direct use.
//
#[cfg(feature = "profile-with-tracing")]
#[doc(hidden)]
pub mod tracing {
    pub struct TracingGuard {
        span: tracing::Span,
        subscriber: tracing::Dispatch,
    }

    impl TracingGuard {
        pub fn new(span: tracing::Span) -> Self {
            let subscriber = tracing::dispatcher::get_default(|d| d.clone());
            if let Some(id) = span.id() {
                subscriber.enter(&id);
            }
            Self { span, subscriber }
        }
    }

    impl Drop for TracingGuard {
        fn drop(&mut self) {
            if let Some(id) = self.span.id() {
                self.subscriber.exit(&id);
            }
        }
    }
}
