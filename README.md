# profiling

Provides an very thin abstraction over instrumented profiling crates like puffin, optick, and tracy.

## Who Is This For?
 * Authors of binaries that want to have multiple options for profiling their code, but don't want to duplicate their
   instrumentation once per each profiler's individual API.
 * Authors of libraries that would like to instrument their crate for their end-users.
 
This crate is intended to be as **TINY**. It won't support every possible usage, just the basics. I'm open to adding
a few more things but I plan to be very selective to maintain a very slim size.

Why not use the tracing crate? The tracing crate is significantly larger than necessary for this narrow use-case,
and it's expected that these scopes may end up on a very hot path where any overhead at all can add noise to the
captured data.

Why not use puffin/optick/etc. directly?
 * For authors of binaries, you'll still need to use APIs on those crates to get started. But when instrumenting your
   code, `profiling::scope!("Scope Name")` inside a function or `#[profiling::function]` on a function will instrument 
   it for all the supported profiler-specific crates. You can still use those crates directly if you want to take 
   advantage of custom APIs they provide to surface additional data.
 * For authors of upstream libraries, this crate lets you implement simple instrumentation once. Hopefully this will
   allow the community to benefit from instrumented profiling, even if a significant amount of a codebase is made
   of upstream crates.

## Using From a Binary

It's up to you to initialize the profiling crate of your choice. The examples demonstrate this, but it's worth looking
at the docs for the profiler your're interested in using! Once initialized, you can mix/match the macros provided
by your profiler of choice and the generic ones in this crate. For example:

```rust
// This may map to something like:
// - puffin::profile_scope!("Scope Name")
// - optick::event!("Scope Name")
// - tracing::span!(tracing::Level::INFO, $name)
profiling::scope!("Scope Name");

// This may map to something like:
// - puffin::profile_scope_data!("Scope Name", "extra data")
// - optick::event!("Scope Name"); optick::tag!("tag", "extra data");
// - tracing::span!(tracing::Level::INFO, "Scope Name", tag = "extra data")
profiling::scope!("Scope Name", "extra data");
```

There is also a proc macro to decorate functions:

```rust
#[profiling::function]
fn my_function() {

}
```

Take a look at the code for the helpful macros `register_thread!()` and `finish_frame!()`. 

If you want to enable profiling in upstream crates, you'll need to enable the appropriate features in them. This also
lets you turn them on/off per crate:

```toml
[dependencies]
puffin = { version = "0.3.1", optional = true }
optick = { version = "1.3.4", optional = true }
tracing = { version = "0.1", optional = true }
some_upstream_crate = "0.1"

[features]
profile-with-puffin = ["profiling/profile-with-puffin", "some_upstream_crate/profile-with-puffin", "puffin"]
profile-with-optick = ["profiling/profile-with-optick", "some_upstream_crate/profile-with-puffin", "optick"]
profile-with-tracing = ["profiling/profile-with-tracing", "some_upstream_crate/profile-with-puffin", "tracing"]
```

## Using From a Library

Add the profiling crate to Cargo.toml Add the following features but don't enable them. Those features should only be
enabled by the binary. If the end-user of your library doesn't use profiling, the macros in this crate will emit no code
at all.

```toml
[dependencies]
puffin = { version = "0.3.1", optional = true }
optick = { version = "1.3.4", optional = true }
tracing = { version = "0.1", optional = true }

[features]
profile-with-puffin = ["profiling/profile-with-puffin", "puffin"]
profile-with-optick = ["profiling/profile-with-optick", "optick"]
profile-with-tracing = ["profiling/profile-with-tracing", "tracing"]
```

The downstream binary can now turn these features on per crate by enabling the appropriate features within the crate as
described above.

## Feature Flags

 * profile-with-puffin: Enable the `puffin` crate
 * profile-with-optick: Enable the `optick` crate
 * profile-with-tracing: Enable the `tracing` crate. (The profiler crate `tracy` consumes data through this abstraction)

## Examples

 * simple: Shows a bare minimum requirents to do some simple instrumented profiling. Once it's running, you
   can connect to the process using optick or tracy

```
run --package profiling --example simple --features="profile-with-optick,profile-with-tracy,profile-with-puffin" 
```

 * puffin: Launches a basic app with imgui integration showing the puffin UI.
 
```
cargo run --package profiling --example puffin --features="profile-with-puffin"
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The examples directory contains [`mplus-1p-regular.ttf`](http://mplus-fonts.osdn.jp), available under its own license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
