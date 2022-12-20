use crate::task::increment_syscall_times;

mod fs;
mod process;

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GETTIMEOFDAY: usize = 169;
pub const SYSCALL_TASK_INFO: usize = 410;

pub fn syscall(syscall_id: usize, args: [usize;3]) -> isize {
    increment_syscall_times(syscall_id);

    match syscall_id {
        SYSCALL_WRITE => fs::sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => process::sys_exit(args[0] as i32),
        SYSCALL_YIELD => process::sys_yield(),
        SYSCALL_GETTIMEOFDAY => process::sys_get_time(args[0] as _, args[1]),
        SYSCALL_TASK_INFO => process::sys_task_info(args[0] as _),
        _ => {
            error!("Unsupported syscall_id: {}", syscall_id);
            process::sys_exit(-1);
        }
    }
}