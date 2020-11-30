#![allow(dead_code)]

#[inline]
pub fn get_fs_byte(addr: *const u8) -> u8 {
    let _v: u8;
    unsafe { llvm_asm!("movb %fs:$1,$0":"=r" (_v):"m" (*addr)); }
    _v
}

#[inline]
pub fn get_fs_word(addr: *const u16) -> u16 {
    let _v: u16;
    unsafe { llvm_asm!("movb %fs:$1,$0":"=r" (_v):"m" (*addr)); }
    _v
}

#[inline]
pub fn get_fs_long(addr: *const u32) -> u32 {
    let _v: u32;
    unsafe { llvm_asm!("movl %fs:$1,$0":"=r" (_v):"m" (*addr)); }
    _v
}

#[inline]
pub fn put_fs_byte(val: u8, addr: *mut u8) {
    unsafe { llvm_asm!("movb $0,%fs:$1"::"r" (val),"m" (*addr)); }
}

#[inline]
pub fn put_fs_word(val: u16, addr: *mut u16) {
    unsafe { llvm_asm!("movw $0,%fs:$1"::"r" (val),"m" (*addr)); }
}

#[inline]
pub fn put_fs_long(val: u32, addr: *mut u32) {
    unsafe { llvm_asm!("movl $0,%fs:$1"::"r" (val),"m" (*addr)); }
}

/*
 * Someone who knows GNU asm better than I should double check the followig.
 * It seems to work, but I don't know if I'm doing something subtly wrong.
 * --- TYT, 11/24/91
 * [ nothing wrong here, Linus ]
 */

#[inline]
pub fn get_fs() -> u32 {
    let _v: u16;
    unsafe { llvm_asm!("mov %fs,%ax":"=a" (_v):); }
    _v as u32
}

#[inline]
pub fn get_ds() -> u32 {
    let _v: u16;
    unsafe { llvm_asm!("mov %ds,%ax":"=a" (_v):); }
    _v as u32
}

#[inline]
pub fn set_fs(val: u32) {
    unsafe { llvm_asm!("mov $0,%fs"::"a" (val as u16)); }
}

