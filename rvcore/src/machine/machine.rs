use crate::hart::Hart;
use crate::memory::RAM;
use crate::trap::Exception;

pub struct Machine {
    pub hart: Hart,
    pub ram: RAM,
}
impl Machine {
    pub fn new() -> Self {
        Self {
            hart: Hart::new(),
            ram: RAM::new(),
        }
    }
    pub fn step(&mut self) -> Result<(), Exception> {
        // fetch
        let instruction = self.ram.load32(self.hart.pc)?;
        let opcode = instruction & 0x7f; // (bit masking leaving only the opcode)
        
        /*
        opcode indentifies the instruction class, not the exact instruction
        */
        match opcode {
            0x13 => self.hart.exec_op_imm(instruction), // I-type (register+immediate)
            0x33 => self.hart.exec_op(instruction),     // R-type (register-register)
            0x03 => self.hart.exec_load(instruction),   // Load
            0x23 => self.hart.exec_store(instruction),  // Store
            _ => panic!("Unknown opcode {:x}", opcode),
        }
        self.hart.pc += 4;
        Ok(())
    }
    pub fn run(&mut self) {
        loop {
            match self.step() {
                Ok(_) => {}
                Err(_) => {
                    println!("Trap encountered!");
                    break;
                }
            }
        }
    }
}
