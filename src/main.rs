mod bus;
mod cpu;
mod instructions;

fn main() {
    let mut cpu = cpu::CPU::new_cpu(&String::from("Tetris.GB"));
    loop {
        cpu.tick(false);
    }
}
