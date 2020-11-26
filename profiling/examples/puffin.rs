// In principle, this is what you need to run puffin:
// fn main() {
//     // Create the UI
//     let mut profiler_ui = puffin_imgui::ProfilerUi::default();
//
//     // Enable it
//     puffin::set_scopes_on(true);
//
//     loop {
//         profiling::scope!("Scope Name Here!");
//
//         // Draw the UI
//         profiler_ui.window(ui);
//
//         //
//         puffin::GlobalProfiler::lock().new_frame();
//     }
// }
//

use skulpin::app::TimeState;
use skulpin::skia_safe;
use skulpin::skia_safe::Canvas;
use skulpin::winit;
use skulpin::CoordinateSystemHelper;

use skulpin_plugin_imgui::imgui;
use skulpin_plugin_imgui::ImguiRendererPlugin;

mod imgui_support;
use imgui_support::ImguiManager;

// This encapsulates the demo logic
struct ExampleApp {
    profiler_ui: puffin_imgui::ProfilerUi,
}

impl ExampleApp {
    pub fn new() -> Self {
        ExampleApp {
            profiler_ui: puffin_imgui::ProfilerUi::default(),
        }
    }

    fn update(
        &mut self,
        _time_state: &TimeState,
    ) {
        profiling::scope!("update");
    }

    fn draw(
        &mut self,
        canvas: &mut Canvas,
        _coordinate_system_helper: &CoordinateSystemHelper,
        imgui_manager: &ImguiManager,
    ) {
        profiling::scope!("draw");
        //
        //Draw an inspect window for the example struct
        //
        imgui_manager.with_ui(|ui| {
            self.profiler_ui.window(ui);
        });

        //
        // Generally would want to clear data every time we draw
        //
        canvas.clear(skia_safe::Color::from_argb(0, 0, 0, 255));

        //
        // Make a color to draw with
        //
        let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(0.8, 0.8, 0.8, 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::StrokeAndFill);
        paint.set_stroke_width(1.0);

        //
        // Draw the circle that the user can manipulate
        //
        canvas.draw_circle(skia_safe::Point::new(200.0, 200.0), 100.0, &paint);

        //
        // Draw FPS text
        //
        let mut text_paint =
            skia_safe::Paint::new(skia_safe::Color4f::new(1.0, 1.0, 0.0, 1.0), None);
        text_paint.set_anti_alias(true);
        text_paint.set_style(skia_safe::paint::Style::StrokeAndFill);
        text_paint.set_stroke_width(1.0);

        //
        // Draw user's custom string
        //
        let mut font = skia_safe::Font::default();
        font.set_size(20.0);
        canvas.draw_str(
            imgui::im_str!("profiling demo using puffin"),
            (50, 100),
            &font,
            &text_paint,
        );
    }
}

// Creates a window and runs the event loop.
fn main() {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Enable puffin
    puffin::set_scopes_on(true);

    // Create the winit event loop
    let event_loop = winit::event_loop::EventLoop::<()>::with_user_event();

    // Set up the coordinate system to be fixed at 900x600, and use this as the default window size
    // This means the drawing code can be written as though the window is always 900x600. The
    // output will be automatically scaled so that it's always visible.
    let logical_size = winit::dpi::LogicalSize::new(900.0, 600.0);
    let visible_range = skulpin::skia_safe::Rect {
        left: 0.0,
        right: logical_size.width as f32,
        top: 0.0,
        bottom: logical_size.height as f32,
    };
    let scale_to_fit = skulpin::skia_safe::matrix::ScaleToFit::Center;

    // Create a single window
    let winit_window = winit::window::WindowBuilder::new()
        .with_title("Skulpin")
        .with_inner_size(logical_size)
        .build(&event_loop)
        .expect("Failed to create window");

    // Wrap it in an interface for skulpin to interact with the window
    let window = skulpin::WinitWindow::new(&winit_window);

    // Initialize imgui
    let imgui_manager = imgui_support::init_imgui_manager(&winit_window);

    // Initialize an interface for skulpin to interact with imgui
    let mut imgui_plugin: Option<Box<dyn skulpin::RendererPlugin>> = None;
    imgui_manager.with_context(|context| {
        imgui_plugin = Some(Box::new(ImguiRendererPlugin::new(context)));
    });

    // Create the renderer, which will draw to the window
    let renderer = skulpin::RendererBuilder::new()
        .use_vulkan_debug_layer(true)
        .coordinate_system(skulpin::CoordinateSystem::VisibleRange(
            visible_range,
            scale_to_fit,
        ))
        .add_plugin(imgui_plugin.unwrap())
        .build(&window);

    // Check if there were errors setting up vulkan
    if let Err(e) = renderer {
        println!("Error during renderer construction: {:?}", e);
        return;
    }

    let mut renderer = renderer.unwrap();

    let mut app = ExampleApp::new();
    let mut time_state = skulpin::app::TimeState::new();

    // Start the window event loop. Winit will not return once run is called. We will get notified
    // when important events happen.
    event_loop.run(move |event, _window_target, control_flow| {
        let window = skulpin::WinitWindow::new(&winit_window);

        imgui_manager.handle_event(&winit_window, &event);

        match event {
            //
            // Halt if the user requests to close the window
            //
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,

            //
            // Close if the escape key is hit
            //
            winit::event::Event::WindowEvent {
                event:
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,

            //
            // Request a redraw any time we finish processing events
            //
            winit::event::Event::MainEventsCleared => {
                time_state.update();

                app.update(&time_state);

                // Queue a RedrawRequested event.
                winit_window.request_redraw();
            }

            //
            // Redraw
            //
            winit::event::Event::RedrawRequested(_window_id) => {
                if let Err(e) = renderer.draw(&window, |canvas, coordinate_system_helper| {
                    imgui_manager.begin_frame(&winit_window);

                    app.draw(canvas, &coordinate_system_helper, &imgui_manager);

                    imgui_manager.render(&winit_window);
                }) {
                    println!("Error during draw: {:?}", e);
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }

                profiling::finish_frame!();
            }

            //
            // Ignore all other events
            //
            _ => {}
        }
    });
}
