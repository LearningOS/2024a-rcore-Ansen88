//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
// use alloc::collections::btree_map::BTreeMap;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
    // ready_queue: BTreeMap<usize,Arc<TaskControlBlock>>
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        // println!("[kernel]: anlj: new");
        Self {
            ready_queue: VecDeque::new(),
            // ready_queue: BTreeMap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        // println!("[kernel]: anlj: add");
        self.ready_queue.push_back(task);
        // let key = task.add_stride();
        // let key = 0;
        
        // self.ready_queue.insert(key, task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // println!("[kernel]: anlj: fetch");
        self.ready_queue.pop_front()
        
        // if let Some((_, task)) =  self.ready_queue.pop_first(){
        //     // println!("[kernel]: anlj: task");
        //     Some(task)
        // }else{
        //     // println!("[kernel]: anlj: no task");
        //     None
        // }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
