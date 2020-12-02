use include::signal::*;
use include::asm::system::*;
use include::linux::head::*;
use include::linux::kernel::*;
use include::linux::sched::*;
use kernel::println;

#[inline]
fn oom() {
    println!("out of memory");
    panic!("out of memory");
    // do_exit(SIGSEGV);
}

