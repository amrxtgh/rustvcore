use crate::cpu::CPU;
// Base RV32I

pub fn add(cpu: &mut CPU, rd: usize, rs1: usize, rs2: usize) {
    let val = cpu.read_reg(rs1).wrapping_add(cpu.read_reg(rs2));
    cpu.write_reg(rd,
        cpu.read_reg(rs1).wrapping_add(cpu.read_reg(rs2))
    );
}
pub fn sub(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}

pub fn addi(cpu: &mut CPU, rd: usize, rs1: usize, imm: i32) {
    cpu.write_reg(rd,
        cpu.read_reg(rs1).wrapping_add(imm as u32)
    );
}

pub fn slt(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
// set less than for unsigned number
pub fn sltu(cpu: &mut CPU, rd: usize, rs1: usize, rs2: usize) {
    cpu.write_reg(rd,
        (cpu.read_reg(rs1) < cpu.read_reg(rs2)) as u32
    );
}
pub fn slti(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn sltiu(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn and(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn or(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn xor(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn andi(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn ori(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn xori(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn sll(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn srl(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn sra(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn slli(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _shamt: u32) {}
pub fn srli(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _shamt: u32) {}
pub fn srai(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _shamt: u32) {}
pub fn lui(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: i32) {}
pub fn auipc(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: i32) {}
pub fn lb(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lbu(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lh(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lhu(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lw(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn sb(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn sh(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn sw(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn beq(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bne(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn blt(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bge(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bltu(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bgeu(_cpu: &mut crate::cpu::CPU, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn jal(_cpu: &mut crate::cpu::CPU, _rd: usize, _imm: i32) {}
pub fn jalr(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn ecall(_cpu: &mut crate::cpu::CPU) {}
pub fn ebreak(_cpu: &mut crate::cpu::CPU) {}
