use core::arch::asm;

pub fn cpu_id() -> usize {
    let mut cpuid: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) cpuid);
    }
    cpuid
}