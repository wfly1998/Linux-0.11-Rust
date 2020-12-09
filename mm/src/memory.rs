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
const PAGING_MEMORY: usize = (15*1024*1024);
const PAGING_PAGES: usize = (PAGING_MEMORY>>12);
#[inline] fn MAP_NR(addr: usize) -> usize { (addr-LOW_MEM) >> 12 }
const USED: u8 = 100;

static mut HIGH_MEMORY: usize = 0;

#[inline]
fn copy_page(from: usize, to: usize) {
    unsafe { llvm_asm!("cld ; rep ; movsl"::"S" (from),"D" (to),"c" (1024)) }
}

static mut mem_map: [u8; PAGING_PAGES] = [0; PAGING_PAGES];


pub fn mem_init(start_mem: usize, mut end_mem: usize)
{
    unsafe {
        HIGH_MEMORY = end_mem;
        for i in 0..PAGING_PAGES {
            mem_map[i] = USED;
        }
        let mut i = MAP_NR(start_mem);
        end_mem -= start_mem;
        end_mem >>= 12;
        while (end_mem > 0) {
            end_mem -= 1;
            mem_map[i]=0;
            i += 1;
        }
    }
}

