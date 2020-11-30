#![no_std]
#![allow(const_err)]
#![feature(llvm_asm)]
#![feature(const_raw_ptr_deref)]

pub mod chr_drv;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
