#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeMode {
    User = 0,
    Supervisor = 1,
    Machine = 3,
    Debug = 4,
}

impl PrivilegeMode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PrivilegeMode::User,
            1 => PrivilegeMode::Supervisor,
            3 => PrivilegeMode::Machine,
            4 => PrivilegeMode::Debug,
            _ => PrivilegeMode::Machine,
        }
    }
}

pub const XLEN: u32 = 32;

pub struct CPU {
    pub regs: [u32; XLEN as usize],
    pub pc: u32,
    pub mode: PrivilegeMode,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0; XLEN as usize],
            pc: 0,
            mode: PrivilegeMode::Machine,
        }
    }
    pub fn read_reg(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else {
            self.regs[index]
        } // ensure rv x0 is always 0
    }
    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 {
            let _ = self.regs[index] = value;
        } // ensure writing to x0 not possible
    }

    pub fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let val = self.read_reg(rs1).wrapping_add(self.read_reg(rs2));
        self.write_reg(rd, val);
    }
}
