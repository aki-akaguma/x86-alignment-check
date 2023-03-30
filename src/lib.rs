/*!
`x86-alignment-check` is set `ac` flag in `eflags` on `x86` or `x86_64`

# Features
- set `ac` flag bit into ON, its included `eflags` of `x86`.
- `x86_64` are supported too.
- `#![no_std]`

# Example 1: If your code is correctly controlled by alignment
First, add the following to `Cargo.toml`:

```text
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

# Example 2: call_onece style
```rust
    let val = x86_alignment_check::call_once(|| {
        // processing anythings
        // return value for assertion
        1
    });
    assert_eq!(val, 1);
```
For now, assertions such as `assert_eq!()` cannot be included inside `FnOnce`,
because of the rust runtime bug.

*/
#![no_std]

/// alignment check flag manipulation
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn x86_alignment_check(b: bool) -> bool {
    let old_eflags = unsafe { __read_eflags() };
    let new_eflags = if b {
        old_eflags | EFLAGS_AC_BIT
    } else {
        old_eflags & !EFLAGS_AC_BIT
    };
    unsafe { __write_eflags(new_eflags) };
    //
    (old_eflags & EFLAGS_AC_BIT) != 0
}

#[cfg(target_arch = "x86")]
const EFLAGS_AC_BIT: u32 = 1 << 18; // 0x0004_0000

#[cfg(target_arch = "x86_64")]
const EFLAGS_AC_BIT: u64 = 1 << 18; // 0x0004_0000

#[cfg(target_arch = "x86")]
#[inline(always)]
unsafe fn __read_eflags() -> u32 {
    let mut eflags: u32;
    core::arch::asm!("pushfd; pop {eflags:e}", eflags = out(reg) eflags);
    eflags
}

#[cfg(target_arch = "x86")]
#[inline(always)]
unsafe fn __write_eflags(eflags: u32) {
    core::arch::asm!("push {eflags:e}; popfd", eflags = in(reg) eflags);
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn __read_eflags() -> u64 {
    let mut rflags: u64;
    core::arch::asm!("pushfq; pop {rflags}", rflags = out(reg) rflags);
    rflags
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn __write_eflags(rflags: u64) {
    core::arch::asm!("push {rflags}; popfq", rflags = in(reg) rflags);
}

/// execute under alignment check
pub fn call_once<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let old = x86_alignment_check(true);
    let r = f();
    let _ = x86_alignment_check(old);
    r
}

// reference:
// https://www.felixcloutier.com/x86/pushf:pushfd:pushfq

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let old_0 = x86_alignment_check(true);
        let old_1 = x86_alignment_check(true);
        let old_2 = x86_alignment_check(false);
        let old_3 = x86_alignment_check(true);
        let old_4 = x86_alignment_check(false);
        let _old_5 = x86_alignment_check(old_0);
        //
        assert!(old_1);
        assert!(old_2);
        assert!(!old_3);
        assert!(old_4);
    }
    #[test]
    fn it_works_1() {
        let val = call_once(|| 1);
        assert_eq!(val, 1);
    }
    #[test]
    #[ignore]
    fn it_works_ignore_0() {
        let buf = [0_u8; 100];
        //
        let _old_0 = x86_alignment_check(true);
        {
            let ptr = buf.as_ptr();
            let ptr = unsafe { ptr.add(3) };
            // next should "(signal: 7, SIGBUS: access to undefined memory)"
            let _v: u32 = unsafe { *(ptr as *const u32) };
        }
    }
}
