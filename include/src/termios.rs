pub const TTY_BUF_SIZE: usize = 1024;

/* 0x54 is just a magic number to make these relatively uniqe ('T') */

pub const TCGETS:       usize = 0x5401;
pub const TCSETS:       usize = 0x5402;
pub const TCSETSW:      usize = 0x5403;
pub const TCSETSF:      usize = 0x5404;
pub const TCGETA:       usize = 0x5405;
pub const TCSETA:       usize = 0x5406;
pub const TCSETAW:      usize = 0x5407;
pub const TCSETAF:      usize = 0x5408;
pub const TCSBRK:       usize = 0x5409;
pub const TCXONC:       usize = 0x540A;
pub const TCFLSH:       usize = 0x540B;
pub const TIOCEXCL:     usize = 0x540C;
pub const TIOCNXCL:     usize = 0x540D;
pub const TIOCSCTTY:    usize = 0x540E;
pub const TIOCGPGRP:    usize = 0x540F;
pub const TIOCSPGRP:    usize = 0x5410;
pub const TIOCOUTQ:     usize = 0x5411;
pub const TIOCSTI:      usize = 0x5412;
pub const TIOCGWINSZ:   usize = 0x5413;
pub const TIOCSWINSZ:   usize = 0x5414;
pub const TIOCMGET:     usize = 0x5415;
pub const TIOCMBIS:     usize = 0x5416;
pub const TIOCMBIC:     usize = 0x5417;
pub const TIOCMSET:     usize = 0x5418;
pub const TIOCGSOFTCAR: usize = 0x5419;
pub const TIOCSSOFTCAR: usize = 0x541A;
pub const TIOCINQ:      usize = 0x541B;

pub struct winsize {
	pub ws_row: u16,
	pub ws_col: u16,
	pub ws_xpixel: u16,
	pub ws_ypixel: u16,
}

pub const NCC: usize = 8;
pub struct termio {
	pub c_iflag: usize,           /* input mode flags */
	pub c_oflag: usize,		/* output mode flags */
	pub c_cflag: usize,           /* control mode flags */
	pub c_lflag: usize,           /* local mode flags */
	pub c_line: u8,             /* line discipline */
	pub c_cc: [u8; NCC],        /* control characters */
}

pub const NCCS: usize = 17;
pub struct termios {
	pub c_iflag: usize,           /* input mode flags */
	pub c_oflag: usize,           /* output mode flags */
	pub c_cflag: usize,           /* control mode flags */
	pub c_lflag: usize,           /* local mode flags */
	pub c_line: u8,             /* line discipline */
	pub c_cc: [u8; NCCS],       /* control characters */
}

/* c_cc characters */
pub const VINTR:    u8 = 0;
pub const VQUIT:    u8 = 1;
pub const VERASE:   u8 = 2;
pub const VKILL:    u8 = 3;
pub const VEOF:     u8 = 4;
pub const VTIME:    u8 = 5;
pub const VMIN:     u8 = 6;
pub const VSWTC:    u8 = 7;
pub const VSTART:   u8 = 8;
pub const VSTOP:    u8 = 9;
pub const VSUSP:    u8 = 10;
pub const VEOL:     u8 = 11;
pub const VREPRINT: u8 = 12;
pub const VDISCARD: u8 = 13;
pub const VWERASE:  u8 = 14;
pub const VLNEXT:   u8 = 15;
pub const VEOL2:    u8 = 16;

/* c_iflag bits */
pub const IGNBRK:   usize = 0000001;
pub const BRKINT:   usize = 0000002;
pub const IGNPAR:   usize = 0000004;
pub const PARMRK:   usize = 0000010;
pub const INPCK:    usize = 0000020;
pub const ISTRIP:   usize = 0000040;
pub const INLCR:    usize = 0000100;
pub const IGNCR:    usize = 0000200;
pub const ICRNL:    usize = 0000400;
pub const IUCLC:    usize = 0001000;
pub const IXON:     usize = 0002000;
pub const IXANY:    usize = 0004000;
pub const IXOFF:    usize = 0010000;
pub const IMAXBEL:  usize = 0020000;

/* c_oflag bits */
pub const OPOST:    usize = 0000001;
pub const OLCUC:    usize = 0000002;
pub const ONLCR:    usize = 0000004;
pub const OCRNL:    usize = 0000010;
pub const ONOCR:    usize = 0000020;
pub const ONLRET:   usize = 0000040;
pub const OFILL:    usize = 0000100;
pub const OFDEL:    usize = 0000200;
pub const NLDLY:    usize = 0000400;
pub const   NL0:    usize = 0000000;
pub const   NL1:    usize = 0000400;
pub const CRDLY:    usize = 0003000;
pub const   CR0:    usize = 0000000;
pub const   CR1:    usize = 0001000;
pub const   CR2:    usize = 0002000;
pub const   CR3:    usize = 0003000;
pub const TABDLY:   usize = 0014000;
pub const   TAB0:   usize = 0000000;
pub const   TAB1:   usize = 0004000;
pub const   TAB2:   usize = 0010000;
pub const   TAB3:   usize = 0014000;
pub const   XTABS:  usize = 0014000;
pub const BSDLY:    usize = 0020000;
pub const   BS0:    usize = 0000000;
pub const   BS1:    usize = 0020000;
pub const VTDLY:    usize = 0040000;
pub const   VT0:    usize = 0000000;
pub const   VT1:    usize = 0040000;
pub const FFDLY:    usize = 0040000;
pub const   FF0:    usize = 0000000;
pub const   FF1:    usize = 0040000;

/* c_cflag bit meaning */
pub const CBAUD:    usize = 0000017;
pub const  B0:      usize = 0000000;      /* hang up */
pub const  B50:     usize = 0000001;
pub const  B75:     usize = 0000002;
pub const  B110:    usize = 0000003;
pub const  B134:    usize = 0000004;
pub const  B150:    usize = 0000005;
pub const  B200:    usize = 0000006;
pub const  B300:    usize = 0000007;
pub const  B600:    usize = 0000010;
pub const  B1200:   usize = 0000011;
pub const  B1800:   usize = 0000012;
pub const  B2400:   usize = 0000013;
pub const  B4800:   usize = 0000014;
pub const  B9600:   usize = 0000015;
pub const  B19200:  usize = 0000016;
pub const  B38400:  usize = 0000017;
pub const EXTA:     usize = B19200;
pub const EXTB:     usize = B38400;
pub const CSIZE:    usize = 0000060;
pub const   CS5:    usize = 0000000;
pub const   CS6:    usize = 0000020;
pub const   CS7:    usize = 0000040;
pub const   CS8:    usize = 0000060;
pub const CSTOPB:   usize = 0000100;
pub const CREAD:    usize = 0000200;
pub const CPARENB:  usize = 0000400;
pub const CPARODD:  usize = 0001000;
pub const HUPCL:    usize = 0002000;
pub const CLOCAL:   usize = 0004000;
pub const CIBAUD:   usize = 03600000  	/* input baud rate (not used) */;
pub const CRTSCTS:  u64 = 020000000000;     /* flow control */

pub const PARENB:   usize = CPARENB;
pub const PARODD:   usize = CPARODD;

/* c_lflag bits */
pub const ISIG:     usize = 0000001;
pub const ICANON:   usize = 0000002;
pub const XCASE:    usize = 0000004;
pub const ECHO:     usize = 0000010;
pub const ECHOE:    usize = 0000020;
pub const ECHOK:    usize = 0000040;
pub const ECHONL:   usize = 0000100;
pub const NOFLSH:   usize = 0000200;
pub const TOSTOP:   usize = 0000400;
pub const ECHOCTL:  usize = 0001000;
pub const ECHOPRT:  usize = 0002000;
pub const ECHOKE:   usize = 0004000;
pub const FLUSHO:   usize = 0010000;
pub const PENDIN:   usize = 0040000;
pub const IEXTEN:   usize = 0100000;

/* modem lines */
pub const TIOCM_LE:     u16 = 0x001;
pub const TIOCM_DTR:    u16 = 0x002;
pub const TIOCM_RTS:    u16 = 0x004;
pub const TIOCM_ST:     u16 = 0x008;
pub const TIOCM_SR:     u16 = 0x010;
pub const TIOCM_CTS:    u16 = 0x020;
pub const TIOCM_CAR:    u16 = 0x040;
pub const TIOCM_RNG:    u16 = 0x080;
pub const TIOCM_DSR:    u16 = 0x100;
pub const TIOCM_CD:     u16 = TIOCM_CAR;
pub const TIOCM_RI:     u16 = TIOCM_RNG;

/* tcflow() and TCXONC use these */
pub const	TCOOFF: u16 = 	0;
pub const	TCOON:  u16 = 	1;
pub const	TCIOFF: u16 = 	2;
pub const	TCION:  u16 = 	3;

/* tcflush() and TCFLSH use these */
pub const	TCIFLUSH:   u16 = 0;
pub const	TCOFLUSH:   u16 = 1;
pub const	TCIOFLUSH:  u16 = 2;

/* tcsetattr uses these */
pub const	TCSANOW:    u16 = 0;
pub const	TCSADRAIN:  u16 = 1;
pub const	TCSAFLUSH:  u16 = 2;

pub type speed_t = i32;
