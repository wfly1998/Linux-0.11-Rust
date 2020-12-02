#![no_std]
#![no_main]
#![allow(const_err)]
#![feature(global_asm)]
#![feature(const_raw_ptr_deref)]

use core::panic::PanicInfo;

use include::asm::system::hlt;
use kernel::chr_drv::tty_io::*;
use kernel::trap::*;
use kernel::println;

global_asm!(include_str!("head.s"));

#[repr(C)]
#[derive(Clone, Copy)]
struct drive_info { dummy: [u8; 32], }

// extern "C" { static mut ROOT_DEV: usize; }
static mut memory_end: usize = 0;
static mut buffer_memory_end: usize = 0;
static mut main_memory_start: usize = 0;
static mut drive_info: drive_info = drive_info { dummy: [0; 32], };


#[no_mangle]
pub extern "C" fn main() -> ! {
    let EXT_MEM_K: u16 = unsafe { *(0x90002 as *const u16) };
    let DRIVE_INFO: drive_info = unsafe { *(0x90080 as *const drive_info) };
    let ORIG_ROOT_DEV: u16 = unsafe { *(0x901FC as *const u16) };
    unsafe {
        // ROOT_DEV = ORIG_ROOT_DEV as usize;
        drive_info = DRIVE_INFO;
        /*
        memory_end = (1usize<<20) + (EXT_MEM_K<<10) as usize;
        memory_end &= 0xfffff000;
        if (memory_end > 16*1024*1024) {
            memory_end = 16*1024*1024;
        }
        if (memory_end > 12*1024*1024)  {
            buffer_memory_end = 4*1024*1024;
        }
        else if (memory_end > 6*1024*1024) {
            buffer_memory_end = 2*1024*1024;
        }
        else {
            buffer_memory_end = 1*1024*1024;
        }
        main_memory_start = buffer_memory_end;
        */
    }

    trap_init();
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
