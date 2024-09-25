#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        let _span = $crate::tracing::span!($crate::tracing::Level::INFO, $name);
        let _span_entered = _span.enter();
    };
    ($name:expr, $data:expr) => {
        let _span = $crate::tracing::span!($crate::tracing::Level::INFO, $name, tag = $data);
        let _span_entered = _span.enter();
    };
}

// NOTE: Not supported as tracing does not support non literal spans. Use #[profiling::function] instead.
#[macro_export]
macro_rules! function_scope {
    () => {};
    ($data:expr) => {};
}

#[macro_export]
macro_rules! register_thread {
    () => {};
    ($name:expr) => {};
}

#[macro_export]
macro_rules! finish_frame {
    () => {};
}
