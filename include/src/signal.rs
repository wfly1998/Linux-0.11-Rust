pub type sig_atomic_t = i32;
pub type sigset_t = u32;

pub const _NSIG: i32 = 32;
pub const NSIG: i32 = _NSIG;

pub const SIGHUP:       i32 = 1;
pub const SIGINT:       i32 = 2;
pub const SIGQUIT:      i32 = 3;
pub const SIGILL:       i32 = 4;
pub const SIGTRAP:      i32 = 5;
pub const SIGABRT:      i32 = 6;
pub const SIGIOT:       i32 = 6;
pub const SIGUNUSED:    i32 = 7;
pub const SIGFPE:       i32 = 8;
pub const SIGKILL:      i32 = 9;
pub const SIGUSR1:      i32 = 10;
pub const SIGSEGV:      i32 = 11;
pub const SIGUSR2:      i32 = 12;
pub const SIGPIPE:      i32 = 13;
pub const SIGALRM:      i32 = 14;
pub const SIGTERM:      i32 = 15;
pub const SIGSTKFLT:    i32 = 16;
pub const SIGCHLD:      i32 = 17;
pub const SIGCONT:      i32 = 18;
pub const SIGSTOP:      i32 = 19;
pub const SIGTSTP:      i32 = 20;
pub const SIGTTIN:      i32 = 21;
pub const SIGTTOU:      i32 = 22;

pub const SA_NOCLDSTOP: i32 = 1;
pub const SA_NOMASK:    u32 = 0x40000000;
pub const SA_ONESHOT:   u32 = 0x80000000;

pub const SIG_BLOCK:    i32 = 0;    /* for blocking signals */
pub const SIG_UNBLOCK:  i32 = 1;    /* for unblocking signals */
pub const SIG_SETMASK:  i32 = 2;    /* for setting the signal mask */

pub struct sigaction {
    sa_handler: fn(i32),
    sa_mask: sigset_t,
    sa_flags: i32,
    sa_restorer: fn(),
}

