#[derive(Debug, Copy, Clone)]
pub enum SyscallTable {
    ECALL = 0,
    EXIT = 1,
    KILL = 2,
    SLEEP = 3,
    READ = 4,
    PRINT = 5,
}
