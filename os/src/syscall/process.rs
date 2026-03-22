//! Process management syscalls
use crate::{
    syscall::*,
    task::{exit_current_and_run_next, get_next_task, suspend_current_and_run_next, TASK_MANAGER},
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
    unsafe {
        SYSCALL_EXIT_COUNT += 1;
    }
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    unsafe {
        SYSCALL_YIELD_COUNT += 1;
    }
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    unsafe {
        SYSCALL_GET_TIME_COUNT += 1;
    }
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
    let task_id = get_next_task();
    unsafe {
        SYSCALL_TRACE_COUNT += 1;
    }
    trace!("kernel: sys_trace");
    if _trace_request == 0 {
        // return u8 pointed by _id
        unsafe { *(_id as *const u8) as isize }
    } else if _trace_request == 1 {
        let data = (_data & 0xFF) as u8;
        unsafe {
            // assign data to the u8 pointed by _id
            *(_id as *mut u8) = data;
        }
        0
    } else if _trace_request == 2 {
        // return syscall counts
        syscall_count(_id) as isize
    } else {
        -1
    }
}

fn syscall_count(syscall_id: usize) -> usize {
    unsafe {
        match syscall_id {
            SYSCALL_WRITE => SYSCALL_WRITE_COUNT,
            SYSCALL_EXIT => SYSCALL_EXIT_COUNT,
            SYSCALL_YIELD => SYSCALL_YIELD_COUNT,
            SYSCALL_GET_TIME => SYSCALL_GET_TIME_COUNT,
            SYSCALL_TRACE => SYSCALL_TRACE_COUNT,
            _ => panic!("Unsupported syscall_id: {}", syscall_id),
        }
    }
}
