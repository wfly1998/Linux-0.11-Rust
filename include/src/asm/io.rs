macro_rules! outb {
    ($($value: tt)+, $($port: tt)+) => {
        llvm_asm!("outb %%al,%%dx"::"a" (value),"d" (port))
    }
}

macro_rules! inb {
    ($($port: tt)+) => {
        let _v: u8;
        llvm_asm!("inb %%dx,%%al":"=a" (_v):"d" (port));
        _v
    }

}

macro_rules! outb_p {
    ($($value: tt)+, $($port: tt)+) => {
        llvm_asm!("outb %%al,%%dx\n"
		  "\tjmp 1f\n"
		  "1:\tjmp 1f\n"
		  "1:"::"a" (value),"d" (port))
    }
}

macro_rules! inb {
    ($($port: tt)+) => {
        let _v: u8;
        llvm_asm!("inb %%dx,%%al\n"
                  "\tjmp 1f\n"
	          "1:\tjmp 1f\n"
	          "1:":"=a" (_v):"d" (port));
        _v
    }

}

