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
    // General-purpose integer registers (x0..x31).
    pub regs: [u32; XLEN as usize],
    // Program counter (byte address of current instruction).
    pub pc: u32,
    // Current privilege mode.
    pub mode: PrivilegeMode,
}

impl CPU {
    // Creating a CPU in machine mode with zeroed state.
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

    // Write register. Writes to x0 are ignored by ISA definition.
    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 {
            let _ = self.regs[index] = value;
        }
    }

    // RV32I integer arithmetic (R- and I-format).
    pub fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn addi(&mut self, rd: usize, rs1: usize, imm: i32) {}

    // RV32I compare/set instructions.
    pub fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn slti(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn sltiu(&mut self, rd: usize, rs1: usize, imm: i32) {}

    // RV32I logical operations.
    pub fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn andi(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn ori(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn xori(&mut self, rd: usize, rs1: usize, imm: i32) {}

    // RV32I shifts (register and immediate forms).
    pub fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn slli(&mut self, rd: usize, rs1: usize, shamt: u32) {}
    pub fn srli(&mut self, rd: usize, rs1: usize, shamt: u32) {}
    pub fn srai(&mut self, rd: usize, rs1: usize, shamt: u32) {}

    // RV32/64 load instructions (I-format).
    // Note: LWU/LD are RV64-oriented placeholders in this RV32-sized CPU state.
    pub fn lb(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn lbu(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn lh(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn lhu(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn lw(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn lwu(&mut self, rd: usize, rs1: usize, imm: i32) {}
    pub fn ld(&mut self, rd: usize, rs1: usize, imm: i32) {}

    // RV32/64 store instructions (S-format).
    // Note: SD is an RV64-oriented placeholder.
    pub fn sb(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn sh(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn sw(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn sd(&mut self, rs1: usize, rs2: usize, imm: i32) {}

    // RV32I branch instructions (B-format).
    pub fn beq(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn bne(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn blt(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn bge(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn bltu(&mut self, rs1: usize, rs2: usize, imm: i32) {}
    pub fn bgeu(&mut self, rs1: usize, rs2: usize, imm: i32) {}

    // RV32I control-flow jumps (J- and I-format).
    pub fn jal(&mut self, rd: usize, imm: i32) {}
    pub fn jalr(&mut self, rd: usize, rs1: usize, imm: i32) {}

    // RV32I/RV32Zicsr system instructions.
    pub fn ecall(&mut self) {}
    pub fn ebreak(&mut self) {}

    // Zicsr control and status register operations.
    pub fn csrrw(&mut self, rd: usize, rs1: usize, csr: u32) {}
    pub fn csrrs(&mut self, rd: usize, rs1: usize, csr: u32) {}
    pub fn csrrc(&mut self, rd: usize, rs1: usize, csr: u32) {}
    pub fn csrwi(&mut self, rd: usize, imm: u32, csr: u32) {}
    pub fn csrsi(&mut self, rd: usize, imm: u32, csr: u32) {}
    pub fn csrci(&mut self, rd: usize, imm: u32, csr: u32) {}

    // M extension (integer multiply/divide).
    pub fn mul(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn mulh(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn mulhsu(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn mulhu(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn div(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn divu(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn rem(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn remu(&mut self, rd: usize, rs1: usize, rs2: usize) {}

    // A extension (atomic memory operations).
    // Note: *_d variants are RV64-oriented placeholders.
    pub fn lr_w(&mut self, rd: usize, rs1: usize) {}
    pub fn lr_d(&mut self, rd: usize, rs1: usize) {}
    pub fn sc_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn sc_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoswap_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoswap_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoadd_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoadd_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoxor_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoxor_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoand_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoand_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoor_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amoor_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amomin_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amomin_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amomax_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amomax_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amominu_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amominu_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amomaxu_w(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn amomaxu_d(&mut self, rd: usize, rs1: usize, rs2: usize) {}

    // F extension (single-precision floating-point).
    pub fn fadd_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fsub_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fmul_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fdiv_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fsqrt_s(&mut self, rd: usize, rs1: usize) {}
    pub fn fsgnj_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fsgnjn_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fsgnjx_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fmin_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fmax_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fcvt_w_s(&mut self, rd: usize, rs1: usize) {}
    pub fn fcvt_s_w(&mut self, rd: usize, rs1: usize) {}
    pub fn fmv_x_w(&mut self, rd: usize, rs1: usize) {}
    pub fn fmv_w_x(&mut self, rd: usize, rs1: usize) {}
    pub fn feq_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn flt_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fle_s(&mut self, rd: usize, rs1: usize, rs2: usize) {}
    pub fn fclass_s(&mut self, rd: usize, rs1: usize) {}
}
