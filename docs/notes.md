Application
   |
My Init(Systemd in rust)
   |
My kernel
   |
Emulator
   |
Hardware


Instead of cpu -> ram
 we have 
CPU
 | 
Bus
 |-RAM
 |-UART
 |-CLINT
 |-PLIC
 |-VirtIO
 |-Flash

Milestone 1: RV32I emulator
Milestone 2: Boot bare metal binaries
Milestone 3: Boot xv6-riscv
Milestone 4: Boot linux
Milestone 5: Write Rust init
Milestone 6: Own kernel


`sudo pacman -S riscv64-unknown-elf-gcc riscv64-unknown-elf-binutils`
it natively supports compiling the both RV32 and RV64 architecture using the `-march`
and `-mabi` flags (`-march=rv32i  -mabi=ilp32`)

Installing the Spike simulator
Official `riscv-tests`

introduce differential testing (it means `my emulator` - `registers after instruction N` - `Compare` - `Spike`) and if they differ then we know we found a bug













