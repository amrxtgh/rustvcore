/// Default DRAM size: 1 GiB.
pub const DRAM_SIZE: u64 = 1024 * 1024 * 1024;
pub struct RAM {
    pub data: Vec<u8>,
}
impl RAM {
    pub fn new() -> Self {
        Self {
            data: vec![0; DRAM_SIZE as usize],
        }
    }
    pub fn load32(&self, _addr: u32) {}
    pub fn store32(&self, _addr: u32, _val: u32) {}
}
