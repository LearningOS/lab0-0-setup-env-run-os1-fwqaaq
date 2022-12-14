use crate::config::MAX_SYSCALL_NUM;
use crate::task::{self, TaskStatus};
use crate::timer;


pub fn sys_exit(exit_code: i32) -> !{
    info!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next();
    unreachable!();
}

/// app -> OS, syscall_id = 124
/// 将总是返回 0
pub fn sys_yield() -> isize {
    task::suspend_current_and_run_next();
    0
}

#[repr(C)]
pub struct TimeVal{
    pub sec: usize,
    pub usec: usize,
}

/// 忽略 _tz, syscall_id = 169
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    unsafe {
        (*ts).usec = timer::get_time_ms();
    }
    0
}

pub struct Taskinfo {
    status: TaskStatus,
    syscall_times: [u32;MAX_SYSCALL_NUM],
    time: usize,
}

/// 查询任务 syscall_id = 410
pub fn sys_task_info(ti: *mut Taskinfo) -> isize {
    unsafe{
        (*ti).status = TaskStatus::Running;
        task::set_syscall_times(&mut (*ti).syscall_times);
        (*ti).time = timer::get_time_ms() - task::get_start_time();
    }
    0
}