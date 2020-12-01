#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

use include::asm::system::hlt;
use kernel::chr_drv::tty_io::*;
use kernel::*;

global_asm!(include_str!("head.s"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    tty_init();
    println!("Hello");
    loop {
        unsafe {
            hlt();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
