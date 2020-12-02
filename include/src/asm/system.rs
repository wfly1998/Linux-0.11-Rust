#![allow(dead_code)]

use crate::linux::head::*;

#[inline]
pub unsafe fn hlt() {
    llvm_asm!("hlt");
}
#[inline]
pub unsafe fn move_to_user_mode() {
    llvm_asm!(r#"movl %esp,%eax
                 pushl $0x17
                 pushl %eax
                 pushfl
                 pushl $0x0f
                 pushl $1f
                 iret
                 1:movl $0x17,%eax
                 movw %ax,%ds
                 movw %ax,%es
                 movw %ax,%fs
                 movw %ax,%gs"#
                 :::"ax");
}

#[inline]
pub unsafe fn sti() { llvm_asm!("sti"); }
#[inline]
pub unsafe fn cli() { llvm_asm!("cli"); }
#[inline]
pub unsafe fn nop() { llvm_asm!("nop"); }
#[inline]
pub unsafe fn iret() { llvm_asm!("iret"); }

#[inline]
unsafe fn _set_gate(gate_addr: &mut desc_struct, type_: usize, dpl: usize, addr: usize) {
    llvm_asm!(r#"movw %dx,%ax
                 movw $0,%dx
                 movl %eax,$1
                 movl %edx,$2"#
                 :
                 : "ri" ((0x8000+(dpl<<13)+(type_<<8)) as i16),
                 "o" (*(gate_addr as *const _ as *const char)),
                 "o" (*((4+(gate_addr as *const _ as usize)) as *const char)),
                 "{edx}" ((addr as *const char)),"{eax}" (0x00080000));
}

#[inline]
pub unsafe fn set_intr_gate(n: usize, addr: usize) {
    _set_gate(&mut idt[n], 14, 0, addr);
}

#[inline]
pub unsafe fn set_trap_gate(n: usize, addr: usize) {
    _set_gate(&mut idt[n], 15, 0, addr);
}

#[inline]
pub unsafe fn set_system_gate(n: usize, addr: usize) {
    _set_gate(&mut idt[n], 15, 3, addr);
}

/*
#define _set_seg_desc(gate_addr,type,dpl,base,limit) {\
	*(gate_addr) = ((base) & 0xff000000) | \
		(((base) & 0x00ff0000)>>16) | \
		((limit) & 0xf0000) | \
		((dpl)<<13) | \
		(0x00408000) | \
		((type)<<8); \
	*((gate_addr)+1) = (((base) & 0x0000ffff)<<16) | \
		((limit) & 0x0ffff); }
*/

macro_rules! _set_tssldt_desc {
    ($n: tt, $addr: tt, $type: literal) => {
        llvm_asm!(concat!(r#"movw $104,%1\n\t
                             movw %ax,%2\n\t
                             rorl $16,%eax\n\t
                             movb %al,%3\n\t
                             movb $"#, $type, r#",%4\n\t
                             movb $0x00,%5\n\t
                             movb %ah,%6\n\t
                             rorl $16,%eax"#)
                             ::"a" ($addr), "m" (*($n as *mut char)), "m" (*(($n+2) as *mut char)), "m" (*(($n+4) as *mut char)),
                             "m" (*(($n+5) as *mut char)), "m" (*(($n+6) as *mut char)), "m" (*(($n+7) as *mut char))
        );
    }
}

#[inline]
pub unsafe fn set_tss_desc(n: usize, addr: usize) { _set_tssldt_desc!(n, addr, "0x89"); }

#[inline]
pub unsafe fn set_ldt_desc(n: usize, addr: usize) { _set_tssldt_desc!(n, addr, "0x82"); }

