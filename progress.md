# rustvcore — Progress Tracker

> **Rule:** Update this file daily. Answer the two questions at the top every session.

---

## Today's Reflection

**Which leverage point did I touch today?**

**What new feedback loop appeared?**

---

## Project Status: Phase 1 — Core Correctness (In Progress)

### Implemented
- [x] Cargo workspace (`rvcore` + `rvcli`)
- [x] CPU state: `regs: [u32; 32]`, `pc: u32`, `mode: PrivilegeMode`
- [x] x0 hardwired-to-zero invariant (read + write)
- [x] RAM: 1 GiB byte-addressable with load8/16/32, store8/16/32
- [x] Little-endian memory helpers with alignment checks
- [x] Exception enum with correct RISC-V cause codes
- [x] Machine `step()` loop: fetch → dispatch → pc += 4
- [x] `ADDI` instruction (only fully implemented instruction)
- [x] ISA module scaffolding: rv32i, rv32m, rv32a, rv32f, rv32zicsr
- [x] ADR-001: learning-first direction document

### Stubbed (function signatures exist, bodies are empty)
- [ ] RV32I: ADD, SUB, SLT, SLTU, AND, OR, XOR, SLL, SRL, SRA
- [ ] RV32I: SLTI, SLTIU, ANDI, ORI, XORI, SLLI, SRLI, SRAI
- [ ] RV32I: LUI, AUIPC
- [ ] RV32I: LB, LH, LW, LBU, LHU, SB, SH, SW
- [ ] RV32I: BEQ, BNE, BLT, BGE, BLTU, BGEU
- [ ] RV32I: JAL, JALR
- [ ] RV32I: ECALL, EBREAK
- [ ] RV32M: MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU
- [ ] RV32A: LR.W, SC.W, all AMO ops
- [ ] RV32F: all floating-point ops
- [ ] RV32Zicsr: CSRRW, CSRRS, CSRRC, CSRWI, CSRSI, CSRRCI
- [ ] CSR file (mstatus, mtvec, mepc, mcause, mtval, mscratch)
- [ ] Trap routing (exception → mtvec, mret)
- [ ] ELF loader
- [ ] riscv-tests integration
- [ ] Instruction tracing / disassembly output
- [ ] Unit tests

### Empty Directories (not yet wired into lib.rs)
- `csr/` — will hold CSR register model
- `exec/` — will hold execution helpers
- `util/` — will hold shared utilities

### Not Yet Started
- [ ] Proper `Instruction` enum / decoded representation
- [ ] Full decode dispatch (branch, jump, lui, auipc, ecall, ebreak)
- [ ] Memory-mapped I/O
- [ ] Privilege transitions
- [ ] GDB stub
- [ ] Performance work (JIT, fast paths)

---

## Invariants (enforced in code)
1. `x0 == 0` always (reads return 0, writes ignored)
2. `pc` must be 4-byte aligned at fetch
3. All memory accesses are little-endian
4. Load/store operations check alignment before access
5. Unknown opcode → `IllegalInstruction` trap (currently panics — needs fixing)

---

## Next Immediate Actions (This Week)
1. Implement remaining RV32I register-register ops (ADD, SUB, SLT, SLTU, AND, OR, XOR, SLL, SRL, SRA)
2. Implement remaining RV32I immediate ops (SLTI, SLTIU, ANDI, ORI, XORI, SLLI, SRLI, SRAI)
3. Implement LUI, AUIPC
4. Implement branch instructions (BEQ, BNE, BLT, BGE, BLTU, BGEU)
5. Implement JAL, JALR
6. Wire load/store instructions to RAM (LB, LH, LW, LBU, LHU, SB, SH, SW)
7. Replace `panic!` on unknown opcode with `IllegalInstruction` trap
8. Add unit tests for each instruction group

---

## Weekly Checkpoint Log

| Week | Dates | Focus | Status | Notes |
|------|-------|-------|--------|-------|
| — | — | Project scaffolding | ✅ Done | Workspace, CPU, RAM, exceptions, ADR-001 |
| — | — | RV32I complete | 🔲 Pending | Only ADDI implemented so far |
| — | — | Traps + CSRs | 🔲 Pending | |
| — | — | M extension | 🔲 Pending | |
| — | — | Testing + Spike diff | 🔲 Pending | |
| — | — | Platform / ELF boot | 🔲 Pending | |

---

## Architecture Notes

- **Current decode strategy:** Direct opcode match in `Machine::step()` — no intermediate `Instruction` enum yet. This is fine for learning but will need refactoring before branches/jumps.
- **Memory model:** Flat 1 GiB RAM. No memory map regions, no MMIO yet.
- **Privilege:** CPU tracks `PrivilegeMode` but no CSR-backed privilege transitions exist.
- **Dependencies:** Zero external crates. Consider adding `thiserror`, `tracing`, `object` (for ELF) when ready.
