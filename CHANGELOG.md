# Changelog: x86-alignment-check
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Include project specifications in `specs/`

## [0.1.6] - 2023-06-03
### Fixed
- Resolve panic in `tests::it_works_3` caused by misaligned pointer dereference

## [0.1.5] - 2023-04-01
### Added
- Provide example usage in `README.md`

## [0.1.4] - 2023-04-01
### Added
- Introduce `no_ac_call_once()` function

### Changed
- Rename `call_once()` to `ac_call_once()`

## [0.1.3] - 2023-03-30
### Added
- Provide `call_once()` function

## [0.1.2] - 2023-03-30
### Fixed
- Correct typo in `x86_alignment_check()`

## [0.1.1] - 2023-03-30
### Added
- Provide documentation comments

### Fixed
- Correct return value in `__read_eflags()`
- Update requirement for Rust 1.59.0 to support `core::arch::asm!`

## [0.1.0] - 2023-03-30
### Added
- Release initial version

[Unreleased]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.6..HEAD
[0.1.6]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/x86-alignment-check/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/x86-alignment-check/releases/tag/v0.1.0
