#[repr(C)]
pub struct desc_struct {
    a: usize,
    b: usize,
}

pub type desc_table = [desc_struct; 256];

extern "C" {
    pub static mut pg_dir: [usize; 1024];
    pub static mut idt: desc_table;
    pub static mut gdt: desc_table;
}

pub const GDT_NUL: usize = 0;
pub const GDT_CODE: usize = 1;
pub const GDT_DATA: usize = 2;
pub const GDT_TMP: usize = 3;

pub const LDT_NUL: usize = 0;
pub const LDT_CODE: usize = 1;
pub const LDT_DATA: usize = 2;

