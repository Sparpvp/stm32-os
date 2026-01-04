# adapted from rust-embedded cortex-m-quickstart
# https://github.com/rust-embedded/cortex-m-quickstart/blob/ac02415275d0190a1a7aa730ec2b0bdf7c3ef88f/openocd.gdb
# 2024-10-02

target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

break _start
# break _context_switch

load

# start the process but immediately halt the processor
stepi
