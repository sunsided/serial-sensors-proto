# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Types, where possible, now implement `Ord`, `PartialOrd` and `Hash`.
- Added the `SensorId` type.
- Added the `Identification` and `LinearRanges` types for self-describing sensors.

## [0.2.3] - 2024-07-03

[0.2.3]: https://github.com/sunsided/serial-sensors-proto/releases/tag/v0.2.3

### Added

- Adds `From<F> for S` with fundamental types and sensor types.

## [0.2.2] - 2024-07-03

[0.2.2]: https://github.com/sunsided/serial-sensors-proto/releases/tag/v0.2.2

### Fixed

- Exposes the missing `Version1DataFrame`.

## [0.2.1] - 2024-07-03

[0.2.1]: https://github.com/sunsided/serial-sensors-proto/releases/tag/v0.2.1

### Fixed

- Fixed `micromath` and `quaternion` feature gates.

## [0.2.0] - 2024-07-03

[0.2.0]: https://github.com/sunsided/serial-sensors-proto/releases/tag/v0.2.0

### Added

- Added support for `micromath` types.
- Added `Index` and `IndexMut` support for fundamental types.
- Added `new` constructors for the fundamental types.
- Added destructuring into tuples and vectors for fundamental types.
- Added construction from tuples and vectors for fundamental types.
- Added `defmt` support.

### Internal

- Now using [uniform-array-derive](https://crates.io/crates/uniform-array-derive) to implement array-like
  behavior of fundamental types on `unsafe` crate feature.

### Changed

- The fundamental types (`ScalarData`, `Vector3Data`, ...) were made more easily accessible.

## [0.1.0] - 2024-07-03

[0.1.0]: https://github.com/sunsided/serial-sensors-proto/releases/tag/v0.1.0

### Added

- Initial release.
