use core::fmt::{self, Write};
use crate::chr_drv::tty_io::tty_write;

#[no_mangle]
fn print(p: usize) -> isize {    // p: *const char
    // private, this func is for asm
    let mut i: usize = 0;
    loop {
        let ptr = (p + i) as *const u8;
        let v = unsafe { *ptr };
        if (v == 0) {
            break;
        }
        i += 1;
        tty_write(0, &[v; 1], 1);
    }
    i as isize
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        tty_write(0, s.as_bytes(), s.len());
        Ok(())
    }
}

pub fn print_args(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print::print_args(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print::print_args(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

