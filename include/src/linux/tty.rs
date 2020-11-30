use crate::termios::*;

pub struct tty_queue {
    pub data: usize,
    pub head: usize,
    pub tail: usize,
    // TODO: add proc_list in sched.rs
    // pub proc_list: [task_struct],
    pub buf: [u8; TTY_BUF_SIZE],
}

macro_rules! INC { ($($a: tt)+) => { $($a)+ = ($($a)+ + 1) & (TTY_BUF_SIZE - 1); } }
macro_rules! DEC { ($($a: tt)+) => { $($a)+ = ($($a)+ - 1) & (TTY_BUF_SIZE - 1); } }
#[inline]
pub fn EMPTY(a: &tty_queue) -> bool { a.head == a.tail }
#[inline]
pub fn LEFT(a: &tty_queue) -> usize { (a.tail-a.head-1) & (TTY_BUF_SIZE-1) }
#[inline]
pub fn LAST(a: &tty_queue) -> u8 { a.buf[(TTY_BUF_SIZE-1) & (a.head-1)] }
#[inline]
pub fn FULL(a: &tty_queue) -> bool { LEFT(a) == 0 }
#[inline]
pub fn CHARS(a: &tty_queue) -> usize { (a.head-a.tail) & (TTY_BUF_SIZE-1) }
#[inline]
pub fn GETCH(queue: &mut tty_queue) -> u8 { let c = queue.buf[queue.tail]; INC!(queue.tail); c }
#[inline]
pub fn PUTCH(c: u8, queue: &mut tty_queue) { queue.buf[queue.head] = c; INC!(queue.head); }
#[inline]
pub fn INTR_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VINTR as usize] }
#[inline]
pub fn QUIT_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VQUIT as usize] }
#[inline]
pub fn ERASE_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VERASE as usize] }
#[inline]
pub fn KILL_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VKILL as usize] }
#[inline]
pub fn EOF_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VEOF as usize] }
#[inline]
pub fn START_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VSTART as usize] }
#[inline]
pub fn STOP_CHART_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VSTOP as usize] }
#[inline]
pub fn SUSPEND_CHART_CHAR(tty: &tty_struct) -> u8 { tty.termios.c_cc[VSUSP as usize] }

#[repr(C)]
pub struct tty_struct {
    pub termios: termios,
    pub pgrp: isize,
    pub stopped: isize,
    pub write: fn(&mut tty_struct),
    pub read_q: tty_queue,
    pub write_q: tty_queue,
    pub secondary: tty_queue,
}

// extern "C" {
    // #[no_mangle]
    // static tty_table: *const tty_struct;
// }

pub const INIT_C_CC: [u8; 17] = [003, 034, 177, 025, 004, 0, 1, 0, 021, 023, 032, 0, 022, 017, 027, 026, 0];

