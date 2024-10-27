use core::{
    mem::size_of,
    ptr::{self, null_mut},
};

use crate::{
    allocator::memory::{free, zalloc_block, zalloc_stack},
    peripherals::core::SysTick,
    scheduler::{ScheduleList, Scheduler, CURR_PROC, PROC_LIST},
};

const STACK_SIZE: u16 = 328;
const IDLE_BUFFER: u16 = 128;
static mut NEW_PID: u16 = 1;

#[repr(C)]
pub struct Context {
    // R0 -> R7
    pub general_regs: [usize; 8],
    pub curr_sp: usize,
    pub lr: usize,
    pub pc: usize,
    // PSR, PRIMASK, CONTROL
    pub flags_regs: [usize; 3],
}

#[repr(C)]
pub enum ProcessState {
    Running,
    Ready,
    Waiting,
    Terminated,
}

#[repr(C)]
pub struct Process {
    pub ctx: Context,
    pub stack_base: *mut u8, // Descending type; points to the top of the allocated stack
    pub state: ProcessState,
    pub pid: u16,
}
pub struct ProcessSpawner {
    idle_task_stack: *mut u8,
}

impl Context {
    fn new_default(func_addr: usize, stack_base: *mut u8) -> Context {
        let control: usize = 1 << 1; // Use Process Stack Pointer (PSP)
        let xpsr: usize = 1 << 24; // set T-bit (we're running in Thumb)
        let freg: [usize; 3] = [xpsr, 0, control];

        Context {
            general_regs: [0; 8],
            flags_regs: freg,
            curr_sp: stack_base as usize,
            lr: 0xFFFFFFFF, // Reset value
            pc: func_addr,
        }
    }
}

impl ProcessSpawner {
    pub fn new(self, func: fn()) -> Self {
        Process::new(func).enqueue();
        self
    }

    // Also here the inline is mandatory
    #[inline(always)]
    pub fn spawn(self) {
        unsafe {
            // Initialize the current process with the first one
            // when we're terminating the builder
            CURR_PROC.write(ptr::read(PROC_LIST.0));
        }

        Scheduler::init(self.idle_task_stack);
        SysTick::enable();
    }
}

impl Process {
    pub fn spawner() -> ProcessSpawner {
        ProcessSpawner {
            idle_task_stack: unsafe { zalloc_stack(IDLE_BUFFER).byte_add(IDLE_BUFFER as usize) },
        }
    }

    pub fn new(func: fn()) -> Self {
        let func_addr = func as usize;
        // Since the stack is descending-order, and the allocator gives us the
        // starting address on RAM, we add its size to reference the top
        let stack_base = unsafe { zalloc_stack(STACK_SIZE).byte_add(STACK_SIZE as usize) };

        let proc = Process {
            ctx: Context::new_default(func_addr, stack_base),
            stack_base: stack_base,
            state: ProcessState::Ready,
            pid: unsafe { NEW_PID },
        };

        unsafe {
            // Cortex M0 is single CPU, we don't have to deal with atomics.
            NEW_PID += 1;
        };

        proc
    }

    pub fn enqueue(self) {
        let new_schedule =
            unsafe { &mut *(zalloc_block(size_of::<ScheduleList>() as u16) as *mut ScheduleList) };
        new_schedule.proc.write(self);
        new_schedule.next = null_mut();

        if unsafe { PROC_LIST.0 } == null_mut() {
            unsafe {
                PROC_LIST = Scheduler(new_schedule);
            }
        } else {
            let mut head = unsafe { &mut *(PROC_LIST.0) };
            while head.next != null_mut() {
                head = unsafe { &mut *(head.next) };
            }
            head.next = new_schedule;
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let bottom_stack: usize = self.stack_base as usize - STACK_SIZE as usize;
        free(bottom_stack as *mut u8);
    }
}
