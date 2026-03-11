// F extensions (floating point)
pub fn fadd_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fsub_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fmul_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fdiv_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fsqrt_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize) {}
pub fn fsgnj_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fmin_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fmax_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fcvt_w_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize) {}
pub fn fcvt_s_w(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize) {}
pub fn fmv_x_w(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize) {}
pub fn fmv_w_x(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize) {}
pub fn feq_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn flt_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fle_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fclass_s(_cpu: &mut crate::cpu::CPU, _rd: usize, _rs1: usize) {}
