# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

## [Unreleased]

This release has an [MSRV][] of 1.75.

## [0.4.0]  - 2024-11-11

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

[#16]: https://github.com/linebender/velato/pull/16
[#17]: https://github.com/linebender/velato/pull/17
[#19]: https://github.com/linebender/velato/pull/19
[#42]: https://github.com/linebender/velato/pull/42

[Unreleased]: https://github.com/linebender/velato/compare/v0.3.0...HEAD
[0.4.0]: https://github.com/linebender/velato/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/linebender/velato/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/linebender/velato/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/linebender/velato/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/linebender/velato/releases/tag/v0.1.0

[MSRV]: README.md#minimum-supported-rust-version-msrv
