use super::table::SyscallTable;

pub extern "C" fn ecall(syscall: SyscallTable, arg: u32) {
    unsafe {
        asm!("mv t0, $0" :: "r"(syscall) :: "volatile");
        asm!("mv t1, $0" :: "r"(arg) :: "volatile");
        asm!("ecall" :::: "volatile");
    }
}
