use core::mem::transmute;

use include::asm::io::*;
use include::asm::system::*;
use include::linux::tty::*;
use crate::chr_drv::tty_io::TTY_TABLE;

extern "C" {
    fn rs1_interrupt();
    fn rs2_interrupt();
}

unsafe fn init(port: u16) {
    outb_p(0x80,port+3);    /* set DLAB of line control reg */
    outb_p(0x30,port);      /* LS of divisor (48 -> 2400 bps */
    outb_p(0x00,port+1);    /* MS of divisor */
    outb_p(0x03,port+3);    /* reset DLAB */
    outb_p(0x0b,port+4);    /* set DTR,RTS, OUT_2 */
    outb_p(0x0d,port+1);    /* enable all intrs but writes */
    inb(port);        /* read data port to reset things (?) */
}

pub fn rs_init() {
    unsafe {
        set_intr_gate(0x24, transmute(&rs1_interrupt));
        set_intr_gate(0x23, transmute(&rs2_interrupt));
        init(TTY_TABLE[1].read_q.data as u16);
        init(TTY_TABLE[2].read_q.data as u16);
        outb(inb_p(0x21)&0xE7,0x21);
    }
}

/*
 * This routine gets called when tty_write has put something into
 * the write_queue. It must check wheter the queue is empty, and
 * set the interrupt register accordingly
 *
 *      void _rs_write(struct tty_struct * tty);
 */
pub fn rs_write(tty: &mut tty_struct) {
    unsafe {
        cli();
        if !EMPTY(&tty.write_q) {
            outb(inb_p(tty.write_q.data as u16+1)|0x02,tty.write_q.data as u16+1);
        }
        sti();
    }
}

