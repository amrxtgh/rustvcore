/*
I think we are here to implement only the architectural exceptions/traps defined
in the base RV32I which are the ones that CPU itself can generate we are ignoring 
the traps for advanced privilege systems like:
- page faults
- virtual memory faults
- interrupt controller traps
- machine timer interrupts
- supervisor traps 

these appears when implementing :
- MMU
- virtual memory
- operating systems
- devices

What are the traps I am implementing here:
Fetch stage:
 - instruction address misaligned(PC not 4-byte aligned)
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

 