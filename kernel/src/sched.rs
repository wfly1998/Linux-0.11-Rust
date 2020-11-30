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

