#![no_std]
#![no_main]
#![allow(non_snake_case)]

use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};

#[xdp]
pub fn xdpHandler(_ctx: XdpContext) -> u32 {
  xdp_action::XDP_PASS
}

// The panic_handler attribute defines the function that the compiler should invoke when a panic
// occurs. The standard library provides its own panic handler function, but in a no_std environment
// we need to define it ourselves.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

/*
  Language items are special functions and types that are required internally by the compiler.

  The eh_personality language item marks a function that is used for implementing stack unwinding.
  By default, Rust uses unwinding to run the destructors of all live stack variables in case of a
  panic. This ensures that all used memory is freed and allows the parent thread to catch the
  panic and continue execution.

  Unwinding, however, is a complicated process and requires some OS-specific libraries (e.g.
  libunwind on Linux), so we'll disable it, by setting panic = "abort" in Cargo.toml.
*/
