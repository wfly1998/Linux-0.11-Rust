#![no_std]
#![no_main]
#![allow(const_err)]
#![feature(global_asm)]
#![feature(const_raw_ptr_deref)]

use core::panic::PanicInfo;

use include::asm::system::*;
use kernel::chr_drv::tty_io::*;
use kernel::sched::*;
use kernel::trap::*;
use kernel::println;
use mm::memory::*;

// use lazy_static::lazy_static;

global_asm!(include_str!("head.s"));
global_asm!(include_str!("asm.s"));
// global_asm!(include_str!("keyboard.s"));
global_asm!(include_str!("system_call.s"));

#[repr(C)]
#[derive(Clone, Copy)]
struct drive_info_t { dummy: [u8; 32], }

// extern "C" { static mut ROOT_DEV: usize; }
static mut MEMORY_END: usize = 0;
static mut BUFFER_MEMORY_END: usize = 0;
static mut MAIN_MEMORY_START: usize = 0;
static mut DRIVE_INFO: drive_info_t = drive_info_t { dummy: [0; 32], };


#[no_mangle]
pub extern "C" fn main() -> ! {
    let ext_mem_k: u16 = unsafe { *(0x90002 as *const u16) };
    let _orig_root_dev: u16 = unsafe { *(0x901FC as *const u16) };
    unsafe {
        // ROOT_DEV = ORIG_ROOT_DEV as usize;
        DRIVE_INFO = *(0x90080 as *const drive_info_t);
        // /*
        MEMORY_END = (1usize<<20) + (ext_mem_k as usize) << 10;
        MEMORY_END &= 0xfffff000;
        if MEMORY_END > 16*1024*1024 {
            MEMORY_END = 16*1024*1024;
        }
        if MEMORY_END > 12*1024*1024  {
            BUFFER_MEMORY_END = 4*1024*1024;
        }
        else if MEMORY_END > 6*1024*1024 {
            BUFFER_MEMORY_END = 2*1024*1024;
        }
        else {
            BUFFER_MEMORY_END = 1*1024*1024;
        }
        MAIN_MEMORY_START = BUFFER_MEMORY_END;
        // */
    }

    unsafe {
        // println!("main_memory_start: {:x}, memory_end: {:x}", main_memory_start, memory_end);
        mem_init(MAIN_MEMORY_START, MEMORY_END);
    }
    trap_init();
    tty_init();
    sched_init();
    println!("Hello");
    loop {
        unsafe { hlt(); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

