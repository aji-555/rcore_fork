#![no_main]
#![no_std]

mod arch;
mod console;
mod lang_items;
mod logger;
mod sbi;

use core::arch::global_asm;

use log::info;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();
    check_segment();

    info!("hello world!");
    panic!("Shut down!")
}

fn check_segment() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        // fn boot_stack();
        // fn boot_stack_top();
    }
    info!(
        "{:<7} [{:#018x}, {:#018x})",
        ".text", stext as usize, etext as usize
    );
    info!(
        "{:<7} [{:#018x}, {:#018x})",
        ".rodata", srodata as usize, erodata as usize
    );
    info!(
        "{:<7} [{:#018x}, {:#018x})",
        ".data", sdata as usize, edata as usize
    );
    info!(
        "{:<7} [{:#018x}, {:#018x})",
        ".bss", sbss as usize, ebss as usize
    );
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0);
    });
}
