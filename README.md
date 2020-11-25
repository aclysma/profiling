# profiling

Provides an very thin abstraction over instrumented profiling crates like puffin, optick, and tracy. There are two
main usecases:
 * Add profiling to a binary
 * Intrumenting a library so that downstream binaries can see scopes from your library
 
Why not use the tracing crate? The tracing crate is significantly larger than necessary for this particular use-case,
and it's very likely these scopes may end up on a very hot path where any overhead at all can add noise to the captured
data.

This crate is intended to be as small as possible, providing support for operations that are fairly universal.

## Using From a Binary

It's up to you to initialize the profiling crate of your choice. Once initialized, you can use BOTH the macros provided
by that crate or the generic ones in this crate. For example:
```rust
// Depending on the features you enable on the profiling crate this may map to something like:
// - puffin::profile_scope!("Scope Name")
// - optick::event!("Scope Name")
profiling::profile_scope!("Scope Name");
```

Or:

```rust
// Depending on the features you enable on the profiling crate this may map to something like:
// - puffin::profile_scope_data!("Scope Name", "tag");
// - optick::event!("Scope Name"); optick::tag("tag");
profiling::profile_scope!("Scope Name", "tag");
```

This gets mapped into something like `puffin::profile_scope!("Scope Name")` or `optick::event!("Scope Name")`
depending on the features that are enabled.

## Using From a Library

Add the profiling crate to Cargo.toml. Don't use any features. Those features should only be enabled by the binary. If
the end-user of your library doesn't use profiling, the macros in this crate will emit no code at all.

Not every feature will be exposed, so in some cases it still might make sense to import specific profiling crates if
you want to add specific functionality that is offered by a particular profiler crate.

## Feature Flags

profile-with-puffin: Enable the `puffin` crate
profile-with-optick: Enable the `optick` crate
profile-with-tracing: Enable the `tracing` crate. (The profiler crate `tracy` consumes data through this abstraction)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
