#![no_main] // indicate to the compiler that we're not going to be using the usual startup chain
#![no_std] // remove links to the Rust STD because they rely on an underlying OS

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";


/// This function is called on panic.
/// We need to define this since we said we won't get a standard library.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// We're overwriting the OS' entry point, since we've removed our own main (since it
/// didn't have a runtime that would call it given we've used no_main)
#[no_mangle] // don't mangle the name of this function, so that the name is fixed, and we can pass it to the linker
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
