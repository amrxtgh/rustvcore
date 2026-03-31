use crate::trap::Exception;

/// Default DRAM size: 1 GiB.
const DRAM_SIZE: u64 = 1024 * 1024 * 1024;
pub struct RAM {
    pub data: Vec<u8>,
}
impl RAM {
    pub fn new() -> Self {
        Self {
            data: vec![0; DRAM_SIZE as usize],
        }
    }
    // TODO: i think this methods can be done properly using
    // pattern matching.
    pub fn load8(&self, addr: u32) -> Result<u8, Exception> {
        /* load8 specifically means read 1 byte from the memory */
        if addr as u64 >= DRAM_SIZE {
            return Err(Exception::LoadAccessFault);
        }
        Ok(self.data[addr as usize])
    }
    pub fn load16(&self, addr: u32) -> Result<u16, Exception> {
        /* Alignment check if the address is divisible by 2*/
        if addr % 2 != 0 {
            return Err(Exception::LoadAddressMisaligned);
        }
        if addr as u64 + 1 >= DRAM_SIZE {
            return Err(Exception::LoadAccessFault);
        }

        let index = addr as usize;
        let b0 = self.data[index] as u16;
        let b1 = self.data[index + 1] as u16;

        let value = b0 | (b1 << 8);

        Ok(value)
    }
    pub fn load32(&self, addr: u32) -> Result<u32, Exception> {
        if addr % 4 != 0 {
            return Err(Exception::LoadAddressMisaligned);
        }
        if addr as u64 + 3 >= DRAM_SIZE {
            return Err(Exception::LoadAccessFault);
        }

        let index = addr as usize;
        // reading 4 bytes
        let b0 = self.data[index] as u32;
        let b1 = self.data[index + 1] as u32;
        let b2 = self.data[index + 2] as u32;
        let b3 = self.data[index + 3] as u32;

        // converting the bits into little-endian order
        // combining the bytes into 32-bit word
        let value = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
        Ok(value)
    }
    pub fn store8(&mut self, addr: u32, value: u8) -> Result<(), Exception> {
        if addr as u64 >= DRAM_SIZE {
            return Err(Exception::StoreAccessFault);
        }
        let index = addr as usize;
        self.data[index] = value;
        Ok(())
    }
    pub fn store16(&mut self, addr: u32, value: u16) -> Result<(), Exception> {
        if addr % 2 != 0 {
            return Err(Exception::StoreAddressMisaligned);
        }
        if addr as u64 + 1 >= DRAM_SIZE {
            return Err(Exception::StoreAccessFault);
        }
        let index = addr as usize;
        /* Extract the low byte and high byte */
        let b0 = (value & 0xFF) as u8;
        let b1 = (value >> 8) as u8;
        self.data[index] = b0;
        self.data[index + 1] = b1;
        Ok(())
    }

    pub fn store32(&mut self, addr: u32, value: u32) -> Result<(), Exception> {
        if addr % 4 != 0 {
            return Err(Exception::StoreAddressMisaligned);
        }
        if addr as u64 + 3 >= DRAM_SIZE {
            return Err(Exception::StoreAccessFault);
        }
        let index = addr as usize;

        let b0 = (value & 0xFF) as u8;
        let b1 = ((value >> 8) & 0xFF) as u8;
        let b2 = ((value >> 16) & 0xFF) as u8;
        let b3 = ((value >> 24) & 0xFF) as u8;

        self.data[index] = b0;
        self.data[index + 1] = b1;
        self.data[index + 2] = b2;
        self.data[index + 3] = b3;
        Ok(())
    }
}
