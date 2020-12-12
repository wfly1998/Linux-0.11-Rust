#![allow(dead_code, non_snake_case, unused_variables)]

use core::mem::transmute;

use include::asm::io::*;
use include::asm::system::*;
use include::linux::sched::*;
use include::linux::tty::*;
// use crate::chr_drv::tty_io::*;


const VIDEO_TYPE_MDA: u8 = 0x10;    /* Monochrome Text Display	*/
const VIDEO_TYPE_CGA: u8 = 0x11;    /* CGA Display 			*/
const VIDEO_TYPE_EGAM: u8 = 0x20;   /* EGA/VGA in Monochrome Mode	*/
const VIDEO_TYPE_EGAC: u8 = 0x21;   /* EGA/VGA in Color Mode	*/

const NPAR: usize = 16;

extern "C" {
    fn keyboard_interrupt();
}

static mut VIDEO_TYPE: u8 = 0;              /* Type of display being used	*/
#[no_mangle]
static mut VIDEO_NUM_COLUMNS: usize = 0;    /* Number of text columns	*/
static mut VIDEO_SIZE_ROW: usize = 0;       /* Bytes per row		*/
static mut VIDEO_NUM_LINES: usize = 0;      /* Number of test lines		*/
static mut VIDEO_PAGE: u8 = 0;              /* Initial video page		*/
static mut VIDEO_MEM_START: usize = 0;      /* Start of video RAM		*/
static mut VIDEO_MEM_END: usize = 0;        /* End of video RAM (sort of)	*/
static mut VIDEO_PORT_REG: u16 = 0;         /* Video register select port	*/
static mut VIDEO_PORT_VAL: u16 = 0;	    /* Video register value port	*/
static mut VIDEO_ERASE_CHAR: u16 = 0;       /* Char+Attrib to erase with	*/

static mut ORIGIN: usize = 0;  /* Used for EGA/VGA fast scroll	*/
static mut SCR_END: usize = 0; /* Used for EGA/VGA fast scroll	*/
static mut POS: usize = 0;
static mut X: usize = 0;
static mut Y: usize = 0;
static mut TOP: usize = 0;
static mut BOTTOM: usize = 0;
static mut STATE: usize = 0;
static mut NPAR_: usize = 0;
static mut PAR: [usize; NPAR] = [0; NPAR];
static mut QUES: bool = false;
static mut ATTR: u8 = 0x07;

/*
 * this is what the terminal answers to a ESC-Z or csi0c
 * query (= vt100 response).
 */
const RESPONSE: [u8; 7] = [33, '[' as u8, '?' as u8, '1' as u8, ';' as u8, '2' as u8, 'c' as u8];

/* NOTE! gotoxy thinks X==video_num_columns is ok */
#[inline]
fn gotoxy(new_x: usize, new_y: usize) {
    unsafe {
        if new_x > VIDEO_NUM_COLUMNS || new_y >= VIDEO_NUM_LINES {
            return;
        }
        X=new_x;
        Y=new_y;
        POS=ORIGIN + Y*VIDEO_SIZE_ROW + (X<<1);
    }
}

#[inline]
fn set_origin() {
    unsafe {
        cli();
        outb_p(12, VIDEO_PORT_REG);
        outb_p((0xff&((ORIGIN-VIDEO_MEM_START)>>9)) as u8, VIDEO_PORT_VAL);
        outb_p(13, VIDEO_PORT_REG);
        outb_p((0xff&((ORIGIN-VIDEO_MEM_START)>>1)) as u8, VIDEO_PORT_VAL);
        sti();
    }
}

unsafe fn scrup() {
    if VIDEO_TYPE == VIDEO_TYPE_EGAC || VIDEO_TYPE == VIDEO_TYPE_EGAM {
        if TOP == 0 && BOTTOM == VIDEO_NUM_LINES {
            ORIGIN += VIDEO_SIZE_ROW;
            POS += VIDEO_SIZE_ROW;
            SCR_END += VIDEO_SIZE_ROW;
            if SCR_END > VIDEO_MEM_END {
                llvm_asm!(r#"cld
                             rep
                             movsl
                             movl VIDEO_NUM_COLUMNS,$1
                             rep
                             stosw"#
                             ::"{eax}" (VIDEO_ERASE_CHAR),
                             "{ecx}" ((VIDEO_NUM_LINES-1)*VIDEO_NUM_COLUMNS>>1),
                             "{edi}" (VIDEO_MEM_START),
                             "{esi}" (ORIGIN)
                         );
                SCR_END -= ORIGIN-VIDEO_MEM_START;
                POS -= ORIGIN-VIDEO_MEM_START;
                ORIGIN = VIDEO_MEM_START;
            } else {
                llvm_asm!(r#"cld
                             rep
                             stosw"#
                             ::"{eax}" (VIDEO_ERASE_CHAR),
                             "{ecx}" (VIDEO_NUM_COLUMNS),
                             "{edi}" (SCR_END-VIDEO_SIZE_ROW)
                         );
            }
            set_origin();
        } else {
            llvm_asm!(r#"cld
                         rep
                         movsl
                         movl VIDEO_NUM_COLUMNS,%ecx
                         rep
                         stosw"#
                         ::"{eax}" (VIDEO_ERASE_CHAR),
                         "{ecx}" ((BOTTOM-TOP-1)*VIDEO_NUM_COLUMNS>>1),
                         "{edi}" (ORIGIN+VIDEO_SIZE_ROW*TOP),
                         "{esi}" (ORIGIN+VIDEO_SIZE_ROW*(TOP+1))
                     );
        }
    }
    else		/* Not EGA/VGA */
    {
        llvm_asm!(r#"cld
                     rep
                     movsl
                     movl VIDEO_NUM_COLUMNS,%ecx
                     rep
                     stosw"#
                     ::"{eax}" (VIDEO_ERASE_CHAR),
                     "{ecx}" ((BOTTOM-TOP-1)*VIDEO_NUM_COLUMNS>>1),
                     "{edi}" (ORIGIN+VIDEO_SIZE_ROW*TOP),
                     "{esi}" (ORIGIN+VIDEO_SIZE_ROW*(TOP+1))
                 );
    }
}

unsafe fn scrdown() {
    if VIDEO_TYPE == VIDEO_TYPE_EGAC || VIDEO_TYPE == VIDEO_TYPE_EGAM {
        llvm_asm!(r#"std
                     rep
                     movsl
                     addl $2,%edi
                     movl VIDEO_NUM_COLUMNS,%ecx
                     rep
                     stosw"#
                     ::"{eax}" (VIDEO_ERASE_CHAR),
                     "{ecx}" ((BOTTOM-TOP-1)*VIDEO_NUM_COLUMNS>>1),
                     "{edi}" (ORIGIN+VIDEO_SIZE_ROW*BOTTOM-4),
                     "{esi}" (ORIGIN+VIDEO_SIZE_ROW*(BOTTOM-1)-4)
                 );
    }
    else		/* Not EGA/VGA */
    {
        llvm_asm!(r#"std
                     rep
                     movsl
                     addl $2,%edi
                     movl VIDEO_NUM_COLUMNS,%ecx
                     rep
                     stosw"#
                     ::"{eax}" (VIDEO_ERASE_CHAR),
                     "{ecx}" ((BOTTOM-TOP-1)*VIDEO_NUM_COLUMNS>>1),
                     "{edi}" (ORIGIN+VIDEO_SIZE_ROW*BOTTOM-4),
                     "{esi}" (ORIGIN+VIDEO_SIZE_ROW*(BOTTOM-1)-4)
                 );
    }
}

unsafe fn lf() {
    if Y+1<BOTTOM {
        Y += 1;
        POS += VIDEO_SIZE_ROW;
        return;
    }
    scrup();
}

unsafe fn ri() {
    if Y>TOP {
        Y -= 1;
        POS -= VIDEO_SIZE_ROW;
        return;
    }
    scrdown();
}

unsafe fn cr() {
    POS -= X<<1;
    X = 0;
}

unsafe fn del() {
    if X != 0 {
        POS -= 2;
        X -= 1;
        *(POS as *mut u16) = VIDEO_ERASE_CHAR;
    }
}

unsafe fn csi_J(par_: usize) {
    let count: usize;
    let start: usize;

    match par_ {
        0 => {  /* erase from cursor to end of display */
            count = (SCR_END - POS) >> 1;
            start = POS;
        },
        1 => {  /* erase from start to cursor */
            count = (POS - ORIGIN) >> 1;
            start = ORIGIN;
        },
        2 => {  /* erase whole display */
            count = VIDEO_NUM_COLUMNS * VIDEO_NUM_LINES;
            start = ORIGIN;
        },
        _ => {
            return;
        }
    }
    llvm_asm!(r#"cld
                 rep
                 stosw"#
                 ::"{ecx}" (count),
                 "{edi}" (start),"{ax}" (VIDEO_ERASE_CHAR)
             );
}

unsafe fn csi_K(par_: usize) {
    let count: usize;
    let start: usize;

    match par_ {
        0 => {  /* erase from cursor to end of line */
            if X >= VIDEO_NUM_COLUMNS {
                return;
            }
            count = VIDEO_NUM_COLUMNS - X;
            start = POS;
        },
        1 => {  /* erase from start of line to cursor */
            start = POS - (X << 1);
            count = if X < VIDEO_NUM_COLUMNS { X } else { VIDEO_NUM_COLUMNS };
        },
        2 => {  /* erase whole line */
            start = POS - (X << 1);
            count = VIDEO_NUM_COLUMNS;
        },
        _ => {
            return;
        }
    }
    llvm_asm!(r#"cld
                 rep
                 stosw"#
                 ::"{ecx}" (count),
                 "{edi}" (start),"{eax}" (VIDEO_ERASE_CHAR)
             );
}

unsafe fn csi_m() {
    for i in 0..=NPAR_ {
        match PAR[i] {
            0 | 27 => ATTR = 0x07,
            1 | 4 => ATTR = 0x0f,
            7 => ATTR = 0x70,
            _ => {},
        }
    }
}

#[inline]
unsafe fn set_cursor() {
    cli();
    outb_p(14, VIDEO_PORT_REG);
    outb_p((0xff&((POS-VIDEO_MEM_START)>>9)) as u8, VIDEO_PORT_VAL);
    outb_p(15, VIDEO_PORT_REG);
    outb_p((0xff&((POS-VIDEO_MEM_START)>>1)) as u8, VIDEO_PORT_VAL);
    sti();
}

unsafe fn respond(tty: &mut tty_struct) {
    cli();
    for p in RESPONSE.iter() {
        PUTCH(*p, &mut tty.read_q);
    }
    sti();
    // copy_to_cooked(tty);
}

unsafe fn insert_char() {
    let mut i: usize = X;
    let mut tmp: u16;
    let mut old: u16 = VIDEO_ERASE_CHAR;
    let mut p: *mut u16 = POS as *mut u16;
    while i < VIDEO_NUM_COLUMNS {
        i += 1;
        tmp = *p;
        *p = old;
        old = tmp;
        p = ((p as *const _ as usize) + 2) as *mut u16;
    }
}

unsafe fn insert_line() {
    let oldtop: usize = TOP;
    let oldbottom: usize = BOTTOM;
    TOP = Y;
    BOTTOM = VIDEO_NUM_LINES;
    scrdown();
    TOP = oldtop;
    BOTTOM = oldbottom;
}

unsafe fn delete_char() {
    let mut i: usize = X;
    let mut p: *mut u16 = POS as *mut u16;
    if X >= VIDEO_NUM_COLUMNS {
        return;
    }
    i += 1;
    while i < VIDEO_NUM_COLUMNS {
        let tmp: *mut u16 = ((p as *const _ as usize) + 2) as *mut u16;
        *p = *tmp;
        p = tmp;
        i += 1;
    }
    *p = VIDEO_ERASE_CHAR;
}

unsafe fn delete_line() {
    let oldtop: usize = TOP;
    let oldbottom: usize = BOTTOM;
    TOP = Y;
    BOTTOM = VIDEO_NUM_LINES;
    scrup();
    TOP = oldtop;
    BOTTOM = oldbottom;
}

unsafe fn csi_at(mut nr: usize) {
    if nr > VIDEO_NUM_COLUMNS {
        nr = VIDEO_NUM_COLUMNS;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        insert_char();
    }
}

unsafe fn csi_L(mut nr: usize) {
    if nr > VIDEO_NUM_LINES {
        nr = VIDEO_NUM_LINES;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        insert_line();
    }
}

unsafe fn csi_P(mut nr: usize) {
    if nr > VIDEO_NUM_COLUMNS {
        nr = VIDEO_NUM_COLUMNS;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        delete_char();
    }
}

unsafe fn csi_M(mut nr: usize) {
    if nr > VIDEO_NUM_LINES {
        nr = VIDEO_NUM_LINES;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        delete_line();
    }
}

static mut SAVED_X: usize = 0;
static mut SAVED_Y: usize = 0;

unsafe fn save_cur() {
    SAVED_X = X;
    SAVED_Y = Y;
}

unsafe fn restore_cur() {
    gotoxy(SAVED_X, SAVED_Y);
}

pub fn con_write(tty: &mut tty_struct) {
    unsafe {
        let nr = CHARS(&tty.write_q);
        for _ in 0..nr{
            let mut c: u8 = GETCH(&mut tty.write_q);
            if STATE == 0 {
                if c>31 && c<127 {
                    if X>=VIDEO_NUM_COLUMNS {
                        X -= VIDEO_NUM_COLUMNS;
                        POS -= VIDEO_SIZE_ROW;
                        lf();
                    }
                    llvm_asm!("movw %ax, (%edx)"
                              ::"{ah}"(ATTR), "{al}"(c), "{edx}"(POS));  
                    POS += 2;
                    X += 1;
                } else if c==27 {
                    STATE=1;
                }
                else if c==10 || c==11 || c==12 {
                    lf();
                } else if c==13 {
                    cr();
                } else if c==ERASE_CHAR(tty) {
                    del();
                } else if c==8 {
                    if X != 0 {
                        X -= 1;
                        POS -= 2;
                    }
                } else if c==9 {
                    c=8 - (X&7) as u8;
                    X += c as usize;
                    POS += (c as usize)<<1;
                    if X>VIDEO_NUM_COLUMNS {
                        X -= VIDEO_NUM_COLUMNS;
                        POS -= VIDEO_SIZE_ROW;
                        lf();
                    }
                    c = 9;
                } else if c==7 {
                    sysbeep();
                }
            } else if STATE == 1 {
                STATE=0;
                if c=='[' as u8 {
                    STATE=2;
                } else if c=='E' as u8 {
                    gotoxy(0,Y+1);
                } else if c=='M' as u8 {
                    ri();
                } else if c=='D' as u8 {
                    lf();
                } else if c=='Z' as u8 {
                    respond(tty);
                } else if X=='7' as usize {
                    save_cur();
                } else if X=='8' as usize {
                    restore_cur();
                }
            } else {
                loop {
                    if STATE == 2 {
                        for npar_ in 0..NPAR {
                            PAR[npar_] = 0;
                        }
                        NPAR_ = NPAR;
                        STATE = 3;
                        QUES = c == '?' as u8;
                        if QUES {
                            break;
                        }
                    } else if STATE == 3 {
                        if c==';' as u8 && NPAR_<NPAR-1 {
                            NPAR_ += 1;
                            break;
                        } else if c>=('0' as u8) && c<=('9' as u8) {
                            PAR[NPAR_] = 10 * PAR[NPAR_] + (c - ('0' as u8)) as usize;
                            break;
                        } else {
                            STATE = 4;
                        }
                    } else if STATE == 4 {
                        STATE = 0;
                        match c as char {
                            'G' | '`' => {
                                if PAR[0] != 0 {
                                    PAR[0] -= 1;
                                }
                                gotoxy(PAR[0], Y);
                            },
                            'A' => {
                                if PAR[0] == 0 {
                                    PAR[0] += 1;
                                }
                                gotoxy(X, Y-PAR[0]);
                            },
                            'B' | 'e' => {
                                if PAR[0] == 0 {
                                    PAR[0] += 1;
                                }
                                gotoxy(X, Y+PAR[0]);
                            },
                            'C' | 'a' => {
                                if PAR[0] == 0 {
                                    PAR[0] += 1;
                                }
                                gotoxy(X+PAR[0], Y);
                            },
                            'D' => {
                                if PAR[0] == 0 {
                                    PAR[0] += 1;
                                }
                                gotoxy(X-PAR[0], Y);
                            },
                            'E' => {
                                if PAR[0] == 0 {
                                    PAR[0] += 1;
                                }
                                gotoxy(0, Y+PAR[0]);
                            },
                            'F' => {
                                if PAR[0] == 0 {
                                    PAR[0] += 1;
                                }
                                gotoxy(0, Y-PAR[0]);
                            },
                            'd' => {
                                if PAR[0] != 0 {
                                    PAR[0] -= 1;
                                }
                                gotoxy(X, PAR[0]);
                            },
                            'H' | 'f' => {
                                if PAR[0] != 0 {
                                    PAR[0] -= 1;
                                }
                                if PAR[1] != 0 {
                                    PAR[1] -= 1;
                                }
                                gotoxy(PAR[1], PAR[0]);
                            },
                            'J' => {
                                csi_J(PAR[0]);
                            },
                            'K' => {
                                csi_K(PAR[0]);
                            },
                            'L' => {
                                csi_L(PAR[0]);
                            },
                            'M' => {
                                csi_M(PAR[0]);
                            },
                            'P' => {
                                csi_P(PAR[0]);
                            },
                            '@' => {
                                csi_at(PAR[0]);
                            },
                            'm' => {
                                csi_m();
                            },
                            'r' => {
                                if PAR[0] != 0 {
                                    PAR[0] -= 1;
                                }
                                if PAR[1] == 0 {
                                    PAR[1] = VIDEO_NUM_LINES;
                                }
                                if PAR[0] < PAR[1] && PAR[1] <= VIDEO_NUM_LINES {
                                    TOP = PAR[0];
                                    BOTTOM = PAR[1];
                                }
                            },
                            's' => {
                                save_cur();
                            },
                            'u' => {
                                restore_cur();
                            },
                            _ => {}
                        }
                    }
                    break;
                }
            }
        }
        set_cursor();
    }
}

/*
 *  void con_init(void);
 *
 * This routine initalizes console interrupts, and does nothing
 * else. If you want the screen to clear, call tty_write with
 * the appropriate escape-sequece.
 *
 * Reads the information preserved by setup.s to determine the current display
 * type and sets everything accordingly.
 */
pub fn con_init() {
    let orig_x: u8 = unsafe { *(0x90000 as *const u8) };
    let orig_y: u8 = unsafe { *(0x90001 as *const u8) };
    let orig_video_page: u16 = unsafe { *(0x90004 as *const u16) };
    let orig_video_mode: u16 = unsafe { *(0x90006 as *const u16) & 0xff };
    let orig_video_cols: u16 = unsafe { (*(0x90006 as *const u16) & 0xff00) >> 8 };
    let orig_video_lines: usize = 25;
    let orig_video_ega_ax: u16 = unsafe { *(0x90008 as *const u16) };
    let orig_video_ega_bx: u16 = unsafe { *(0x9000a as *const u16) };
    let orig_video_ega_cx: u16 = unsafe { *(0x9000c as *const u16) };

    unsafe {
        let a: u8;
        let mut display_desc: &str = "????";
        let mut display_ptr: usize; // *mut u8
        VIDEO_NUM_COLUMNS = orig_video_cols as usize;
        VIDEO_SIZE_ROW = VIDEO_NUM_COLUMNS * 2;
        VIDEO_NUM_LINES = orig_video_lines;
        VIDEO_PAGE = orig_video_page as u8;
        VIDEO_ERASE_CHAR = 0x0720;

        if orig_video_mode == 7 {     /* Is this a monochrome display? */
            VIDEO_MEM_START = 0xb0000;
            VIDEO_PORT_REG = 0x3b4;
            VIDEO_PORT_VAL = 0x3b5;
            if (orig_video_ega_bx & 0xff) != 0x10 {
                VIDEO_TYPE = VIDEO_TYPE_EGAM;
                VIDEO_MEM_END = 0xb8000;
                display_desc = "EGAm";
            }
            else
            {
                VIDEO_TYPE = VIDEO_TYPE_MDA;
                VIDEO_MEM_END = 0xb2000;
                display_desc = "*MDA";
            }
        } else {
            VIDEO_MEM_START = 0xb8000;
            VIDEO_PORT_REG	= 0x3d4;
            VIDEO_PORT_VAL	= 0x3d5;
            if (orig_video_ega_bx & 0xff) != 0x10 {
                VIDEO_TYPE = VIDEO_TYPE_EGAC;
                VIDEO_MEM_END = 0xbc000;
                display_desc = "EGAc";
            }
            else
            {
                VIDEO_TYPE = VIDEO_TYPE_CGA;
                VIDEO_MEM_END = 0xba000;
                display_desc = "*CGA";
            }
        }

        /* Let the user known what kind of display driver we are using */

        display_ptr = VIDEO_MEM_START + VIDEO_SIZE_ROW - 8;
        for ch in display_desc.chars() {
            *(display_ptr as *mut char) = ch;
            display_ptr += 2;
        }

        /* Initialize the variables used for scrolling (mostly EGA/VGA)	*/

        ORIGIN	= VIDEO_MEM_START;
        SCR_END	= VIDEO_MEM_START + VIDEO_NUM_LINES * VIDEO_SIZE_ROW;
        TOP	= 0;
        BOTTOM	= VIDEO_NUM_LINES;

        gotoxy(orig_x.into(), orig_y.into());
        set_trap_gate(0x21, transmute(&keyboard_interrupt));
        outb_p((inb_p(0x21)&0xfd).into(), 0x21);
        a = inb_p(0x61);
        outb_p((a|0x80).into(), 0x61);
        outb(a.into(), 0x61);
    }
}
/* from bsd-net-2: */

unsafe fn sysbeepstop() {
    /* disable counter 2 */
    outb((inb_p(0x61)&0xFC).into(), 0x61);
}

static mut BEEPCOUNT: usize = 0;

unsafe fn sysbeep() {
    /* enable counter 2 */
    outb_p((inb_p(0x61)|3).into(), 0x61);
    /* set command for counter 2, 2 byte write */
    outb_p(0xB6, 0x43);
    /* send 0x637 for 750 HZ */
    outb_p(0x37, 0x42);
    outb(0x06, 0x42);
    /* 1/8 second */
    BEEPCOUNT = HZ/8;	
}

