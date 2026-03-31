/*
I think we are here to implement only the architectural exceptions/traps defined
in the base RV32I which are the ones that CPU itself can generate & we are ignoring
the traps for advanced privilege systems like:
- page faults
- virtual memory faults
- interrupt controller traps
- machine timer interrupts
- supervisor traps

these appears when implementing (particularly EEI) :
- MMU, virtual memory, Operating System, devices

What are the traps I am implementing here:
Fetch stage:
 - Instruction address misaligned(PC not 4-byte aligned)
 - Instruction Access Fault(failed instruction fetch)
Decode stage:
 - Illegal Instruction(unknown opcode or invalid encodings)
Execute Stage:
 - ECALL (environment call).
 - EBREAK (debugger breakpoint)
Memory stage:
 - Load address misaligned (address not aligned)
 - Load access fault (invalid memory access)
 - Store Address Misaligned(address not aligned)
 - Store Access Fault (invalid memory write)
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Exception {
    // Fetch Stage
    InstructionAddressMisaligned,
    InstructionAccessFault,

    // Decode Stage
    IllegalInstruction,

    // Execute Stage
    Breakpoint,
    EnvironmentCall,

    // Memory Stage
    LoadAddressMisaligned,
    LoadAccessFault,

    StoreAddressMisaligned,
    StoreAccessFault,
}

impl Exception {
    pub fn code(self) -> u32 {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::EnvironmentCall => 11,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault => 7,
        }
    }
}
