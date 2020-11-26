fn main() {
    println!("Starting loop, profiler can now be attached");

    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    let mut any_profiler_enabled = false;

    #[allow(unused_assignments)]
    #[cfg(feature = "profile-with-puffin")]
    {
        any_profiler_enabled = true;
    }

    #[allow(unused_assignments)]
    #[cfg(feature = "profile-with-optick")]
    {
        any_profiler_enabled = true;
    }

    assert!(any_profiler_enabled, "No profiler feature flags were enabled. Since this is an example, this is probably a mistake.");

    #[cfg(feature = "profile-with-optick")]
    optick::register_thread("main");

    #[cfg(feature = "profile-with-tracy")]
    tracy_client::set_thread_name("Main Thread");

    loop {
        profiling::scope!("Main Thread");
        some_function();
        some_other_function(10);

        println!("frame complete");

        #[cfg(feature = "profile-with-puffin")]
        puffin::GlobalProfiler::lock().new_frame();

        #[cfg(feature = "profile-with-optick")]
        optick::next_frame();

        #[cfg(feature = "profile-with-tracy")]
        tracy_client::finish_continuous_frame!();
    }
}

#[profiling::function]
fn some_function() {
    profiling::scope!("some_function");
    std::thread::sleep(std::time::Duration::from_millis(5));
}

fn some_other_function(iterations: usize) {
    profiling::scope!("some_other_function");
    std::thread::sleep(std::time::Duration::from_millis(5));
    for i in 0..iterations {
        some_inner_function_that_sleeps(i);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn some_inner_function_that_sleeps(_iteration_index: usize) {
    profiling::scope!("some_inner_function_that_sleeps");
    std::thread::sleep(std::time::Duration::from_millis(10));
}
