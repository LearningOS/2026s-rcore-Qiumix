//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    // From xv6 lab: syscall
    // uint64 sys_trace(void) {
    //   int mask;
    //   if (argint(0, &mask) < 0)
    //     return -1;
    //   myproc()->mask = mask;
    //   return 0;
    // }
    //
    // pub struct TaskContext {
    //     /// Ret position after task switching
    //     ra: usize,
    //     /// Stack pointer
    //     sp: usize,
    //     /// s0-11 register, callee saved
    //     s: [usize; 12],
    // }
    trace!("kernel: sys_trace");
    if _trace_request == 0 {
        unsafe { *(_id as *const u8) as isize }
    } else if _trace_request == 1 {
        let data = (_data & 0xFF) as u8;
        unsafe {
            *(_id as *mut u8) = data;
        }
        0
    } else if _trace_request == 2 {
        -1
    } else {
        -1
    }
}
