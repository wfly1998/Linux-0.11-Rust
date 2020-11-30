#![no_std]
#![feature(llvm_asm)]
#![feature(unchecked_math)]

pub mod asm;
pub mod ctype;
pub mod head;
pub mod linux;
pub mod signal;
pub mod termios;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

