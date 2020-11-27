#![allow(dead_code, unused_macros, unused_imports)]

use include::linux::tty::*;
use include::signal::*;
use include::termios::*;

const ALRMMASK: u32 = (1 << SIGALRM) - 1;
const KILLMASK: u32 = (1 << SIGKILL) - 1;
const INTMASK:  u32 = (1 << SIGINT) - 1;
const QUITMASK: u32 = (1 << SIGQUIT) - 1;
const TSTPMASK: u32 = (1 << SIGTSTP) - 1;

macro_rules! _L_FLAG{ ($($tty: tt)+, $($f: tt)+) => { $($tty)+.termios.c_lflag & $($f)+ } }
macro_rules! _I_FLAG{ ($($tty: tt)+, $($f: tt)+) => { $($tty)+.termios.c_iflag & $($f)+ } }
macro_rules! _O_FLAG{ ($($tty: tt)+, $($f: tt)+) => { $($tty)+.termios.c_oflag & $($f)+ } }

macro_rules! L_CANON { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ICANON) } }
macro_rules! L_ISIG { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ISIG) } }
macro_rules! L_ECHO { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ECHO) } }
macro_rules! L_ECHOE { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ECHOE) } }
macro_rules! L_ECHOK { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ECHOK) } }
macro_rules! L_ECHOCTL { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ECHOCTL) } }
macro_rules! L_ECHOKE { ($($tty: tt)+) => { _L_FLAG!(($($tty)+), ECHOKE) } }

macro_rules! I_UCLC { ($($tty: tt)+) => { _I_FLAG!(($($tty)+), IUCLC) } }
macro_rules! I_NLCR { ($($tty: tt)+) => { _I_FLAG!(($($tty)+), INLCR) } }
macro_rules! I_CRNL { ($($tty: tt)+) => { _I_FLAG!(($($tty)+), ICRNL) } }
macro_rules! I_NOCR { ($($tty: tt)+) => { _I_FLAG!(($($tty)+), INOCR) } }

macro_rules! O_POST { ($($tty: tt)+) => { _O_FLAG!(($($tty)+), OPOST) } }
macro_rules! O_NLCR { ($($tty: tt)+) => { _O_FLAG!(($($tty)+), ONLCR) } }
macro_rules! O_CRNL { ($($tty: tt)+) => { _O_FLAG!(($($tty)+), OCRNL) } }
macro_rules! O_NLRET { ($($tty: tt)+) => { _O_FLAG!(($($tty)+), ONLRET) } }
macro_rules! O_LCUC { ($($tty: tt)+) => { _O_FLAG!(($($tty)+), OLCUC) } }

static tty_table: [tty_struct; 1] = [
    tty_struct {
        termios: termios {
            c_iflag: ICRNL,
            c_oflag: OPOST | ONLCR,
            c_cflag: 0,
            c_lflag: ISIG | ICANON | ECHO | ECHOCTL | ECHOKE,
            c_line: 0,
            c_cc: INIT_C_CC,
        },
        pgrp: 0,
        stopped: 0,
        write: con_write,
        read_q: tty_queue {
            data: 0,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        },
        write_q: tty_queue {
            data: 0,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        },
        secondary: tty_queue {
            data: 0,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        }
    }
];

fn tty_write(channel: usize, buf: &[u8], nr: i32) -> i32 {
    static mut cr_flag: i32 = 0;
    0
}
