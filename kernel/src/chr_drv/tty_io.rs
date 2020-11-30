#![allow(dead_code, unused_macros, unused_imports)]

use include::linux::tty::*;
use include::signal::*;
use include::termios::*;
use crate::chr_drv::console::*;

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

static mut tty_table: [tty_struct; 1] = [
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

fn tty_init() {
    // rs_init();
    con_init();
}

fn tty_intr(tty: &tty_struct, mask: usize) {
    /*
    if (tty.pgrp <= 0) {
        return;
    }
    for i in 0..NR_TASKS {
        if (tasks[i] != 0 && task[i].pgrp == tty.pgrp) {
            task[i].signal |= mask;
        }
    }
    */
}

fn sleep_if_empty(queue: &tty_queue) {
    /*
    cli();
    while (!current->signal && EMPTY(*queue))
        interruptible_sleep_on(&queue->proc_list);
    sti();
    */
}

fn sleep_if_full(queue: &tty_queue) {
    /*
	if (!FULL(*queue))
		return;
	cli();
	while (!current->signal && LEFT(*queue)<128)
		interruptible_sleep_on(&queue->proc_list);
	sti();
    */
}


pub fn tty_write(channel: usize, buf: &[u8], nr: i32) -> i32 {
    static mut cr_flag: i32 = 0;
    let c: char;
    let b: &[u8] = buf;
    if (channel>2 || nr<0) {
        return -1;
    }
    unsafe {
        // let mut tty: &tty_struct = &mut tty_table[channel];
        for c in buf.iter() {
            PUTCH(*c, &mut tty_table[channel].write_q);
        }
        (tty_table[channel].write)(&mut tty_table[channel]);
    }
    /*
    	static int cr_flag=0;
	struct tty_struct * tty;
	char c, *b=buf;

	if (channel>2 || nr<0) return -1;
	tty = channel + tty_table;
	while (nr>0) {
		sleep_if_full(&tty->write_q);
		if (current->signal)
			break;
		while (nr>0 && !FULL(tty->write_q)) {
			c=get_fs_byte(b);
			if (O_POST(tty)) {
				if (c=='\r' && O_CRNL(tty))
					c='\n';
				else if (c=='\n' && O_NLRET(tty))
					c='\r';
				if (c=='\n' && !cr_flag && O_NLCR(tty)) {
					cr_flag = 1;
					PUTCH(13,tty->write_q);
					continue;
				}
				if (O_LCUC(tty))
					c=toupper(c);
			}
			b++; nr--;
			cr_flag = 0;
			PUTCH(c,tty->write_q);
		}
		tty->write(tty);
		if (nr>0)
			schedule();
	}
	return (b-buf);
    */
    0
}
