fn main() {
    println!("Starting loop, profiler can now be attached");

    #[cfg(feature = "profile-with-optick")]
    optick::register_thread("main");

    // You could do something like this with tracy
    //#[cfg(feature = "profile-with-tracy")]
    //tracy_client::set_thread_name("Main Thread");

    loop {
        profiling::scope!("Main Thread");
        some_function();
        some_other_function(10);

        #[cfg(feature = "profile-with-puffin")]
        puffin::GlobalProfiler::lock().new_frame();

        #[cfg(feature = "profile-with-optick")]
        optick::next_frame();

        // You could do something like this with tracy
        // #[cfg(feature = "profile-with-tracy")]
        // tracy_client::finish_continuous_frame!();
    }
}

#[profiling::function]
fn some_function() {
    profiling::scope!("some_function");
    std::thread::sleep(std::time::Duration::from_millis(1));
}

fn some_other_function(iterations: usize) {
    profiling::scope!("some_other_function");
    std::thread::sleep(std::time::Duration::from_millis(1));
    for _ in 0..iterations {
        some_inner_function_that_sleeps();
    }
}

fn some_inner_function_that_sleeps() {
    profiling::scope!("some_inner_function_that_sleeps");
    std::thread::sleep(std::time::Duration::from_millis(2));
}
