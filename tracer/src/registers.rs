#[derive(Debug, Default, Clone)]
pub struct Registers {
	pub r15: u64,
	pub r14: u64,
	pub r13: u64,
	pub r12: u64,
	pub rbp: u64,
	pub rbx: u64,
	pub r11: u64,
	pub r10: u64,
	pub r9: u64,
	pub r8: u64,
	pub rax: u64,
	pub rcx: u64,
	pub rdx: u64,
	pub rsi: u64,
	pub rdi: u64,
	pub orig_rax: u64,
	pub rip: u64,
	pub cs: u64,
	pub eflags: u64,
	pub rsp: u64,
	pub ss: u64,
	pub fs_base: u64,
	pub gs_base: u64,
	pub ds: u64,
	pub es: u64,
	pub fs: u64,
	pub gs: u64,
}
