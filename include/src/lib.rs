#![no_std]
#![feature(llvm_asm)]

pub mod asm;
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

