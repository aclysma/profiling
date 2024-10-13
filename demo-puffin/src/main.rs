//
// Example of marking up all functions on an impl block
//
struct Foo;

#[profiling::all_functions]
impl Foo {
    pub fn function1() {
        some_other_function(2);
    }

    #[profiling::skip]
    pub fn function2() {
        some_other_function(1);
    }
}

//
// Examples of marking up a single function
//

// This `profiling::function` attribute is equivalent to profiling::scope!(function_name)
#[profiling::function]
fn some_function() {
    burn_time(2);
}

#[profiling::function]
fn some_inner_function(_iteration_index: usize) {
    burn_time(1);
}

fn some_macro_function() {
    profiling::function_scope!();
    burn_time(5);
}

//
// Example of multiple scopes in a single function
//
fn some_other_function(iterations: usize) {
    profiling::scope!("some_other_function");
    burn_time(1);

    {
        profiling::scope!("do iterations");
        for i in 0..iterations {
            profiling::scope!(
                "some_inner_function_that_sleeps",
                format!("other data {}", i).as_str()
            );

            some_inner_function(i);
            burn_time(1);
        }
    }
}

// This function just spin-waits for some amount of time
fn burn_time(millis: u128) {
    let start_time = std::time::Instant::now();
    loop {
        if (std::time::Instant::now() - start_time).as_millis() > millis {
            break;
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp;

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        TemplateApp
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        // Generate some profiling info
        profiling::scope!("Main Thread");
        some_function();
        some_other_function(3);
        some_macro_function();

        Foo::function1();
        Foo::function2();

        println!("frame complete");

        puffin_egui::profiler_window(ctx);

        // Finish the frame.
        profiling::finish_frame!();
    }
}

fn main() -> eframe::Result<()> {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Enable puffin
    puffin::set_scopes_on(true);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )
}
