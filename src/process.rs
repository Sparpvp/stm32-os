use crate::allocator::memory::{self, zalloc_block};

const STACK_SIZE: u16 = 328;
static mut PID: u16 = 1;

#[repr(C)]
pub struct Context {
    // R0 -> R7
    pub general_regs: [usize; 7],
    // R14 -> R15: LR, PC
    pub special_regs: [usize; 2],
    // PSR, PRIMASK, CONTROL
    pub flags_regs: [usize; 3],
}

pub enum ProcessState {
    Running,
    Ready,
    Waiting,
    Terminated,
}

#[repr(C)]
pub struct Process {
    ctx: Context,
    stack: *mut u8, // Descending type
    state: ProcessState,
    pid: u16,
}

impl Context {
    fn new_empty() -> Context {
        let control: usize = 1 << 1;
        let freg: [usize; 3] = [0, 0, control];

        Context {
            general_regs: [0; 7],
            special_regs: [0; 2],
            flags_regs: freg,
        }
    }
}

impl Process {
    pub fn new_proc(func: fn()) -> Self {
        let func_addr = func as usize;

        let mut proc = Process {
            ctx: Context::new_empty(),
            stack: zalloc_block(STACK_SIZE),
            state: ProcessState::Ready,
            pid: unsafe { PID },
        };

        unsafe {
            // Cortex M0 is single CPU, we don't have to deal with atomics.
            PID += 1;
        };

        todo!()
    }
}
