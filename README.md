# Sandbox-grading-system
A syscall-level sandbox for grading system. This is the repo for CS230 operating system project.

# Safe-box
This is the main part of the safebox, source code and reference permission configure file is in /safe-box/src

## Usage
- Put the program to be run into an LXD container
- Put the sandbox and configuration file into the container
''' 
lxc file push safe-box myubuntu/{directory path in container}
lxc file push ip_permission.conf myubuntu/{directory path in container}
lxc file push file_permission.conf myubuntu/{directory path in container}
'''
- Execute the program in sandbox
'''
lxc exec myubuntu -- /bin/bash
./safe-box {-aa -ip} {Program name} {Program parameter}
'''

## Notice (important)
Because the library relaying on has not been published on Rust Cargo, user need to change the Cargo.toml in /safe-box/src manually. We hope that we can publish that on Rust Cargo as soon as possible.

For more detailed usgae, please see the usage in detail, please run safebox with --help

# Tracer
A library for trace another process. It is based of ptrace from C++, and it provides following methods for struct Tracee 
- new(args: &Vec<String>, all: bool) -> Result<Tracee,&'static str>
- take_pid(&self) -> pid_t
- from(pid: i32, all: bool) -> Result<Tracee, String>
- trace_me(&self) -> Result<i64,i64>
- attach(&self) -> Result<i64,String>
- detach(&self) -> Result<i64,i64>
- do_continue(&self)
- take_reg(&self, reg: u64) -> Result<u64, &'static str>
- take_regs(&self) -> Result<Registers, &'static str >
- set_reg(&self, reg: u64, val: u64)
- peek_data(&self, addr: u64)
- read_string(&self, mut addr: u64)
- deny(&self)
- kill(&self)
- base_request(&self, option: Request, addr: *mut libc::c_void, data: *mut libc::c_void) -> Result<i64, i64>
- is_entry(&self) -> bool
- wait_syscall(&mut self) -> Result<bool,String>

For detailed description of the above method, please
- check source code at /tracer/src/lib.rs
- Or, alternatively, you can check ptrace from C++

# Contact
If you have any question, feel free to contact:
- Yuyang Rong
    + email: rongyy@shanghaitech.edu.cn
- Jianxiong Cai
    + email: caijx@shanghaitech.edu.cn
