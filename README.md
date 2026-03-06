# rustvcore

A learning-first RISC-V emulator written in Rust.

This repository is intentionally built as an educational codebase: simple structure, explicit instruction handlers, and staged feature growth from minimal CPU behavior toward Linux-capable emulation.

## Project Status

Current state (as of 2026-03-06):
- Workspace contains `rvcore` for emulator primitives (`CPU`, `RAM`, `Machine`)
- Workspace contains `rvcli` as a minimal executable using `rvcore`
- CPU register model exists (`x0..x31`) with x0 read-only behavior enforced
- Instruction handler function scaffolding exists for `I`, `M`, `A`, and `F` families
- Most instruction bodies are placeholders by design (for incremental learning)
- RAM and machine wiring are started, with memory access logic mostly stubbed

This project is not yet feature-complete or spec-compliant. That is expected for this phase.

## Goals

- Build a clean, understandable RISC-V emulator from first principles
- Keep architecture modular so features can be layered without rewrites
- Learn ISA details through implementation, testing, and traceability
- Gradually move from toy examples to realistic program execution

## Non-Goals (for now)

- Production-grade performance from day one
- Full Linux compatibility immediately
- Implementing every extension before core correctness

## Repository Layout

- `rvcore/src/cpu/core.rs`: CPU state and instruction handler methods
- `rvcore/src/memory/ram.rs`: RAM model and load/store entry points
- `rvcore/src/machine/machine.rs`: composition of CPU + RAM
- `rvcli/src/main.rs`: minimal runner / scratchpad
- `docs/`: optional design notes and deep-dive docs

## Build and Run

Requirements:
- Rust stable toolchain (edition 2024 compatible)

Commands:
```bash
cargo check
cargo run -p rvcli
```

## Design Direction

### Core Principles

- Make decode and execute paths explicit
- Keep each instruction handler narrow and testable
- Prioritize correctness and visibility over premature optimization
- Use clear state ownership (`CPU`, `RAM`, `Machine`) with limited hidden side effects

### Recommended Execution Pipeline (Next)

1. Fetch instruction at `pc`
2. Decode opcode/funct fields into instruction enum
3. Dispatch to instruction handler
4. Update architectural state (`regs`, `pc`, memory, CSRs)
5. Optionally emit trace/log event

## Instruction Coverage Strategy

Implement in this order for fastest learning feedback:

1. **RV32I base integer subset**: arithmetic, logic, branches, jumps, loads/stores
2. **System/CSR essentials**: `ecall`, `ebreak`, minimal `zicsr`
3. **M extension**: multiply/divide
4. **A extension**: atomics
5. **F extension**: float ops (or postpone until integer path is solid)

## What To Build More (Prioritized)

You listed strong future ideas. Here is the practical order to maximize progress.

### Tier 1: Must-Have Foundations

- Decode layer with strongly typed `Instruction` enum
- Proper memory semantics (byte/half/word loads/stores + sign extension)
- Exception/trap framework (illegal instruction, memory faults, ecall)
- Unit tests per instruction family
- ISA compliance testing against RISC-V test binaries

Reason: without these, advanced extensions are hard to verify.

### Tier 2: Developer Tooling (High ROI)

- Step-by-step instruction tracing/logging
- Register dump per step (`pc`, modified regs, optional memory diffs)
- Basic interactive debug shell (`step`, `continue`, `regs`, `mem`)
- GDB stub integration

Reason: debugging speed directly affects learning velocity.

### Tier 3: Feature Extensions

- `B` extension (`Zba`, `Zbb`, `Zbc`, `Zbs`)
- `V` extension (vector)
- `Zfinx` (float in integer registers)

Reason: valuable but should follow stable base semantics.

### Tier 4: Platform/Peripheral Growth

- VirtIO block + better syscall coverage first
- VirtIO net (network)
- VirtIO GPU / framebuffer output

Reason: these move you toward realistic OS/userspace execution.

### Tier 5: Performance and UX

- JIT path (after interpreter correctness baseline is proven)
- Web UI with live register/memory views
- Pipeline visualization (fetch/decode/execute timeline)

Reason: optimize and visualize once behavior is trustworthy.

## Suggested Milestones

- **M1**: RV32I integer correctness + tests + traces
- **M2**: traps/CSR + ELF loading + simple userspace programs
- **M3**: `M` + `A` extensions + compliance tests
- **M4**: debugger + GDB stub
- **M5**: peripheral model + syscall expansion
- **M6**: optional JIT and UI tooling

## Quality Checklist

For each implemented instruction:
- Decode test exists
- Execute-state transition test exists
- Edge cases covered (signed/unsigned, overflow behavior, x0 behavior)
- Trace output validated (if tracing enabled)

For each subsystem:
- Public API documented
- Error paths explicit
- Behavior tested at unit + integration level

## Contribution Workflow (Personal or Team)

- Keep PRs/slices small (single subsystem or instruction family)
- Add tests in same change as logic
- Avoid mixing refactors with behavior changes
- Maintain a short `docs/` note for any non-obvious architectural decision

## Near-Term TODO (Concrete)

1. Build decode enum and dispatcher
2. Implement core RV32I arithmetic + register tests
3. Implement `load32`/`store32` behavior in RAM
4. Add trace mode (behind feature flag or config)
5. Add first compliance test target and CI check

## License

Add a license file before open distribution.

Suggested default for learning projects: MIT.
