/*!
`x86-alignment-check` is set `ac` flag in `eflags` on `x86` or `x86_64`

# Features
- set `ac` flag bit into ON, its included `eflags` of `x86`.
- `x86_64` are supported too.

# Example
```rust
    use x86_alignment_check::x86_alighment_check;
    //
    let old_flag = x86_alighment_check(true);
    // processing anythings, a bus error may occur.
    let _ = x86_alighment_check(old_flag);
```
*/
#![no_std]
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn x86_alighment_check(b: bool) -> bool {
    let old_eflags = unsafe { __read_eflags() };
    let new_eflags = if b {
        old_eflags | EFLAGS_AC_BIT
    } else {
        old_eflags & !EFLAGS_AC_BIT
    };
    unsafe { __write_eflags(new_eflags) };
    //
    let old_ac = unsafe { AC_ATOM.load(Ordering::Relaxed) };
    unsafe { AC_ATOM.store(b, Ordering::Relaxed) };
    //
    old_ac
}

static mut AC_ATOM: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "x86")]
const EFLAGS_AC_BIT: u32 = 1 << 18; // 0x0004_0000

#[cfg(target_arch = "x86_64")]
const EFLAGS_AC_BIT: u64 = 1 << 18; // 0x0004_0000

#[cfg(target_arch = "x86")]
#[inline(always)]
unsafe fn __read_eflags() -> u32 {
    let eflags: u32 = 0;
    core::arch::asm!("pushfd; pop {eflags:e}", eflags = out(reg) _);
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
    let rflags: u64 = 0;
    core::arch::asm!("pushfq; pop {rflags}", rflags = out(reg) _);
    rflags
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn __write_eflags(rflags: u64) {
    core::arch::asm!("push {rflags}; popfq", rflags = in(reg) rflags);
}

// reference:
// https://www.felixcloutier.com/x86/pushf:pushfd:pushfq

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let old_0 = x86_alighment_check(true);
        let old_1 = x86_alighment_check(true);
        let old_2 = x86_alighment_check(false);
        let old_3 = x86_alighment_check(true);
        let _old_4 = x86_alighment_check(old_0);
        //
        assert!(old_1);
        assert!(old_2);
        assert!(!old_3);
    }
    #[test]
    #[ignore]
    fn it_works_1() {
        let buf = [0_u8;100];
        //
        let _old_0 = x86_alighment_check(true);
        {
            let ptr = buf.as_ptr();
            let ptr = unsafe { ptr.add(3) };
            // next should "(signal: 7, SIGBUS: access to undefined memory)"
            let _v: u32 = unsafe { *(ptr as *const u32) };
        }
    }
}
