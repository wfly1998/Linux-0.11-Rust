use include::asm::segment::*;
use include::linux::sched::*;
// use include::linux::kernel::*;

#[no_mangle]
fn math_error() {
    /*
    llvm_asm!("fnclex");
    if (last_task_used_math) {
        last_task_used_math->signal |= 1<<(SIGFPE-1);
    }
    */
}

