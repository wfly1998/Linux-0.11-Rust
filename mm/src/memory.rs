#![allow(dead_code, unused_variables)]
/*
use include::signal::*;
use include::asm::system::*;
use include::linux::head::*;
use include::linux::kernel::*;
use include::linux::sched::*;
*/
use kernel::println;

#[inline]
fn oom() {
    println!("out of memory");
    panic!("out of memory");
    // do_exit(SIGSEGV);
}

unsafe fn invalidate() {
    llvm_asm!("movl %%eax,%%cr3"::"{eax}" (0))
}

const LOW_MEM: usize = 0x100000;
const PAGING_MEMORY: usize = 15*1024*1024;
const PAGING_PAGES: usize = PAGING_MEMORY>>12;
#[inline] fn map_nr(addr: usize) -> usize { (addr-LOW_MEM) >> 12 }
const USED: u8 = 100;

static mut HIGH_MEMORY: usize = 0;

#[inline]
fn copy_page(from: usize, to: usize) {
    unsafe { llvm_asm!("cld ; rep ; movsl"::"S" (from),"D" (to),"c" (1024)) }
}

static mut MEM_MAP: [u8; PAGING_PAGES] = [0; PAGING_PAGES];


#[no_mangle]
pub fn mem_init(start_mem: usize, end_mem: usize)
{
    unsafe {
        HIGH_MEMORY = end_mem;
        for i in 0..PAGING_PAGES {
            MEM_MAP[i] = USED;
        }
        /*
        let mut i = map_nr(start_mem);
        let mut end_mem = (end_mem - start_mem) >> 12;
        while end_mem > 0 {
            end_mem -= 1;
            MEM_MAP[i]=0;
            i += 1;
        }   // error will occur if use this codes
        */
    }
}

