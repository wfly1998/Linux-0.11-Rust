use core::mem::transmute;
use core::slice::from_raw_parts;

use include::asm::io::*;
use include::asm::system::*;
use crate::{print, println};

global_asm!(include_str!("asm.s"));

#[inline]
unsafe fn get_seg_byte(seg: u16, addr: usize) -> u8 {
    let __res: u8;
    llvm_asm!("push %fs; mov %ax, %fs; movb %fs:(%edx), %eax; pop %fs":"=a" (__res):"0" (seg), "d" (addr));
    __res
}

#[inline]
unsafe fn get_seg_long(seg: u16, addr: usize) -> usize {
    let __res: usize;
    llvm_asm!("push %fs; mov %ax, %fs; movl %fs:(%edx), %eax; pop %fs":"=a" (__res):"0" (seg), "d" (addr));
    __res
}

#[inline]
unsafe fn _fs() -> u16 {
    let __res: u16;
    llvm_asm!("mov %fs, %ax": "={ax}" (__res));
    __res
}


pub fn die(s: &str, esp_ptr: usize, nr: usize) {
    let esp: &[usize] = unsafe { from_raw_parts(esp_ptr as *const usize, 7) };
    println!("{}: {:04x}", s, nr&0xffff);
    println!("EIP:\t{:04x}:{:04x}\nEFLAGS:\t{:04x}\nESP:\t{:04x}:{:04x}", esp[1], esp[0], esp[2], esp[4], esp[3]);
    println!("fs: {:04x}\n", unsafe { _fs() } );
    // println!("base: {04:x}, limit: {04:x}\n");
    /*
    if (esp[4] == 0x17) {
        print!("Stack:");
        for i in 0..4 {
            print!(" {:04x}", get_seg_long(0x17, i+esp[3]));
        }
        print!("\n");
    }
    */
    // println!("Pid: {}, process nr: {}");
    /*
    for i in 0..10 {
        print!("{:02x} ", 0xff&get_seg_byte(esp[1], (i+esp[0])));
    }
    */
    print!("\n");
    panic!();
}

extern "C" {
    fn divide_error();
    fn debug();
    fn nmi();
    fn int3();
    fn overflow();
    fn bounds();
    fn invalid_op();
    fn device_not_available();
    fn double_fault();
    fn coprocessor_segment_overrun();
    fn invalid_TSS();
    fn segment_not_present();
    fn stack_segment();
    fn general_protection();
    fn page_fault();
    fn reserved();
    fn coprocessor_error();
    fn parallel_interrupt();
    fn irq13();
}

#[no_mangle]
fn do_double_fault(esp: usize, error_code: usize) {
    die("double fault", esp, error_code);
}

#[no_mangle]
fn do_general_protection(esp: usize, error_code: usize) {
    die("general protection",esp,error_code);
}

#[no_mangle]
fn do_divide_error(esp: usize, error_code: usize) {
    die("divide error",esp,error_code);
}

#[no_mangle]
fn do_int3(esp: usize, error_code: usize,
		fs: usize, es: usize, ds: usize,
		ebp: usize, esi: usize, edi: usize,
		edx: usize, ecx: usize, ebx: usize, eax: usize) {
    let tr: usize;
    let esp_array: &[usize] = unsafe { from_raw_parts(esp as *const usize, 3) };
    unsafe { llvm_asm!("str %ax":"={ax}" (tr):"0" (0)); }
    println!("eax\t\tebx\t\tecx\t\tedx\n{:08x}\t{:08x}\t{:08x}\t{:08x}", eax, ebx, ecx, edx);
    println!("esi\t\tedi\t\tebp\t\tesp\n{:08x}\t{:08x}\t{:08x}\t{:08x}", esi, edi, ebp, esp);
    println!("\nds\tes\tfs\ttr\n{:04x}\t{:04x}\t{:04x}\t{:04x}", ds, es, fs, tr);
    println!("EIP: {:08x}\tCS: {:04x}\tEFLAGS: {:08x}", esp_array[0], esp_array[1], esp_array[2]);
}

#[no_mangle]
fn do_nmi(esp: usize, error_code: usize) {
    die("nmi", esp, error_code);
}

#[no_mangle]
fn do_debug(esp: usize, error_code: usize) {
    die("debug", esp, error_code);
}

#[no_mangle]
fn do_overflow(esp: usize, error_code: usize) {
    die("overflow", esp, error_code);
}

#[no_mangle]
fn do_bounds(esp: usize, error_code: usize) {
    die("bounds", esp, error_code);
}

#[no_mangle]
fn do_invalid_op(esp: usize, error_code: usize) {
    die("invalid operand", esp, error_code);
}

#[no_mangle]
fn do_device_not_available(esp: usize, error_code: usize) {
    die("device not available", esp, error_code);
}

#[no_mangle]
fn do_coprocessor_segment_overrun(esp: usize, error_code: usize) {
    die("coprocessor segment overrun", esp, error_code);
}

#[no_mangle]
fn do_invalid_TSS(esp: usize, error_code: usize) {
    die("invalid TSS", esp, error_code);
}

#[no_mangle]
fn do_segment_not_present(esp: usize, error_code: usize) {
    die("segment not present", esp, error_code);
}

#[no_mangle]
fn do_stack_segment(esp: usize, error_code: usize) {
    die("stack segment", esp, error_code);
}

#[no_mangle]
fn do_coprocessor_error(esp: usize, error_code: usize) {
    // if (last_task_used_math != current) {
        // return;
    // }
    die("coprocessor error", esp, error_code);
}

#[no_mangle]
fn do_reserved(esp: usize, error_code: usize) {
    die("reserved (15,17-47) error", esp, error_code);
}

pub fn trap_init() {
    unsafe {
        set_trap_gate(0, transmute(&divide_error));
        set_trap_gate(1, transmute(&debug));
        set_trap_gate(2, transmute(&nmi));
        set_system_gate(3, transmute(&int3));    /* int3-5 can be called from all */
        set_system_gate(4, transmute(&overflow));
        set_system_gate(5, transmute(&bounds));
        set_trap_gate(6, transmute(&invalid_op));
        set_trap_gate(7, transmute(&device_not_available));
        set_trap_gate(8, transmute(&double_fault));
        set_trap_gate(9, transmute(&coprocessor_segment_overrun));
        set_trap_gate(10, transmute(&invalid_TSS));
        set_trap_gate(11, transmute(&segment_not_present));
        set_trap_gate(12, transmute(&stack_segment));
        set_trap_gate(13, transmute(&general_protection));
        set_trap_gate(14, transmute(&page_fault));
        set_trap_gate(15, transmute(&reserved));
        set_trap_gate(16, transmute(&coprocessor_error));
        for i in 17..48 {
            set_trap_gate(i, transmute(&reserved));
        }
        set_trap_gate(45, transmute(&irq13));
        outb_p(inb_p(0x21)&0xfb, 0x21);
        outb(inb_p(0xA1)&0xdf, 0xA1);
        set_trap_gate(39, transmute(&parallel_interrupt));
    }
}

