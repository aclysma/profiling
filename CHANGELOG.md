# Changelog

## 0.1.3
 * Re-export the profiler crates, simplifying changes needed in end-user's cargo.toml

## 0.1.2
 * Remove unintended and unnecessary dependencies from the procmacro crate
 * Republish the bindings crate mainly to fix readme typos and add a bit more info about the exposed APIs

## 0.1.1
 * Add profiling::function procmacro
 * Add profiling::register_thread!()
 * Add profiling::finish_frame!()
 * Add support for superluminal
 * Fixed incorrect usage of span!() that affected tracy captures
 * More examples, improved documentation

## 0.1.0
 * Initial release