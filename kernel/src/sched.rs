use core::mem::transmute;
use include::asm::io::*;
use include::asm::system::*;
use include::linux::sched::*;
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

const LATCH: usize = (1193180/HZ);

pub fn sched_init() {
    // set_tss_desc(gdt[FIRST_TSS_ENTRY], &(init_task.task.tss));
    // set_tss_desc(gdt[FIRST_LDT_ENTRY], &(init_task.task.tss));
    /*
    let p = &gdt[2+FIRST_TSS_ENTRY];
    for i=1..NR_TASKS {
        task[i] = NULL;
        p.a = 0; p.b = 0;
        p += 1;
        p.a = 0; p.b = 0;
        p += 1;
    }
    */
    /* Clear NT, so that we won't have troubles with that later on */
    unsafe {
        // llvm_asm!("pushfl; andl 0xffffbfff, (%esp); popfl");
        ltr(0);
        lldt(0);
        outb_p(0x36, 0x43);		/* binary, mode 3, LSB/MSB, ch 0 */
        outb_p((LATCH & 0xff) as u8, 0x40);	/* LSB */
        outb((LATCH >> 8) as u8, 0x40);	/* MSB */
        // set_intr_gate(0x20,&timer_interrupt);
        outb(inb_p(0x21)&!0x01, 0x21);
        // set_system_gate(0x80,&system_call);
    }
}

