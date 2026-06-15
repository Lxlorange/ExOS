mod context;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

use alloc::sync::Arc;

use crate::loader::get_app_data_by_name;
use lazy_static::*;
use switch::__switch;

pub use context::TaskContext;
pub use manager::{add_task, fetch_task};
pub use pid::{kernel_stack_position, pid_alloc, KernelStack, PidHandle};
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
};
pub use task::{TaskControlBlock, TaskStatus};

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> =
        Arc::new(TaskControlBlock::new(get_app_data_by_name("initproc").unwrap()));
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn suspend_current_and_run_next() {
    let task = take_current_task().unwrap();

    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);

    add_task(task);
    schedule(task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Zombie;
    task_inner.exit_code = exit_code;
    drop(task_inner);
    schedule(task_cx_ptr);
}
