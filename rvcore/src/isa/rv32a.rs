// A extensions (atomic)
pub fn lr_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize) {}
pub fn sc_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amoswap_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amoadd_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amoxor_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amoand_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amoor_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amomin_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amomax_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amominu_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
pub fn amomaxu_w(_hart: &mut crate::hart::Hart, _rd: usize, _rs1: usize, _rs2: usize) {}
