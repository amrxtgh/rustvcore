pub struct RAM {
    pub data: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }
    pub fn load32(&self, addr: usize) {

    }

    pub fn store32(&self, addr: usize, val: u32) {
    }
}



