use crate::trap::Exception;

// to boot xv6 just this is needed 0x300-0x3FF, 0x700-0x7FF, 0xB00-0xBFF
#[derive(Debug, Default)]
pub struct Csr {
    pub mstatus: u32,
    pub mtvec: u32,
    pub mepc: u32,
    pub mcause: u32,
    pub mtval: u32,
}

impl Csr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read(&self, addr: u16) -> Result<u32, CsrError> {
        todo!()
    }
    pub fn write(&mut self, addr: u16, value: u32) -> Result<(), CsrError> {
        todo!()
    }
}
// Zicsr (Control and Status Register Extension)
pub fn csrrw(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _csr: u32) {}
pub fn csrrs(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _csr: u32) {}
pub fn csrrc(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _csr: u32) {}
pub fn csrwi(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: u32, _csr: u32) {}
pub fn csrsi(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: u32, _csr: u32) {}
pub fn csrci(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: u32, _csr: u32) {}
