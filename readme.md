# oxidize-rb

[![Join the discussion](https://img.shields.io/badge/slack-chat-blue.svg)](https://join.slack.com/t/oxidize-rb/shared_invite/zt-16zv5tqte-Vi7WfzxCesdo2TqF_RYBCw) 

The goal of `oxidize-rb` is to provide developers with tooling to build safe
Ruby extensions in Rust.

## rb-sys

Autogenerated Rust bindings for Ruby. Uses the [`rust-bindgen`](https://github.com/rust-lang/rust-bindgen) crate to generate bindings from the `ruby.h` header.https://github.com/rust-lang/rust-bindgen

### Usage

1. Add the following to your `Cargo.toml`:

   ```toml
   [dependencies]
   rb-sys = "0.8.0"
   ```

2. [See this example of creating a Ruby gem in Rust](./examples/rust_reverse)

### Linking libruby

If you need to link libruby (i.e. you are initializing a Ruby VM in your Rust code), use can enable the `link-ruby` feature:

```rust
rb-sys = { version = "0.8.0",  features = ["link-ruby"] }
```

### Static libruby

You can also force the `link-ruby` feature to be static:

```rust
rb-sys = { version = "0.8.0", features = ["link-ruby", "ruby-static"] }
```

Alternatively, you can set the `RUBY_STATIC=true` environment variable.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
