#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("head.s"));

/*
 * Method 1
 */
const PAGE_SIZE: usize = 4096;
static USER_STACK: [usize; PAGE_SIZE>>2] = [0; PAGE_SIZE>>2];
#[repr(C)]
pub struct Stack {
    a: &'static [usize],
    b: i16,
}
#[no_mangle]
pub static STACK_START: Stack = Stack {a: &USER_STACK, b: 0x10};

/*
 * Method 2
 */
global_asm!(r#"
stack_end:
    .space 0x10000
stack_start:
"#);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn printk() -> i32 {
    0
}

