#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"HatchOS x86_64";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // vga address for x86 processors

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // set the character
            *vga_buffer.offset(i as isize * 2) = byte;
            // set its color, attributes, etc
            *vga_buffer.offset(i as isize * 2 + 1) = (i+1) as u8;
        }
    }

    loop {}
}

// because we have #![no_std] we have to have our own panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
