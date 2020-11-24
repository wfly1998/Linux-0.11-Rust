#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("head.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn printk() -> i32 {
    0
}

global_asm!(r#"
stack_end:
    .space 0x10000
stack_start:
"#);

