//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, current_user_token, get_currunt_task_syscall_times, get_currunt_task_run_time, get_currunt_task_task_status, TaskStatus,
    },
    timer::get_time_us,
    mm::{translated_byte_buffer, mmap, unmmap},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    // -1
    let buf = _ts as *const _ as *const u8;
    let len = core::mem::size_of::<TimeVal>();

    let buffers = translated_byte_buffer(current_user_token(), buf, len);
    if buffers.is_empty(){
        return -1;
    }
    
    let us = get_time_us();
    let mut time = TimeVal {
        sec: us / 1_000_000, 
        usec: us % 1_000_000,
    }; 

    let  tt = &mut time;
    let ptr = tt as *const _ as *const u8;
    let mut start: usize = 0;
    
    for buffer in buffers {
        let len = buffer.len();
        // buffer.copy_from_slice(&ptr[start..start + len]);
        unsafe {
            let a =core::slice::from_raw_parts(ptr, len);
            buffer.copy_from_slice(&a[start..start+len]);
        }

        start += len;
    }
    
    return 0;
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    // -1
    let buf = _ti as *const _ as *const u8;
    let len = core::mem::size_of::<TaskInfo>();
    let buffers = translated_byte_buffer(current_user_token(), buf, len);
    if buffers.is_empty(){
        return -1;
    }
  
    let mut _ti = TaskInfo{
        status: get_currunt_task_task_status().unwrap(),
        syscall_times: [0; MAX_SYSCALL_NUM],
        time: 0,
    };
    
    get_currunt_task_syscall_times(&mut _ti.syscall_times);
    if let Some(time) = get_currunt_task_run_time() {
        _ti.time = time;
    }else{
        return -1;
    }

    let  tt = &mut _ti;
    let ptr = tt as *const _ as *const u8;
    let mut start: usize = 0;
    
    for buffer in buffers {
        let len = buffer.len();
        // buffer.copy_from_slice(&ptr[start..start + len]);
        unsafe {
            let a =core::slice::from_raw_parts(ptr, len);
            buffer.copy_from_slice(&a[start..start+len]);
        }

        start += len;
    }
    
    return 0;
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    // -1
    let token = current_user_token();
    
    return mmap(token, _start, _len, _port);
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    // -1
    let token = current_user_token();
 
    //fn unmmap(token: usize, _start: usize, _len: usize)->isize
    return unmmap(token, _start, _len);
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
