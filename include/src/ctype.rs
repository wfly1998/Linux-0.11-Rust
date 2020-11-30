const _U: u8 = 0x01;    /* upper */
const _L: u8 = 0x02;    /* lower */
const _D: u8 = 0x04;    /* digit */
const _C: u8 = 0x08;    /* cntrl */
const _P: u8 = 0x10;    /* punct */
const _S: u8 = 0x20;    /* white space (space/lf/tab) */
const _X: u8 = 0x40;    /* hex digit */
const _SP: u8 = 0x80;   /* hard space (0x20) */

const _ctype: [u8; 257] = [0x00,			/* EOF */
_C,_C,_C,_C,_C,_C,_C,_C,			/* 0-7 */
_C,_C|_S,_C|_S,_C|_S,_C|_S,_C|_S,_C,_C,		/* 8-15 */
_C,_C,_C,_C,_C,_C,_C,_C,			/* 16-23 */
_C,_C,_C,_C,_C,_C,_C,_C,			/* 24-31 */
_S|_SP,_P,_P,_P,_P,_P,_P,_P,			/* 32-39 */
_P,_P,_P,_P,_P,_P,_P,_P,			/* 40-47 */
_D,_D,_D,_D,_D,_D,_D,_D,			/* 48-55 */
_D,_D,_P,_P,_P,_P,_P,_P,			/* 56-63 */
_P,_U|_X,_U|_X,_U|_X,_U|_X,_U|_X,_U|_X,_U,	/* 64-71 */
_U,_U,_U,_U,_U,_U,_U,_U,			/* 72-79 */
_U,_U,_U,_U,_U,_U,_U,_U,			/* 80-87 */
_U,_U,_U,_P,_P,_P,_P,_P,			/* 88-95 */
_P,_L|_X,_L|_X,_L|_X,_L|_X,_L|_X,_L|_X,_L,	/* 96-103 */
_L,_L,_L,_L,_L,_L,_L,_L,			/* 104-111 */
_L,_L,_L,_L,_L,_L,_L,_L,			/* 112-119 */
_L,_L,_L,_P,_P,_P,_P,_C,			/* 120-127 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 128-143 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 144-159 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 160-175 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 176-191 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 192-207 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 208-223 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,		/* 224-239 */
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];		/* 240-255 */

#[inline] pub fn isalnum(c: u8) -> bool { (_ctype[c as usize + 1]&(_U|_L|_D)) != 0 }
#[inline] pub fn isalpha(c: u8) -> bool { (_ctype[c as usize + 1]&(_U|_L)) != 0 }
#[inline] pub fn iscntrl(c: u8) -> bool { (_ctype[c as usize + 1]&(_C)) != 0 }
#[inline] pub fn isdigit(c: u8) -> bool { (_ctype[c as usize + 1]&(_D)) != 0 }
#[inline] pub fn isgraph(c: u8) -> bool { (_ctype[c as usize + 1]&(_P|_U|_L|_D)) != 0 }
#[inline] pub fn islower(c: u8) -> bool { (_ctype[c as usize + 1]&(_L)) != 0 }
#[inline] pub fn isprint(c: u8) -> bool { (_ctype[c as usize + 1]&(_P|_U|_L|_D|_SP)) != 0 }
#[inline] pub fn ispunct(c: u8) -> bool { (_ctype[c as usize + 1]&(_P)) != 0 }
#[inline] pub fn isspace(c: u8) -> bool { (_ctype[c as usize + 1]&(_S)) != 0 }
#[inline] pub fn isupper(c: u8) -> bool { (_ctype[c as usize + 1]&(_U)) != 0 }
#[inline] pub fn isxdigit(c: u8) -> bool { (_ctype[c as usize + 1]&(_D|_X)) != 0 }

#[inline] pub fn isascii(c: u8) -> bool { c<=0x7f }
#[inline] pub fn toascii(c: u8) -> u8 { c&0x7f }

#[inline] pub fn tolower(c: u8) -> u8 { if (isupper(c)) { c + ('a' as u8 - 'A' as u8)} else {c} }
#[inline] pub fn toupper(c: u8) -> u8 { if (islower(c)) { c - ('a' as u8 - 'A' as u8)} else {c} }

