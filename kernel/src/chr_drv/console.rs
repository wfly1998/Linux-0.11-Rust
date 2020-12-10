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

static mut video_type: u8 = 0;              /* Type of display being used	*/
#[no_mangle]
static mut video_num_columns: usize = 0;    /* Number of text columns	*/
static mut video_size_row: usize = 0;       /* Bytes per row		*/
static mut video_num_lines: usize = 0;      /* Number of test lines		*/
static mut video_page: u8 = 0;              /* Initial video page		*/
static mut video_mem_start: usize = 0;      /* Start of video RAM		*/
static mut video_mem_end: usize = 0;        /* End of video RAM (sort of)	*/
static mut video_port_reg: u16 = 0;         /* Video register select port	*/
static mut video_port_val: u16 = 0;	    /* Video register value port	*/
static mut video_erase_char: u16 = 0;       /* Char+Attrib to erase with	*/

static mut origin: usize = 0;  /* Used for EGA/VGA fast scroll	*/
static mut scr_end: usize = 0; /* Used for EGA/VGA fast scroll	*/
static mut pos: usize = 0;
static mut x: usize = 0;
static mut y: usize = 0;
static mut top: usize = 0;
static mut bottom: usize = 0;
static mut state: usize = 0;
static mut npar: usize = 0;
static mut par: [usize; NPAR] = [0; NPAR];
static mut ques: bool = false;
static mut attr: u8 = 0x07;

/*
 * this is what the terminal answers to a ESC-Z or csi0c
 * query (= vt100 response).
 */
const RESPONSE: [u8; 7] = [33, '[' as u8, '?' as u8, '1' as u8, ';' as u8, '2' as u8, 'c' as u8];

/* NOTE! gotoxy thinks x==video_num_columns is ok */
#[inline]
fn gotoxy(new_x: usize, new_y: usize) {
    unsafe {
        if new_x > video_num_columns || new_y >= video_num_lines {
            return;
        }
        x=new_x;
        y=new_y;
        pos=origin + y*video_size_row + (x<<1);
    }
}

#[inline]
fn set_origin() {
    unsafe {
        cli();
        outb_p(12, video_port_reg);
        outb_p((0xff&((origin-video_mem_start)>>9)) as u8, video_port_val);
        outb_p(13, video_port_reg);
        outb_p((0xff&((origin-video_mem_start)>>1)) as u8, video_port_val);
        sti();
    }
}

unsafe fn scrup() {
    if video_type == VIDEO_TYPE_EGAC || video_type == VIDEO_TYPE_EGAM {
        if top == 0 && bottom == video_num_lines {
            origin += video_size_row;
            pos += video_size_row;
            scr_end += video_size_row;
            if scr_end > video_mem_end {
                llvm_asm!(r#"cld
                             rep
                             movsl
                             movl video_num_columns,$1
                             rep
                             stosw"#
                             ::"{eax}" (video_erase_char),
                             "{ecx}" ((video_num_lines-1)*video_num_columns>>1),
                             "{edi}" (video_mem_start),
                             "{esi}" (origin)
                         );
                scr_end -= origin-video_mem_start;
                pos -= origin-video_mem_start;
                origin = video_mem_start;
            } else {
                llvm_asm!(r#"cld
                             rep
                             stosw"#
                             ::"{eax}" (video_erase_char),
                             "{ecx}" (video_num_columns),
                             "{edi}" (scr_end-video_size_row)
                         );
            }
            set_origin();
        } else {
            llvm_asm!(r#"cld
                         rep
                         movsl
                         movl video_num_columns,%ecx
                         rep
                         stosw"#
                         ::"{eax}" (video_erase_char),
                         "{ecx}" ((bottom-top-1)*video_num_columns>>1),
                         "{edi}" (origin+video_size_row*top),
                         "{esi}" (origin+video_size_row*(top+1))
                     );
        }
    }
    else		/* Not EGA/VGA */
    {
        llvm_asm!(r#"cld
                     rep
                     movsl
                     movl video_num_columns,%ecx
                     rep
                     stosw"#
                     ::"{eax}" (video_erase_char),
                     "{ecx}" ((bottom-top-1)*video_num_columns>>1),
                     "{edi}" (origin+video_size_row*top),
                     "{esi}" (origin+video_size_row*(top+1))
                 );
    }
}

unsafe fn scrdown() {
    if video_type == VIDEO_TYPE_EGAC || video_type == VIDEO_TYPE_EGAM {
        llvm_asm!(r#"std
                     rep
                     movsl
                     addl $2,%edi
                     movl video_num_columns,%ecx
                     rep
                     stosw"#
                     ::"{eax}" (video_erase_char),
                     "{ecx}" ((bottom-top-1)*video_num_columns>>1),
                     "{edi}" (origin+video_size_row*bottom-4),
                     "{esi}" (origin+video_size_row*(bottom-1)-4)
                 );
    }
    else		/* Not EGA/VGA */
    {
        llvm_asm!(r#"std
                     rep
                     movsl
                     addl $2,%edi
                     movl video_num_columns,%ecx
                     rep
                     stosw"#
                     ::"{eax}" (video_erase_char),
                     "{ecx}" ((bottom-top-1)*video_num_columns>>1),
                     "{edi}" (origin+video_size_row*bottom-4),
                     "{esi}" (origin+video_size_row*(bottom-1)-4)
                 );
    }
}

unsafe fn lf() {
    if y+1<bottom {
        y += 1;
        pos += video_size_row;
        return;
    }
    scrup();
}

unsafe fn ri() {
    if y>top {
        y -= 1;
        pos -= video_size_row;
        return;
    }
    scrdown();
}

unsafe fn cr() {
    pos -= x<<1;
    x = 0;
}

unsafe fn del() {
    if x != 0 {
        pos -= 2;
        x -= 1;
        *(pos as *mut u16) = video_erase_char;
    }
}

unsafe fn csi_J(par_: usize) {
    let count: usize;
    let start: usize;

    match par_ {
        0 => {  /* erase from cursor to end of display */
            count = (scr_end - pos) >> 1;
            start = pos;
        },
        1 => {  /* erase from start to cursor */
            count = (pos - origin) >> 1;
            start = origin;
        },
        2 => {  /* erase whole display */
            count = video_num_columns * video_num_lines;
            start = origin;
        },
        _ => {
            return;
        }
    }
    llvm_asm!(r#"cld
                 rep
                 stosw"#
                 ::"{ecx}" (count),
                 "{edi}" (start),"{ax}" (video_erase_char)
             );
}

unsafe fn csi_K(par_: usize) {
    let count: usize;
    let start: usize;

    match par_ {
        0 => {  /* erase from cursor to end of line */
            if x >= video_num_columns {
                return;
            }
            count = video_num_columns - x;
            start = pos;
        },
        1 => {  /* erase from start of line to cursor */
            start = pos - (x << 1);
            count = if x < video_num_columns { x } else { video_num_columns };
        },
        2 => {  /* erase whole line */
            start = pos - (x << 1);
            count = video_num_columns;
        },
        _ => {
            return;
        }
    }
    llvm_asm!(r#"cld
                 rep
                 stosw"#
                 ::"{ecx}" (count),
                 "{edi}" (start),"{eax}" (video_erase_char)
             );
}

unsafe fn csi_m() {
    for i in 0..=npar {
        match par[i] {
            0 | 27 => attr = 0x07,
            1 | 4 => attr = 0x0f,
            7 => attr = 0x70,
            _ => {},
        }
    }
}

#[inline]
unsafe fn set_cursor() {
    cli();
    outb_p(14, video_port_reg);
    outb_p((0xff&((pos-video_mem_start)>>9)) as u8, video_port_val);
    outb_p(15, video_port_reg);
    outb_p((0xff&((pos-video_mem_start)>>1)) as u8, video_port_val);
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
    let mut i: usize = x;
    let mut tmp: u16;
    let mut old: u16 = video_erase_char;
    let mut p: *mut u16 = pos as *mut u16;
    while i < video_num_columns {
        i += 1;
        tmp = *p;
        *p = old;
        old = tmp;
        p = ((p as *const _ as usize) + 2) as *mut u16;
    }
}

unsafe fn insert_line() {
    let oldtop: usize = top;
    let oldbottom: usize = bottom;
    top = y;
    bottom = video_num_lines;
    scrdown();
    top = oldtop;
    bottom = oldbottom;
}

unsafe fn delete_char() {
    let mut i: usize = x;
    let mut p: *mut u16 = pos as *mut u16;
    if x >= video_num_columns {
        return;
    }
    i += 1;
    while i < video_num_columns {
        let tmp: *mut u16 = ((p as *const _ as usize) + 2) as *mut u16;
        *p = *tmp;
        p = tmp;
        i += 1;
    }
    *p = video_erase_char;
}

unsafe fn delete_line() {
    let oldtop: usize = top;
    let oldbottom: usize = bottom;
    top = y;
    bottom = video_num_lines;
    scrup();
    top = oldtop;
    bottom = oldbottom;
}

unsafe fn csi_at(mut nr: usize) {
    if nr > video_num_columns {
        nr = video_num_columns;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        insert_char();
    }
}

unsafe fn csi_L(mut nr: usize) {
    if nr > video_num_lines {
        nr = video_num_lines;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        insert_line();
    }
}

unsafe fn csi_P(mut nr: usize) {
    if nr > video_num_columns {
        nr = video_num_columns;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        delete_char();
    }
}

unsafe fn csi_M(mut nr: usize) {
    if nr > video_num_lines {
        nr = video_num_lines;
    } else if nr == 0 {
        nr = 1;
    }
    for _ in 0..nr {
        delete_line();
    }
}

static mut saved_x: usize = 0;
static mut saved_y: usize = 0;

unsafe fn save_cur() {
    saved_x = x;
    saved_y = y;
}

unsafe fn restore_cur() {
    gotoxy(saved_x, saved_y);
}

pub fn con_write(tty: &mut tty_struct) {
    unsafe {
        let nr = CHARS(&tty.write_q);
        for _ in 0..nr{
            let mut c: u8 = GETCH(&mut tty.write_q);
            if state == 0 {
                if c>31 && c<127 {
                    if x>=video_num_columns {
                        x -= video_num_columns;
                        pos -= video_size_row;
                        lf();
                    }
                    llvm_asm!("movw %ax, (%edx)"
                              ::"{ah}"(attr), "{al}"(c), "{edx}"(pos));  
                    pos += 2;
                    x += 1;
                } else if c==27 {
                    state=1;
                }
                else if c==10 || c==11 || c==12 {
                    lf();
                } else if c==13 {
                    cr();
                } else if c==ERASE_CHAR(tty) {
                    del();
                } else if c==8 {
                    if x != 0 {
                        x -= 1;
                        pos -= 2;
                    }
                } else if c==9 {
                    c=8 - (x&7) as u8;
                    x += c as usize;
                    pos += (c as usize)<<1;
                    if x>video_num_columns {
                        x -= video_num_columns;
                        pos -= video_size_row;
                        lf();
                    }
                    c = 9;
                } else if c==7 {
                    sysbeep();
                }
            } else if state == 1 {
                state=0;
                if c=='[' as u8 {
                    state=2;
                } else if c=='E' as u8 {
                    gotoxy(0,y+1);
                } else if c=='M' as u8 {
                    ri();
                } else if c=='D' as u8 {
                    lf();
                } else if c=='Z' as u8 {
                    respond(tty);
                } else if x=='7' as usize {
                    save_cur();
                } else if x=='8' as usize {
                    restore_cur();
                }
            } else {
                loop {
                    if state == 2 {
                        for npar_ in 0..NPAR {
                            par[npar_] = 0;
                        }
                        npar = NPAR;
                        state = 3;
                        ques = c == '?' as u8;
                        if ques {
                            break;
                        }
                    } else if state == 3 {
                        if c==';' as u8 && npar<NPAR-1 {
                            npar += 1;
                            break;
                        } else if c>=('0' as u8) && c<=('9' as u8) {
                            par[npar] = 10 * par[npar] + (c - ('0' as u8)) as usize;
                            break;
                        } else {
                            state = 4;
                        }
                    } else if state == 4 {
                        state = 0;
                        match c as char {
                            'G' | '`' => {
                                if par[0] != 0 {
                                    par[0] -= 1;
                                }
                                gotoxy(par[0], y);
                            },
                            'A' => {
                                if par[0] == 0 {
                                    par[0] += 1;
                                }
                                gotoxy(x, y-par[0]);
                            },
                            'B' | 'e' => {
                                if par[0] == 0 {
                                    par[0] += 1;
                                }
                                gotoxy(x, y+par[0]);
                            },
                            'C' | 'a' => {
                                if par[0] == 0 {
                                    par[0] += 1;
                                }
                                gotoxy(x+par[0], y);
                            },
                            'D' => {
                                if par[0] == 0 {
                                    par[0] += 1;
                                }
                                gotoxy(x-par[0], y);
                            },
                            'E' => {
                                if par[0] == 0 {
                                    par[0] += 1;
                                }
                                gotoxy(0, y+par[0]);
                            },
                            'F' => {
                                if par[0] == 0 {
                                    par[0] += 1;
                                }
                                gotoxy(0, y-par[0]);
                            },
                            'd' => {
                                if par[0] != 0 {
                                    par[0] -= 1;
                                }
                                gotoxy(x, par[0]);
                            },
                            'H' | 'f' => {
                                if par[0] != 0 {
                                    par[0] -= 1;
                                }
                                if par[1] != 0 {
                                    par[1] -= 1;
                                }
                                gotoxy(par[1], par[0]);
                            },
                            'J' => {
                                csi_J(par[0]);
                            },
                            'K' => {
                                csi_K(par[0]);
                            },
                            'L' => {
                                csi_L(par[0]);
                            },
                            'M' => {
                                csi_M(par[0]);
                            },
                            'P' => {
                                csi_P(par[0]);
                            },
                            '@' => {
                                csi_at(par[0]);
                            },
                            'm' => {
                                csi_m();
                            },
                            'r' => {
                                if par[0] != 0 {
                                    par[0] -= 1;
                                }
                                if par[1] == 0 {
                                    par[1] = video_num_lines;
                                }
                                if par[0] < par[1] && par[1] <= video_num_lines {
                                    top = par[0];
                                    bottom = par[1];
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
    let ORIG_X: u8 = unsafe { *(0x90000 as *const u8) };
    let ORIG_Y: u8 = unsafe { *(0x90001 as *const u8) };
    let ORIG_VIDEO_PAGE: u16 = unsafe { *(0x90004 as *const u16) };
    let ORIG_VIDEO_MODE: u16 = unsafe { *(0x90006 as *const u16) & 0xff };
    let ORIG_VIDEO_COLS: u16 = unsafe { (*(0x90006 as *const u16) & 0xff00) >> 8 };
    let ORIG_VIDEO_LINES: usize = 25;
    let ORIG_VIDEO_EGA_AX: u16 = unsafe { *(0x90008 as *const u16) };
    let ORIG_VIDEO_EGA_BX: u16 = unsafe { *(0x9000a as *const u16) };
    let ORIG_VIDEO_EGA_CX: u16 = unsafe { *(0x9000c as *const u16) };

    unsafe {
        let a: u8;
        let mut display_desc: &str = "????";
        let mut display_ptr: usize; // *mut u8
        video_num_columns = ORIG_VIDEO_COLS as usize;
        video_size_row = video_num_columns * 2;
        video_num_lines = ORIG_VIDEO_LINES;
        video_page = ORIG_VIDEO_PAGE as u8;
        video_erase_char = 0x0720;

        if ORIG_VIDEO_MODE == 7 {     /* Is this a monochrome display? */
            video_mem_start = 0xb0000;
            video_port_reg = 0x3b4;
            video_port_val = 0x3b5;
            if (ORIG_VIDEO_EGA_BX & 0xff) != 0x10 {
                video_type = VIDEO_TYPE_EGAM;
                video_mem_end = 0xb8000;
                display_desc = "EGAm";
            }
            else
            {
                video_type = VIDEO_TYPE_MDA;
                video_mem_end = 0xb2000;
                display_desc = "*MDA";
            }
        } else {
            video_mem_start = 0xb8000;
            video_port_reg	= 0x3d4;
            video_port_val	= 0x3d5;
            if (ORIG_VIDEO_EGA_BX & 0xff) != 0x10 {
                video_type = VIDEO_TYPE_EGAC;
                video_mem_end = 0xbc000;
                display_desc = "EGAc";
            }
            else
            {
                video_type = VIDEO_TYPE_CGA;
                video_mem_end = 0xba000;
                display_desc = "*CGA";
            }
        }

        /* Let the user known what kind of display driver we are using */

        display_ptr = video_mem_start + video_size_row - 8;
        for ch in display_desc.chars() {
            *(display_ptr as *mut char) = ch;
            display_ptr += 2;
        }

        /* Initialize the variables used for scrolling (mostly EGA/VGA)	*/

        origin	= video_mem_start;
        scr_end	= video_mem_start + video_num_lines * video_size_row;
        top	= 0;
        bottom	= video_num_lines;

        gotoxy(ORIG_X.into(), ORIG_Y.into());
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

static mut beepcount: usize = 0;

unsafe fn sysbeep() {
    /* enable counter 2 */
    outb_p((inb_p(0x61)|3).into(), 0x61);
    /* set command for counter 2, 2 byte write */
    outb_p(0xB6, 0x43);
    /* send 0x637 for 750 HZ */
    outb_p(0x37, 0x42);
    outb(0x06, 0x42);
    /* 1/8 second */
    beepcount = HZ/8;	
}

