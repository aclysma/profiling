
use tracing_tracy;
use tracing;
use tracy_client;

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

    // Tracy requires having a layer set up with the tracing crate
    #[cfg(feature = "profile-with-tracy")]
    {
        use tracing_subscriber::layer::SubscriberExt;
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
        ).unwrap();
    }

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

fn burn_time(millis: u128) {
    let start_time = std::time::Instant::now();
    loop {
        if (std::time::Instant::now() - start_time).as_millis() > millis {
            break;
        }
    }
}

// This `profiling::function` attribute is equivalent to profiling::scope!(function_name)
#[profiling::function]
fn some_function() {
    burn_time(5);
}

fn some_other_function(iterations: usize) {
    profiling::scope!("some_other_function");
    burn_time(5);

    {
        profiling::scope!("do iterations");
        for i in 0..iterations {
            profiling::scope!("some_inner_function_that_sleeps", format!("other data {}", i).as_str());
            optick::tag!("extra_data", "MORE DATA");
            some_inner_function(i);
            burn_time(1);
        }
    }
}

#[profiling::function]
fn some_inner_function(_iteration_index: usize) {
    burn_time(10);
}
