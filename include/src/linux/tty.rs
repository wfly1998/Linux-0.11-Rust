#![allow(unused_macros)]
use crate::termios::*;

pub struct tty_queue {
    data: usize,
    head: usize,
    tail: usize,
    // TODO: add proc_list in sched.rs
    // proc_list: [task_struct],
    buf: [u8; TTY_BUF_SIZE],
}

macro_rules! INC { ($($a: tt)+) => { $($a)+ = ($($a)+ + 1) & (TTY_BUF_SIZE - 1); } }
macro_rules! DEC { ($($a: tt)+) => { $($a)+ = ($($a)+ - 1) & (TTY_BUF_SIZE - 1); } }
macro_rules! EMPTY { ($($a: tt)+) => { $($a)+.head  == $($a)+.tail } }
macro_rules! LEFT { ($($a: tt)+) => { (a.tail-a.head-1) & (TTY_BUF_SIZE-1) } }
macro_rules! LAST { ($($a: tt)+) => { a.buf[(TTY_BUF_SIZE-1) & (a.head-1)] } }
macro_rules! FULL { ($($a: tt)+) => { LEFT!(a) == 0 } }
macro_rules! CHARS { ($($a: tt)+) => { (a.head-a.tail) & (TTY_BUF_SIZE-1) } }
macro_rules! GETCH { ($($queue: tt)+, $($c: tt)+) => { $($c)+ = $($queue)+.buf[$($queue)+.tail]; INC!($($queue)+.tail); } }
macro_rules! PUTCH { ($($queue: tt)+, $($c: tt)+) => { $($queue)+.buf[$($queue)+.head] = $($c)+; INC!($($queue)+.head); } }
macro_rules! INTR_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VINTR] } }
macro_rules! QUIT_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VQUIT] } }
macro_rules! ERASE_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VERASE] } }
macro_rules! KILL_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VKILL] } }
macro_rules! EOF_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VEOF] } }
macro_rules! START_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VSTART] } }
macro_rules! STOP_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VSTOP] } }
macro_rules! SUSPEND_CHAR { ($($tty: tt)+) => { $($tty)+.termios.c_cc[VSUSP] } }

#[repr(C)]
pub struct tty_struct {
    termios: termios,
    pgrp: isize,
    stopped: isize,
    write: fn(&tty_struct),
    read_q: tty_queue,
    write_q: tty_queue,
    secondary: tty_queue,
}

// extern "C" {
    // #[no_mangle]
    // static tty_table: *const tty_struct;
// }

pub const INIT_C_CC: [u8; 17] = [003, 034, 177, 025, 004, 0, 1, 0, 021, 023, 032, 0, 022, 017, 027, 026, 0];

