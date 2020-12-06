pub const NR_TASKS: usize = 64;
pub const HZ: usize = 100;

pub const FIRST_TSS_ENTRY: usize = 4;
pub const FIRST_LDT_ENTRY: usize = FIRST_TSS_ENTRY+1;

#[inline]
fn _TSS(n: usize) -> usize {
    (n<<4)+(FIRST_TSS_ENTRY<<3)
}
#[inline]
fn _LDT(n: usize) -> usize {
    (n<<4)+(FIRST_LDT_ENTRY<<3)
}

#[inline]
pub unsafe fn ltr(n: usize) {
    llvm_asm!("ltr %ax"::"{ax}" (_TSS(n)));
}
#[inline]
pub unsafe fn lldt(n: usize) {
    llvm_asm!("lldt %ax"::"{ax}" (_LDT(n)));
}

