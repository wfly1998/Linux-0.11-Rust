#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

use include::asm::system::hlt;
use kernel::chr_drv::tty_io::*;

global_asm!(include_str!("head.s"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    tty_init();
    tty_write(0, &['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8], 5);
    loop {
        unsafe {
            hlt();
        }
    }
}

#[no_mangle]
pub extern "C" fn printk() -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
