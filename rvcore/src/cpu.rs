use crate::isa::rv32i;

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

/*
Single hart: represents execution context
Each hart has it's own PC, registers and can run fetch-decode-execute loop
*/
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

    // Read register. x0 is hardwired to zero by ISA definition.
    pub fn read_reg(&self, index: usize) -> u32 {
        if index == 0 {
            0
        } else {
            self.regs[index]
        }
    }

    // Write register. Writes to x0 are ignored by ISA definition:.
    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 {
            self.regs[index] = value;
        }
    }

    pub fn exec_op_imm(&mut self, instruction: u32) {
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;

        let imm = (instruction as i32) >> 20;

        match funct3 {
            0x0 => rv32i::addi(self, rd, rs1, imm),
            2 => rv32i::slti(self, rd, rs1, imm),
            3 => rv32i::sltiu(self, rd, rs1, imm),
            4 => rv32i::xori(self, rd, rs1, imm),
            6 => rv32i::ori(self, rd, rs1, imm),
            7 => rv32i::andi(self, rd, rs1, imm),
            1 => rv32i::slli(self, rd, rs1, (imm & 0x1F) as u32),
            5 => {
                if (imm & 0x20) != 0 {
                    rv32i::srai(self, rd, rs1, (imm & 0x1F) as u32)
                } else {
                    rv32i::srli(self, rd, rs1, (imm & 0x1F) as u32)
                }
            }
            _ => {}
        }
    }
    pub fn exec_op(&mut self, instruction: u32) {
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let funct3 = (instruction >> 12) & 0x7;
        let funct7 = instruction >> 25;

        match (funct7, funct3) {
            (0, 0) => rv32i::add(self, rd, rs1, rs2),
            (0, 1) => rv32i::sll(self, rd, rs1, rs2),
            (0, 2) => rv32i::slt(self, rd, rs1, rs2),
            (0, 3) => rv32i::sltu(self, rd, rs1, rs2),
            (0, 4) => rv32i::xor(self, rd, rs1, rs2),
            (0, 5) => rv32i::srl(self, rd, rs1, rs2),
            (0, 6) => rv32i::or(self, rd, rs1, rs2),
            (0, 7) => rv32i::and(self, rd, rs1, rs2),
            (0x20, 0) => rv32i::sub(self, rd, rs1, rs2),
            (0x20, 5) => rv32i::sra(self, rd, rs1, rs2),
            _ => {}
        }
    }

    pub fn exec_load(&mut self, instruction: u32) {
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let imm = (instruction >> 20) as i32;
        let funct3 = (instruction >> 12) & 0x7;

        match funct3 {
            0 => rv32i::lb(self, rd, rs1, imm),
            1 => rv32i::lh(self, rd, rs1, imm),
            2 => rv32i::lw(self, rd, rs1, imm),
            4 => rv32i::lbu(self, rd, rs1, imm),
            5 => rv32i::lhu(self, rd, rs1, imm),
            _ => {}
        }
    }

    pub fn exec_store(&mut self, instruction: u32) {
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let imm = (((instruction >> 25) as i32) << 5) | ((instruction >> 7) & 0x1F) as i32;
        let funct3 = (instruction >> 12) & 0x7;

        match funct3 {
            0 => rv32i::sb(self, rs1, rs2, imm),
            1 => rv32i::sh(self, rs1, rs2, imm),
            2 => rv32i::sw(self, rs1, rs2, imm),
            _ => {}
        }
    }
}
