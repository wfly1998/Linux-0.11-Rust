#![allow(dead_code)]
use include::DEC;
use include::ctype::*;
use include::asm::segment::*;
use include::asm::system::*;
// use include::linux::sched::*;
use include::linux::tty::*;
use include::signal::*;
use include::termios::*;
use crate::chr_drv::console::*;
use crate::chr_drv::serial::*;

const ALRMMASK: u32 = (1 << SIGALRM) - 1;
const KILLMASK: u32 = (1 << SIGKILL) - 1;
const INTMASK:  u32 = (1 << SIGINT) - 1;
const QUITMASK: u32 = (1 << SIGQUIT) - 1;
const TSTPMASK: u32 = (1 << SIGTSTP) - 1;

#[inline] fn _l_flag(tty: &tty_struct, f: usize) -> usize { tty.termios.c_lflag & f }
#[inline] fn _i_flag(tty: &tty_struct, f: usize) -> usize { tty.termios.c_iflag & f }
#[inline] fn _o_flag(tty: &tty_struct, f: usize) -> usize { tty.termios.c_oflag & f }

#[inline] fn l_canon(tty: &tty_struct) -> usize {  _l_flag(tty, ICANON) }
#[inline] fn l_isig(tty: &tty_struct) -> usize { _l_flag(tty, ISIG) }
#[inline] fn l_echo(tty: &tty_struct) -> usize { _l_flag(tty, ECHO) }
#[inline] fn l_echoe(tty: &tty_struct) -> usize { _l_flag(tty, ECHOE) }
#[inline] fn l_echok(tty: &tty_struct) -> usize { _l_flag(tty, ECHOK) }
#[inline] fn l_echoctl(tty: &tty_struct) -> usize { _l_flag(tty, ECHOCTL) }
#[inline] fn l_echoke(tty: &tty_struct) -> usize { _l_flag(tty, ECHOKE) }

#[inline] fn i_uclc(tty: &tty_struct) -> usize { _i_flag(tty, IUCLC) }
#[inline] fn i_nlcr(tty: &tty_struct) -> usize { _i_flag(tty, INLCR) }
#[inline] fn i_crnl(tty: &tty_struct) -> usize { _i_flag(tty, ICRNL) }
#[inline] fn i_nocr(tty: &tty_struct) -> usize { _i_flag(tty, IGNCR) }

#[inline] fn o_post(tty: &tty_struct) -> usize { _o_flag(tty, OPOST) }
#[inline] fn o_nlcr(tty: &tty_struct) -> usize { _o_flag(tty, ONLCR) }
#[inline] fn o_crnl(tty: &tty_struct) -> usize { _o_flag(tty, OCRNL) }
#[inline] fn o_nlret(tty: &tty_struct) -> usize { _o_flag(tty, ONLRET) }
#[inline] fn o_lcuc(tty: &tty_struct) -> usize { _o_flag(tty, OLCUC) }

pub static mut TTY_TABLE: [tty_struct; 3] = [
    tty_struct {
        termios: termios {
            c_iflag: ICRNL,         /* change incoming CR to NL */
            c_oflag: OPOST | ONLCR, /* change outgoing NL to CRNL */
            c_cflag: 0,
            c_lflag: ISIG | ICANON | ECHO | ECHOCTL | ECHOKE,
            c_line: 0,              /* console termio */
            c_cc: INIT_C_CC,
        },
        pgrp: 0,                    /* initial pgrp */
        stopped: 0,                 /* initial stopped */
        write: con_write,
        read_q: tty_queue {         /* console read-queue */
            data: 0,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        },
        write_q: tty_queue {        /* console write-queue */
            data: 0,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        },
        secondary: tty_queue {      /* console secondary queue */
            data: 0,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        }
    },
    tty_struct {
        termios: termios {
            c_iflag: 0,             /* no translation */
            c_oflag: 0,             /* no translation */
            c_cflag: B2400 | CS8,
            c_lflag: 0,
            c_line: 0,
            c_cc: INIT_C_CC,
        },
        pgrp: 0,
        stopped: 0,
        write: rs_write,
        read_q: tty_queue {         /* rs 1 */
            data: 0x3f8,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        },
        write_q: tty_queue {
            data: 0x3f8,
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
    },
    tty_struct {
        termios: termios {
            c_iflag: 0,             /* no translation */
            c_oflag: 0,             /* no translation */
            c_cflag: B2400 | CS8,
            c_lflag: 0,
            c_line: 0,
            c_cc: INIT_C_CC,
        },
        pgrp: 0,
        stopped: 0,
        write: rs_write,
        read_q: tty_queue {         /* rs 2 */
            data: 0x2f8,
            head: 0,
            tail: 0,
            buf: [0; TTY_BUF_SIZE],
        },
        write_q: tty_queue {
            data: 0x2f8,
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
    },
];

pub fn tty_init() {
    rs_init();
    con_init();
}

fn tty_intr(tty: &tty_struct, mask: usize) {
    /*
    if tty.pgrp <= 0 {
        return;
    }
    for i in 0..NR_TASKS {
        if tasks[i] != 0 && task[i].pgrp == tty.pgrp {
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
        while EMPTY(queue) {
            hlt();
        }
        sti();
    }
}

fn sleep_if_full(queue: &tty_queue) {
    if FULL(queue) {
        return;
    }
    unsafe {
        cli();
        while LEFT(queue) < 128 {
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

fn wait_for_keypress() {
    unsafe { sleep_if_empty(&TTY_TABLE[0].secondary); }
}

fn copy_to_cooked(tty: &mut tty_struct) {
    let mut c: u8;

    while !EMPTY(&tty.read_q) && !FULL(&tty.secondary) {
        c = GETCH(&mut tty.read_q);
        if c == 13 {
            if i_crnl(tty) != 0 {
                c = 10;
            } else if i_nocr(tty) != 0 {
                continue;
            }
        } else if c == 10 && i_nlcr(tty) != 0 {
            c = 13;
        }
        if i_uclc(tty) != 0 {
            c = tolower(c);
        }
        if l_canon(tty) != 0 {
            if c==KILL_CHAR(tty) {
                /* deal with killing the input line */
                // while(!(EMPTY(tty->secondary) || (c=LAST(tty->secondary))==10 || c==EOF_CHAR(tty))) {
                loop {
                    if EMPTY(&tty.secondary) {
                        break;
                    }
                    c = LAST(&tty.secondary);
                    if c == 10 || c == EOF_CHAR(tty) {
                        break;
                    }
                    if l_echo(tty) != 0 {
                        if c < 32 {
                            PUTCH(127, &mut tty.write_q);
                        }
                        PUTCH(127, &mut tty.write_q);
                        (tty.write)(tty);
                    }
                    DEC!(tty.secondary.head);
                }
                continue;
            }
            if c==ERASE_CHAR(tty) {
                if EMPTY(&tty.secondary) {
                    continue;
                }
                c = LAST(&tty.secondary);
                if c == 10 || c == EOF_CHAR(tty) {
                    continue;
                }
                if l_echo(tty) != 0 {
                    if c < 32 {
                        PUTCH(127, &mut tty.write_q);
                    }
                    PUTCH(127, &mut tty.write_q);
                    (tty.write)(tty);
                }
                DEC!(tty.secondary.head);
                continue;
            }
            if c==STOP_CHAR(tty) {
                tty.stopped = 1;
                continue;
            }
            if c==START_CHAR(tty) {
                tty.stopped = 0;
                continue;
            }
        }
        if l_isig(tty) != 0 {
            if c == INTR_CHAR(tty) {
                tty_intr(tty, INTMASK as usize);
                continue;
            }
            if c == QUIT_CHAR(tty) {
                tty_intr(tty, QUITMASK as usize);
                continue;
            }
        }
        if c == 10 || c == EOF_CHAR(tty) {
            tty.secondary.data += 1;
        }
        if l_echo(tty) != 0 {
            if c == 10 {
                PUTCH(10, &mut tty.write_q);
                PUTCH(13, &mut tty.write_q);
            } else if c < 32 {
                if l_echoctl(tty) != 0 {
                    PUTCH('^' as u8, &mut tty.write_q);
                    PUTCH(c+64, &mut tty.write_q);
                }
            } else {
                PUTCH(c, &mut tty.write_q);
            }
            (tty.write)(tty);
        }
        PUTCH(c, &mut tty.secondary);
    }
    // wake_up(&tty.secondary.proc_list);
}

/*
fn tty_read(channel: usize, buf: &[u8], nr: usize) -> isize {
	struct tty_struct * tty;
	char c, * b=buf;
	int minimum,time,flag=0;
	long oldalarm;

	if channel>2 || nr<0 return -1;
	tty = &tty_table[channel];
	oldalarm = current->alarm;
	time = 10L*tty->termios.c_cc[VTIME];
	minimum = tty->termios.c_cc[VMIN];
	if time && !minimum {
		minimum=1;
		if (flag=(!oldalarm || time+jiffies<oldalarm))
			current->alarm = time+jiffies;
	}
	if minimum>nr
		minimum=nr;
	while (nr>0) {
		if flag && (current->signal & ALRMMASK) {
			current->signal &= ~ALRMMASK;
			break;
		}
		if current->signal
			break;
		if (EMPTY(tty->secondary) || (L_CANON(tty) &&
		!tty->secondary.data && LEFT(tty->secondary)>20)) {
			sleep_if_empty(&tty->secondary);
			continue;
		}
		do {
			GETCH(tty->secondary,c);
			if c==EOF_CHAR(tty) || c==10
				tty->secondary.data--;
			if c==EOF_CHAR(tty) && L_CANON(tty)
				return (b-buf);
			else {
				put_fs_byte(c,b++);
				if (!--nr)
					break;
			}
		} while (nr>0 && !EMPTY(tty->secondary));
		if (time && !L_CANON(tty)) {
			if ((flag=(!oldalarm || time+jiffies<oldalarm)))
				current->alarm = time+jiffies;
			else
				current->alarm = oldalarm;
		}
		if (L_CANON(tty)) {
			if (b-buf)
				break;
		} else if (b-buf >= minimum)
			break;
	}
	current->alarm = oldalarm;
	if (current->signal && !(b-buf))
		return -EINTR;
	return (b-buf);
}
*/

pub fn tty_write(channel: usize, buf: &[u8], mut nr: usize) -> isize {
    let mut cr_flag: bool = false;
    let mut c: u8;
    let b: &[u8] = buf;
    let mut b_idx: usize = 0;
    if channel>2 {
        return -1;
    }
    let mut tty: &mut tty_struct = unsafe { &mut TTY_TABLE[channel] };
    while nr > 0 {
        sleep_if_full(&tty.write_q);
        while nr>0 && !FULL(&tty.write_q) {
            c = get_fs_byte(&b[b_idx] as *const u8);
            if o_post(tty) != 0 {
                if c == '\r' as u8 && o_crnl(tty) != 0 {
                    c = '\n' as u8;
                } else if c == '\n' as u8 && o_nlret(tty) != 0 {
                    c = '\r' as u8;
                }
                if c == '\n' as u8 && !cr_flag && o_nlcr(tty) != 0 {
                    cr_flag = true;
                    PUTCH(13, &mut tty.write_q);
                    continue;
                }
                if o_lcuc(tty) != 0 {
                    c = toupper(c);
                }
            }
            b_idx += 1; nr -= 1;
            cr_flag = false;
            PUTCH(c, &mut tty.write_q);
        }
        (tty.write)(tty);
        if nr > 0 {
            // schedule();
        }
    }
    b_idx as isize
}

fn do_tty_interrupt(tty: usize) {
    unsafe { copy_to_cooked(&mut TTY_TABLE[tty]); }
}

fn chr_dev_init() {
}

