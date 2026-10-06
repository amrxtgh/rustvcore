use crate::hart::Hart;
// Base RV32I

pub fn add(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    hart.write_reg(rd, hart.read_reg(rs1).wrapping_add(hart.read_reg(rs2)));
}

// set less than for unsigned number
pub fn sltu(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    hart.write_reg(rd, (hart.read_reg(rs1) < hart.read_reg(rs2)) as u32);
}

pub fn and(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    hart.write_reg(rd, hart.read_reg(rs1) & hart.read_reg(rs2));
}
pub fn or(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    hart.write_reg(rd, hart.read_reg(rs1) | hart.read_reg(rs2));
}
pub fn xor(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    hart.write_reg(rd, hart.read_reg(rs1) ^ hart.read_reg(rs2));
}
pub fn sub(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    hart.write_reg(rd, hart.read_reg(rs1).wrapping_sub(hart.read_reg(rs2)));
}
pub fn sll(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    //let a = hart.read_reg(rs1);
    //let shamt = hart.read_reg(rs2) & 0x1F;
    hart.write_reg(rd, hart.read_reg(rs1) << hart.read_reg(rs2) & 0x1F);
}
pub fn srl(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    let a = hart.read_reg(rs1);
    let shamt = hart.read_reg(rs2) & 0x1F;
    hart.write_reg(rd, a >> shamt);
}
pub fn sra(hart: &mut Hart, rd: usize, rs1: usize, rs2: usize) {
    let a = hart.read_reg(rs1) as i32;
    let shamt = hart.read_reg(rs2) & 0x1F;
    hart.write_reg(rd, (a >> shamt) as u32);
}

pub fn addi(hart: &mut Hart, rd: usize, rs1: usize, imm: i32) {
    // hart.write_reg(rd, hart.read_reg(rs1).wrapping_add(imm as u32));
}

pub fn slt(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn slti(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn sltiu(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn andi(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn ori(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn xori(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn slli(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _shamt: u32) {}
pub fn srli(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _shamt: u32) {}
pub fn srai(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _shamt: u32) {}
pub fn lui(_hart: &mut crate::hart::Hart, _rd: usize, _imm: i32) {}
pub fn auipc(_hart: &mut crate::hart::Hart, _rd: usize, _imm: i32) {}
pub fn lb(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lbu(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lh(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lhu(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn lw(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn sb(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn sh(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn sw(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn beq(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bne(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn blt(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bge(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bltu(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn bgeu(_hart: &mut crate::hart::Hart, _rs1: usize, _rs2: usize, _imm: i32) {}
pub fn jal(_hart: &mut crate::hart::Hart, _rd: usize, _imm: i32) {}
pub fn jalr(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _imm: i32) {}
pub fn ecall(_hart: &mut crate::hart::Hart) {}
pub fn ebreak(_hart: &mut crate::hart::Hart) {}
