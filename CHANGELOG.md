# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

## [Unreleased]

This release has an [MSRV][] of 1.86.

## [0.8.1]

This release has an [MSRV][] of 1.86.

### Fixed

- A panic on WASM when finding roots.

## [0.8.0]

This release has an [MSRV][] of 1.86.

### Added

- Added image schema layer. ([#78][] by [@RobertBrewitz][])
- Added deserializer to deserialize into the correct Layer type based on the "ty" field. ([#78][] by [@RobertBrewitz][])
- Added twist, stroke_dash, modifier and rounded_corners schema shapes. ([#78][] by [@RobertBrewitz][])
- Added support for trim paths in animations. ([#83][] by [@RobertBrewitz][])

### Changed

- Changed the rust schema to better match the lottie spec. ([#78][] by [@RobertBrewitz][])

### Fixed

- Fixed tweening issue by replacing keyframe with kurbo cubic bezier implementation. ([#80][] & [#81][] by [@RobertBrewitz][])
- Fixed flickering of hold frames in splines. ([#82][] by [@RobertBrewitz][])
- Fixed matte layer clipping. ([#84][] by [@RobertBrewitz][])
- Fixed easing handles, only use the first two components ([#85][] by [@RobertBrewitz][])

### Removed

- Removed custom LayerId type in favor for new deserializer. ([#78][] by [@RobertBrewitz][])
- Removed unused layer enumeration. ([#78][] by [@RobertBrewitz][])
- Removed SKEW_LIMIT from Transform::evaluate method as it can cause rendering errors. ([#79][] by [@RobertBrewitz][])

## [0.7.0]  - 2025-10-11

This release has an [MSRV][] of 1.86.

### Changed

- Updated to vello 0.6

## [0.6.0]  - 2025-05-26

This release has an [MSRV][] of 1.85.

### Added

- Keyframes in an animated Spline may now specify an `"e"` key, dictating an end value. When specified, this value is used instead of the next keyframe's start value, allowing for discontinuous animations. ([#60][] by [@RishiChalla][])
- The last keyframe in an animated Spline may now specify only the timestamp, omitting all other fields. In this scenario, the previous keyframe's end/start values will be used. ([#60][] by [@RishiChalla][])

### Changed

- Updated to `vello` 0.5 ([#63][] by [@RobertBrewitz][])

## [0.5.0]  - 2025-02-02

This release has an [MSRV][] of 1.82.

### Changed

- Updated to `vello` 0.4 ([#49][] by [@simbleau][]).

## [0.4.0]  - 2024-11-21

This release has an [MSRV][] of 1.75.

### Changed

- Updated to `vello` 0.3
- Updated `thiserror` to 2.0

## [0.3.1]  - 2024-11-11

This release has an [MSRV][] of 1.75.

### Fixed

- Non-linear easing is now correctly interpolated ([#42] by [@atoktoto])

## [0.3.0]  - 2024-07-04

This release has an [MSRV][] of 1.75.

### Added

- Added `velato::Renderer::render`, which now returns a new vello scene.

### Changed

- Updated to vello 0.2
- Renamed `VelatoError` to `velato::Error`
- Renamed the existing `velato::Renderer::render` to `velato::Renderer::append`

### Removed

- All code and related profiling (`wgpu_profiler`) used in examples.

## [0.2.0] - 2024-05-26

This release has an [MSRV][] of 1.75.

### Changed

- Disable `vello`'s default `wgpu` feature, and provide a `wgpu` passthrough feature to turn it back on. ([#17] by [@MarijnS95])

### Fixed

- Image viewBox clipping is now applied to the animation ([#16] by [@luke1188])
- Errors that may occur on parsing a lottie composition are now public as `VelatoError`. ([#19] by [@simbleau])

## [0.1.0] - 2024-03-26

This release has an [MSRV][] of 1.75.

- Initial release

[@luke1188]: https://github.com/luke1188
[@MarijnS95]: https://github.com/MarijnS95
[@simbleau]: https://github.com/simbleau
[@atoktoto]: https://github.com/atoktoto
[@RishiChalla]: https://github.com/RishiChalla
[@RobertBrewitz]: https://github.com/RobertBrewitz

[#16]: https://github.com/linebender/velato/pull/16
[#17]: https://github.com/linebender/velato/pull/17
[#19]: https://github.com/linebender/velato/pull/19
[#42]: https://github.com/linebender/velato/pull/42
[#49]: https://github.com/linebender/velato/pull/49
[#60]: https://github.com/linebender/velato/pull/60
[#63]: https://github.com/linebender/velato/pull/63
[#78]: https://github.com/linebender/velato/pull/78
[#79]: https://github.com/linebender/velato/pull/79
[#80]: https://github.com/linebender/velato/pull/80
[#81]: https://github.com/linebender/velato/pull/81
[#82]: https://github.com/linebender/velato/pull/82
[#83]: https://github.com/linebender/velato/pull/83
[#84]: https://github.com/linebender/velato/pull/82
[#85]: https://github.com/linebender/velato/pull/85

[Unreleased]: https://github.com/linebender/velato/compare/v0.8.1...HEAD
[0.8.1]: https://github.com/linebender/velato/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/linebender/velato/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/linebender/velato/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/linebender/velato/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/linebender/velato/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/linebender/velato/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/linebender/velato/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/linebender/velato/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/linebender/velato/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/linebender/velato/releases/tag/v0.1.0

[MSRV]: README.md#minimum-supported-rust-version-msrv
