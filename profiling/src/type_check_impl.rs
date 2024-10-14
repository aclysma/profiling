// This backend is intended to force type checking

#[macro_export]
macro_rules! scope {
    ($name:literal) => {
        let _: &'static str = $name;
    };
    ($name:literal, $data:expr) => {
        let _: &'static str = $name;
        let _: &str = $data;
    };
}

#[macro_export]
macro_rules! function_scope {
    () => {};
    ($data:expr) => {
        let _: &str = $data;
    };
}

#[macro_export]
macro_rules! register_thread {
    () => {};
    ($name:expr) => {
        let _: &str = $name;
    };
}

#[macro_export]
macro_rules! finish_frame {
    () => {};
}
