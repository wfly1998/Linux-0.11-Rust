#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

use kernel::chr_drv::console::con_init;
use kernel::chr_drv::tty_io::tty_write;

global_asm!(include_str!("head.s"));

const PAGE_SIZE: usize = 4096;
#[no_mangle]
static mut USER_STACK: [usize; PAGE_SIZE >> 2] = [0; PAGE_SIZE >> 2];
#[repr(C)]
pub struct Stack {
    a: *mut usize,
    b: i16,
}
#[no_mangle]
pub static mut STACK_START: Stack = Stack {
    a: unsafe { &mut USER_STACK as *mut [usize] as *mut usize },
    b: 0x10,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    con_init();
    tty_write(0, &['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8], 5);
    loop {}
}

#[no_mangle]
pub extern "C" fn printk() -> i32 {
    0
}
