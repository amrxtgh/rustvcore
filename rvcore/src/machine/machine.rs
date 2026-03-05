use crate::cpu::core::CPU;
use crate::memory::ram::RAM;

pub struct Machine {
    pub cpu: CPU,
    pub ram: RAM,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            ram: RAM::new(),
        }
    }
}
