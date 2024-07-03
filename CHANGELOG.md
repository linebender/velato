# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

## Unreleased

## 0.3.0

### Added

- Added `velato::Renderer::render`, which now returns a new vello scene.

### Changed

- Updated to vello 0.2
- Renamed `VelatoError` to `velato::Error`
- Renamed the existing `velato::Renderer::render` to `velato::Renderer::append`

### Removed

- All code and related profiling (`wgpu_profiler`) used in examples.

## 0.2.0

### Changed

- Disable `vello`'s default `wgpu` feature, and provide a `wgpu` passthrough feature to turn it back on. ([#17](https://github.com/linebender/velato/pull/17))

### Fixed

- Image viewBox clipping is now applied to the animation ([#16](https://github.com/linebender/velato/pull/16))
- Errors that may occur on parsing a lottie composition are now public as `VelatoError`. ([#19](https://github.com/linebender/velato/pull/19))

## 0.1.0 (2024-03-26)

- Initial release
