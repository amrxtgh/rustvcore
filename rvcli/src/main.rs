use rvcore::CPU;

fn main() {
    let mut cpu = CPU::new(1024);
    //cpu.regs[1] = 10;
    //cpu.regs[2] = 20;
    //
    //cpu.add(3, 1, 2);
    //println!("x3 = {}", cpu.read_reg(3));
    cpu.write_reg(1, 42);
    println!("x1 = {}", cpu.read_reg(1));
    println!("x0 = {}", cpu.read_reg(0)); // always 0
}
