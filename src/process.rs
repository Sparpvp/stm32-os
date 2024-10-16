use crate::allocator::memory::zalloc_block;

const STACK_SIZE: u16 = 328;
static mut PID: u16 = 1;

#[repr(C)]
pub struct Context {
    // R0 -> R7
    pub general_regs: [usize; 7],
    curr_sp: usize,
    lr: usize,
    pc: usize,
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
    stack_base: *mut u8, // Descending type; points to the top of the allocated stack
    state: ProcessState,
    pid: u16,
}

impl Context {
    fn new_default(func_addr: usize, stack_base: *mut u8) -> Context {
        let control: usize = 1 << 1; // Use Process Stack Pointer (PSP)
        let xpsr: usize = 1 << 24; // set T-bit (we're running in Thumb)
        let freg: [usize; 3] = [xpsr, 0, control];

        Context {
            general_regs: [0; 7],
            flags_regs: freg,
            curr_sp: stack_base as usize,
            lr: 0xFFFFFFFF, // Reset value
            pc: func_addr,
        }
    }
}

impl Process {
    pub fn new_proc(func: fn()) -> Self {
        let func_addr = func as usize;
        // Since the stack is descending-order, and the allocator gives us the
        // starting address on RAM, we add its size to reference the top
        let stack_base = unsafe { zalloc_block(STACK_SIZE).byte_add(STACK_SIZE as usize) };

        let mut proc = Process {
            ctx: Context::new_default(func_addr, stack_base),
            stack_base: stack_base,
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
