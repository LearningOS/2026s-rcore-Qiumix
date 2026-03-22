//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// save syscall counter
    pub syscall_counter: SyscallCounter,
}

#[derive(Copy, Clone)]
pub struct SyscallCounter {
    pub syscall_write_count: usize,
    pub syscall_exit_count: usize,
    pub syscall_yield_count: usize,
    pub syscall_get_time_count: usize,
    pub syscall_trace_count: usize,
}
impl SyscallCounter {
    pub fn new() -> Self {
        SyscallCounter {
            syscall_write_count: 0,
            syscall_exit_count: 0,
            syscall_yield_count: 0,
            syscall_get_time_count: 0,
            syscall_trace_count: 0,
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
