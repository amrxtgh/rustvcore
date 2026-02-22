pub struct CPU {
    pub regs: [u32; 32], //x0-x31
    pub pc: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
        }
    }
    pub fn read_reg(&self, index: usize) -> u32 {
        if index == 0 { 0 } else { self.regs[index] } // ensure rv x0 is always 0
    }
    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 { let _ = self.regs[index] = value; } // ensure writing to x0 not possible
    }

    pub fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let val = self.read_reg(rs1).wrapping_add(self.read_reg(rs2));
        self.write_reg(rd, val);
    }
}
