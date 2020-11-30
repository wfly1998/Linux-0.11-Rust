use include::ctype::*;
use include::asm::segment::*;
use include::asm::system::*;
use include::linux::sched::*;
use include::linux::tty::*;
use include::signal::*;
use include::termios::*;
use crate::chr_drv::console::*;

const ALRMMASK: u32 = (1 << SIGALRM) - 1;
const KILLMASK: u32 = (1 << SIGKILL) - 1;
const INTMASK:  u32 = (1 << SIGINT) - 1;
const QUITMASK: u32 = (1 << SIGQUIT) - 1;
const TSTPMASK: u32 = (1 << SIGTSTP) - 1;

#[inline] fn _L_FLAG(tty: &tty_struct, f: usize) -> usize { tty.termios.c_lflag & f }
#[inline] fn _I_FLAG(tty: &tty_struct, f: usize) -> usize { tty.termios.c_iflag & f }
#[inline] fn _O_FLAG(tty: &tty_struct, f: usize) -> usize { tty.termios.c_oflag & f }

#[inline] fn L_CANON(tty: &tty_struct) -> usize {  _L_FLAG(tty, ICANON) }
#[inline] fn L_ISIG(tty: &tty_struct) -> usize { _L_FLAG(tty, ISIG) }
#[inline] fn L_ECHO(tty: &tty_struct) -> usize { _L_FLAG(tty, ECHO) }
#[inline] fn L_ECHOE(tty: &tty_struct) -> usize { _L_FLAG(tty, ECHOE) }
#[inline] fn L_ECHOK(tty: &tty_struct) -> usize { _L_FLAG(tty, ECHOK) }
#[inline] fn L_ECHOCTL(tty: &tty_struct) -> usize { _L_FLAG(tty, ECHOCTL) }
#[inline] fn L_ECHOKE(tty: &tty_struct) -> usize { _L_FLAG(tty, ECHOKE) }

#[inline] fn I_UCLC(tty: &tty_struct) -> usize { _I_FLAG(tty, IUCLC) }
#[inline] fn I_NLCR(tty: &tty_struct) -> usize { _I_FLAG(tty, INLCR) }
#[inline] fn I_CRNL(tty: &tty_struct) -> usize { _I_FLAG(tty, ICRNL) }
#[inline] fn I_NOCR(tty: &tty_struct) -> usize { _I_FLAG(tty, IGNCR) }

#[inline] fn O_POST(tty: &tty_struct) -> usize { _O_FLAG(tty, OPOST) }
#[inline] fn O_NLCR(tty: &tty_struct) -> usize { _O_FLAG(tty, ONLCR) }
#[inline] fn O_CRNL(tty: &tty_struct) -> usize { _O_FLAG(tty, OCRNL) }
#[inline] fn O_NLRET(tty: &tty_struct) -> usize { _O_FLAG(tty, ONLRET) }
#[inline] fn O_LCUC(tty: &tty_struct) -> usize { _O_FLAG(tty, OLCUC) }

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

pub fn tty_init() {
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
    unsafe {
        cli();
        // while (!current->signal && EMPTY(*queue))
            // interruptible_sleep_on(&queue->proc_list);
        while (EMPTY(queue)) {
            hlt();
        }
        sti();
    }
}

fn sleep_if_full(queue: &tty_queue) {
    if (FULL(queue)) {
        return;
    }
    unsafe {
        cli();
        while (LEFT(queue) < 128) {
            hlt();
        }
        sti();
    }
    /*
    cli();
    while (!current->signal && LEFT(*queue)<128)
        interruptible_sleep_on(&queue->proc_list);
    sti();
    */
}


pub fn tty_write(channel: usize, buf: &[u8], mut nr: i32) -> i32 {
    let mut cr_flag: bool = false;
    let mut c: u8;
    let b: &[u8] = buf;
    let mut b_idx = 0;
    if (channel>2 || nr<0) {
        return -1;
    }
    let mut tty: &mut tty_struct = unsafe { &mut tty_table[channel] };
    while (nr > 0) {
        sleep_if_full(&tty.write_q);
        while (nr>0 && !FULL(&tty.write_q)) {
            c = get_fs_byte(&b[b_idx] as *const u8);
            if (O_POST(tty) != 0) {
                if (c == '\r' as u8 && O_CRNL(tty) != 0) {
                    c = '\n' as u8;
                } else if (c == '\n' as u8 && O_NLRET(tty) != 0) {
                    c = '\r' as u8;
                }
                if (c == '\n' as u8 && !cr_flag && O_NLCR(tty) != 0) {
                    cr_flag = true;
                    PUTCH(13, &mut tty.write_q);
                    continue;
                }
                if (O_LCUC(tty) != 0) {
                    c = toupper(c);
                }
            }
            b_idx += 1; nr -= 1;
            cr_flag = false;
            PUTCH(c, &mut tty.write_q);
        }
        (tty.write)(tty);
        if (nr > 0) {
            // schedule();
        }
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
