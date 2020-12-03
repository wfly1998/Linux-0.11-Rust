use include::{INC, DEC};
use include::ctype::*;
use include::asm::segment::*;
use include::asm::system::*;
use include::linux::sched::*;
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

pub static mut tty_table: [tty_struct; 3] = [
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

fn wait_for_keypress() {
    unsafe { sleep_if_empty(&tty_table[0].secondary); }
}

fn copy_to_cooked(tty: &mut tty_struct) {
    let mut c: u8;

    while (!EMPTY(&tty.read_q) && !FULL(&tty.secondary)) {
        c = GETCH(&mut tty.read_q);
        if (c == 13) {
            if (I_CRNL(tty) != 0) {
                c = 10;
            } else if (I_NOCR(tty) != 0) {
                continue;
            }
        } else if (c == 10 && I_NLCR(tty) != 0) {
            c = 13;
        }
        if (I_UCLC(tty) != 0) {
            c = tolower(c);
        }
        if (L_CANON(tty) != 0) {
            if (c==KILL_CHAR(tty)) {
                /* deal with killing the input line */
                // while(!(EMPTY(tty->secondary) || (c=LAST(tty->secondary))==10 || c==EOF_CHAR(tty))) {
                loop {
                    if (EMPTY(&tty.secondary)) {
                        break;
                    }
                    c = LAST(&tty.secondary);
                    if (c == 10 || c == EOF_CHAR(tty)) {
                        break;
                    }
                    if (L_ECHO(tty) != 0) {
                        if (c<32) {
                            PUTCH(127, &mut tty.write_q);
                        }
                        PUTCH(127, &mut tty.write_q);
                        (tty.write)(tty);
                    }
                    DEC!(tty.secondary.head);
                }
                continue;
            }
            if (c==ERASE_CHAR(tty)) {
                if (EMPTY(&tty.secondary)) {
                    continue;
                }
                c = LAST(&tty.secondary);
                if (c == 10 || c == EOF_CHAR(tty)) {
                    continue;
                }
                if (L_ECHO(tty) != 0) {
                    if (c<32) {
                        PUTCH(127, &mut tty.write_q);
                    }
                    PUTCH(127, &mut tty.write_q);
                    (tty.write)(tty);
                }
                DEC!(tty.secondary.head);
                continue;
            }
            if (c==STOP_CHAR(tty)) {
                tty.stopped = 1;
                continue;
            }
            if (c==START_CHAR(tty)) {
                tty.stopped = 0;
                continue;
            }
        }
        if (L_ISIG(tty) != 0) {
            if (c == INTR_CHAR(tty)) {
                tty_intr(tty, INTMASK as usize);
                continue;
            }
            if (c == QUIT_CHAR(tty)) {
                tty_intr(tty, QUITMASK as usize);
                continue;
            }
        }
        if (c == 10 || c == EOF_CHAR(tty)) {
            tty.secondary.data += 1;
        }
        if (L_ECHO(tty) != 0) {
            if (c == 10) {
                PUTCH(10, &mut tty.write_q);
                PUTCH(13, &mut tty.write_q);
            } else if (c < 32) {
                if (L_ECHOCTL(tty) != 0) {
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

	if (channel>2 || nr<0) return -1;
	tty = &tty_table[channel];
	oldalarm = current->alarm;
	time = 10L*tty->termios.c_cc[VTIME];
	minimum = tty->termios.c_cc[VMIN];
	if (time && !minimum) {
		minimum=1;
		if ((flag=(!oldalarm || time+jiffies<oldalarm)))
			current->alarm = time+jiffies;
	}
	if (minimum>nr)
		minimum=nr;
	while (nr>0) {
		if (flag && (current->signal & ALRMMASK)) {
			current->signal &= ~ALRMMASK;
			break;
		}
		if (current->signal)
			break;
		if (EMPTY(tty->secondary) || (L_CANON(tty) &&
		!tty->secondary.data && LEFT(tty->secondary)>20)) {
			sleep_if_empty(&tty->secondary);
			continue;
		}
		do {
			GETCH(tty->secondary,c);
			if (c==EOF_CHAR(tty) || c==10)
				tty->secondary.data--;
			if (c==EOF_CHAR(tty) && L_CANON(tty))
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
    if (channel>2) {
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
    b_idx as isize
}

fn do_tty_interrupt(tty: usize) {
    unsafe { copy_to_cooked(&mut tty_table[tty]); }
}

fn chr_dev_init() {
}

