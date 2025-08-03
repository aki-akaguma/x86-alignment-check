# Changelog: x86-alignment-check

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* `specs`


## [0.1.6] (2023-06-03)
### Fixed
* test bug: 'tests::it_works_3' panicked at 'misaligned pointer dereference: address must be a multiple of 0x4 but is 0x7f0c285eb573', src/lib.rs:169:40

## [0.1.5] (2023-04-01)
### Added
* example into `README.md`

## [0.1.4] (2023-04-01)
### Added
* `no_ac_call_once()`

### Changed
* rename `call_once()` to `ac_call_once()`

## [0.1.3] (2023-03-30)
### Added
* `call_once()`

## [0.1.2] (2023-03-30)
### Fixed
* typo: `x86_alignment_check()`

## [0.1.1] (2023-03-30)
### Added
* doc comment

### Fixed
* incorrect return value: `__read_eflags()`
* minimum support `1.59.0` for `core::arch::asm!()`

## [0.1.0] (2023-03-30)
* first commit

[Unreleased]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.6..HEAD
[0.1.6]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/x86-alignment-check/releases/tag/v0.1.0
