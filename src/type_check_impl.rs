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
        $crate::_type_check_scope_name($name)
    };
    ($name:expr, $data:expr) => {
        $crate::_type_check_scope_name_data($name, $data)
    };
}

#[doc(hidden)]
pub fn _type_check_scope_name(_name: &str) {}

#[doc(hidden)]
pub fn _type_check_scope_name_data(
    _name: &str,
    _data: &str,
) {
}

/// Registers a thread with the profiler API(s). This is usually setting a name for the thread.
/// Two variants:
///  - register_thread!() - Tries to get the name of the thread, or an ID if no name is set
///  - register_thread!(name: &str) - Registers the thread using the given name
#[macro_export]
macro_rules! register_thread {
    () => {};
    ($name:expr) => {
        $crate::_type_check_register_thread_name($name)
    };
}

#[doc(hidden)]
pub fn _type_check_register_thread_name(_name: &str) {}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
macro_rules! finish_frame {
    () => {};
}
