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
    pub fn load32(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        // reading 4 bytes
        let b0 = self.data[addr] as u32;
        let b1 = self.data[addr + 1] as u32;
        let b2 = self.data[addr + 2] as u32;
        let b3 = self.data[addr + 3] as u32;

        // converting the bits into little-endian order
        // combining the bytes into 32-bit word
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }
    pub fn store32(&self, _addr: u32, _val: u32) {}
}
