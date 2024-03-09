<div align="center">

# Velato

**An integration to parse and render [Lottie](https://airbnb.io/lottie) with [Vello](https://vello.dev).**

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23gpu-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/197075-gpu)
[![dependency status](https://deps.rs/repo/github/linebender/velato/status.svg)](https://deps.rs/repo/github/linebender/velato)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![wgpu version](https://img.shields.io/badge/wgpu-v0.19-orange.svg)](https://crates.io/crates/wgpu)
[![vello version](https://img.shields.io/badge/vello-v0.1.0-purple.svg)](https://crates.io/crates/vello)

[![Crates.io](https://img.shields.io/crates/v/velato.svg)](https://crates.io/crates/velato)
[![Docs](https://docs.rs/velato/badge.svg)](https://docs.rs/velato)
[![Build status](https://github.com/linebender/velato/workflows/CI/badge.svg)](https://github.com/linebender/velato/actions)

</div>

> [!WARNING]
> The goal of this crate is to provide coverage of the large Lottie spec, up to what vello will support, for use in interactive graphics. We are working towards correctness, but there are several missing features. See [vello](https://github.com/linebender/vello) for more information about limitations.

## Missing features

It is not currently feature complete, but is capable of rendering a large number of Lottie animations.

- Non-linear easings
- Position keyframe (`ti`, `to`) easing
- Time remapping (`tm`)
- Text
- Image embedding
- Advanced shapes (stroke dash, zig-zag, etc.)
- Advanced effects (motion blur, drop shadows, etc.)
- Correct color stop handling
- Split rotations
- Split positions

## Examples

### Native

```shell
cargo run -p with_winit
```

You can also load an entire folder or individual files.

```shell
cargo run -p with_winit -- examples/assets
```

### Web

Because Vello relies heavily on compute shaders, we rely on the emerging WebGPU standard to run on the web.
Until browser support becomes widespread, it will probably be necessary to use development browser versions (e.g. Chrome Canary) and explicitly enable WebGPU.

This uses [`cargo-run-wasm`](https://github.com/rukai/cargo-run-wasm) to build the example for web, and host a local server for it

```shell
# Make sure the Rust toolchain supports the wasm32 target
rustup target add wasm32-unknown-unknown

# The binary name must also be explicitly provided as it differs from the package name
cargo run_wasm -p with_winit --bin with_winit_bin
```

There is also a web demo [available here](https://linebender.github.io/velato) on supporting web browsers.

> [!WARNING]
> The web is not currently a primary target for Vello, and WebGPU implementations are incomplete, so you might run into issues running this example.

## License

This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
