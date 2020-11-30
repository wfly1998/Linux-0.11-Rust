#[inline]
pub unsafe fn outb(value: u8, port: u8) {
    llvm_asm!("outb %al,%dx"::"{al}" (value),"{dx}" (port));
}

#[inline]
pub unsafe fn inb(port: u8) -> u8 {
    let _v: u8;
    llvm_asm!("inb %dx,%al":"={al}" (_v):"{dx}" (port));
    _v
}

#[inline]
pub unsafe fn outb_p(value: usize, port: u8) {
    llvm_asm!(r#"outb %al,%dx
                 jmp 1f
                 1:jmp 1f
                 1:"#
                 ::"{al}" (value),"{dx}" (port));
}

#[inline]
pub unsafe fn inb_p(port: u8) -> u8 {
    let _v: u8;
    llvm_asm!(r#"inb %dx,%al
                jmp 1f
                1:jmp 1f
                1:"#
                :"={al}" (_v):"{dx}" (port));
    _v
}

