# x86-alignment-check

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

`x86-alignment-check` is set `ac` flag in `eflags` on `x86` or `x86_64`

## Features
- set `ac` flag bit into ON, its included `eflags` of `x86`.
- `x86_64` are supported too.
- `#![no_std]`

## Example: If your code is correctly controlled by alignment
First, add the following to `Cargo.toml`:

```
[target.'cfg(any(target_arch = "x86_64", target_arch = "x86"))'.dev-dependencies]
x86-alignment-check = "*"
```

Second, enclose your test code with `x86_alignment_check()` as follows:

```rust
    use x86_alignment_check::x86_alignment_check;
    //
    let old_flag = x86_alignment_check(true);
    //
    // here your test codes, processing anythings, a bus error may occur.
    //
    let _ = x86_alignment_check(old_flag);
```

Finally execute `cargo test`


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/x86-alignment-check/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/x86-alignment-check.svg
[crate-link]: https://crates.io/crates/x86-alignment-check
[docs-image]: https://docs.rs/x86-alignment-check/badge.svg
[docs-link]: https://docs.rs/x86-alignment-check/
[rustc-image]: https://img.shields.io/badge/rustc-1.59+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/x86-alignment-check/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/x86-alignment-check/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/x86-alignment-check/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/x86-alignment-check/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/x86-alignment-check/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/x86-alignment-check/actions/workflows/test-windows.yml
