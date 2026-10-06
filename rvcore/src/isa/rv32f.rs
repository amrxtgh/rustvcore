// F extensions (floating point)
pub fn fadd_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fsub_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fmul_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fdiv_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fsqrt_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
pub fn fsgnj_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fmin_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fmax_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fcvt_w_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
pub fn fcvt_s_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
pub fn fmv_x_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
pub fn fmv_w_x(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
pub fn feq_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn flt_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fle_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn fclass_s(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
