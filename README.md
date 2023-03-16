<div align="center">

# Velato

**An experimental [Lottie](https://airbnb.io/lottie) animation renderer built for [Vello](https://vello.dev)**

[![Xi Zulip](https://img.shields.io/badge/Xi%20Zulip-%23gpu-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/197075-gpu)
[![dependency status](https://deps.rs/repo/github/linebender/velato/status.svg)](https://deps.rs/repo/github/linebender/velato)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![wgpu version](https://img.shields.io/badge/wgpu-v0.15-orange.svg)](https://crates.io/crates/wgpu)
[![vello version](https://img.shields.io/badge/vello-v0.0.1-purple.svg)](https://vello.dev) <!-- https://crates.io/crates/vello -->
<!-- [![Crates.io](https://img.shields.io/crates/v/velato.svg)](https://crates.io/crates/velato) -->
<!-- [![Docs](https://docs.rs/velato/badge.svg)](https://docs.rs/velato) -->
<!-- [![Build status](https://github.com/linebender/velato/workflows/CI/badge.svg)](https://github.com/linebender/velato/actions) -->

</div>

It is not currently feature complete, but is capable of rendering a large number of Lottie animations.

## Examples

To run an example, use:
```
cargo run -p with_winit -- <PATH_TO_ANIMATION.json>
```

<!-- Synced with demo/README.md -->
Note that at the moment, we do not provide any example animations.
<!-- TODO: Make one -->

## Platforms

The current example does not support running on WASM or Android, however Velato should do so.

## Example usage

```rust
let lottie_data = std::fs::read("path/to/lottie.json").unwrap();
let composition = velato::Composition::from_bytes(&lottie_data).unwrap();
let mut renderer = velato::Renderer::new();
let scene = vello::Scene::new();
let mut builder = vello::SceneBuilder::for_scene(&mut scene);
let time_secs = 1.0;
let transform = kurbo::Affine::IDENTITY;
let alpha = 1.0;
renderer.render(&composition, time_secs, transform, alpha, &mut builder);
```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
