#![no_std]              // no standard library
#![no_main]             // no main
#![feature(global_asm)] // need to use some assembly

use core::ptr;

mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn os_entrypoint() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"HatchOS AArch64 Bare Metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
