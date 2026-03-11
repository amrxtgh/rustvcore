// Zicsr (Control and Status Register Extension)
pub fn csrrw(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _csr: u32) {}
pub fn csrrs(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _csr: u32) {}
pub fn csrrc(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _csr: u32) {}
pub fn csrwi(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: u32, _csr: u32) {}
pub fn csrsi(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: u32, _csr: u32) {}
pub fn csrci(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: u32, _csr: u32) {}
