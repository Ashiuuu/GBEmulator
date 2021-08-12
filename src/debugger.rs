use crate::cpu;
use crate::bus;
use crate::gpu;
use crate::instructions;
use crate::instructions2;

pub struct Debugger {
    breakpoints: Vec<u16>,
    value_bp_de: Vec<u16>,
    paused: bool,
    view_tileset: bool,
}

impl Debugger {
    pub fn new_debugger() -> Debugger {
        let mut ret = Debugger {breakpoints: vec![0], value_bp_de: vec![0], paused: false, view_tileset: false, };
        ret.value_bp_de.pop();
        ret
    }

    pub fn start_paused(&mut self){
        self.paused = true;
    }
    
    pub fn viewing_tileset(&self) -> bool {
        self.view_tileset
    }

    fn add_breakpoint(&mut self, bp: u16) {
        if self.is_a_breakpoint(bp) {
            return; // don't allow multiple breakpoints at same address
        }
        self.breakpoints.push(bp);
    }

    fn is_a_breakpoint(&self, bp: u16) -> bool {
        self.breakpoints.iter().any(|&i| i == bp)
    }

    fn is_a_de_val_bp(&self, val: u16) -> bool {
        self.value_bp_de.iter().any(|&i| i == val)
    }

    fn remove_breakpoint(&mut self, bp: u16) {
        if self.is_a_breakpoint(bp) == false {
            return;
        }
        let index = self.breakpoints.iter().position(|i| *i == bp).unwrap();
        self.breakpoints.remove(index);
    }

    pub fn set_paused(&mut self, new: bool) {
        self.paused = new;
    }

    fn print_help() {
        println!("Commands :");
        println!("b: breakpoint manipulation");
        println!("   - add XX : add breakpoint at address XX");
        println!("   - rem XX : remove breakpoint at address XX ; does nothing if breakpoint does not exist");
        println!("   - list : list all breakpoints");
        println!("p: print cpu state");
        println!("v: add value based breakpoint of a register");
        println!("c: continue running");
        println!("d: dump memory");
        println!("t: switch display to tileset");
        println!("s: perform one program step");
    }

    fn str_to_decimal(number: &str) -> u16 {
        if number.starts_with("0x") {
            // hexadecimal
            let nb = number.strip_prefix("0x").unwrap();
            let res = u16::from_str_radix(nb, 16).unwrap();
            return res;
        } else {
            return u16::from_str_radix(number, 10).unwrap();
        }
    }

    fn dump_memory(bus: &bus::Bus, start: u32, length: u32) {
        for i in 0..length {
            if start + i > 0xFFFF {
                // if memory address exceeds address range, stop
                println!("");
                return;
            }
            print!("{:#04x} ", bus.fetch_byte((start + i) as u16));
        }
        println!("");
    }

    fn dump_registers(cpu: &cpu::CPU, bus: &bus::Bus) {
        println!("BC: {}", cpu.bc.print());
        println!("DE: {}", cpu.de.print());
        println!("HL: {}", cpu.hl.print());
        println!("A: {:#04x}", cpu.af.high);
        println!("F: {:#04x}   |  Z: {}   H: {}   N: {}   C: {}", cpu.af.low, cpu.extract_flag('z'), cpu.extract_flag('h'), cpu.extract_flag('n'), cpu.extract_flag('c'));
        println!("PC: {:#06x}", cpu.pc);
        println!("SP: {:#06x}", cpu.sp);
        println!("Memory: {:#04x} {:#04x}", bus.fetch_byte(cpu.pc + 1), bus.fetch_byte(cpu.pc + 2));
        //println!("Stack: {:#04x} {:#04x} {:#04x} {:#04x}", bus.fetch_byte(self.sp - 2), bus.fetch_byte(self.sp - 1), bus.fetch_byte(self.sp), bus.fetch_byte(self.sp + 1));
        println!("");
    }

    fn parse_command(&mut self, command: String, bus: &bus::Bus, cpu: &cpu::CPU) {
        let tokens: Vec<&str> = command.split(' ').collect();
        let com = tokens[0].chars().nth(0);

        if com == Some('b') {
            if tokens[1].starts_with("rem") {
                let address = Debugger::str_to_decimal(tokens[2]);
                self.remove_breakpoint(address);
            } else if tokens[1].starts_with("add") {
                let address = Debugger::str_to_decimal(tokens[2]);
                self.add_breakpoint(address);
            } else if tokens[1].starts_with("list") {
                println!("Breakpoints :");
                let mut i = 0;
                for b in &self.breakpoints {
                    println!("{}: {:#06x}", i, b);
                    i += 1;
                }
            } else {
                println!("Invalid breakpoint command: {}", tokens[1]);
            }
        } else if com == Some('c') {
            self.paused = false;
        } else if com == Some('h') {
            Debugger::print_help();
        } else if com == Some('t') {
            if self.viewing_tileset() == true {
                self.view_tileset = false;
            } else {
                self.view_tileset = true;
            }
        } else if com == Some('d') {
            let start = Debugger::str_to_decimal(tokens[1]) as u32;
            let length = Debugger::str_to_decimal(tokens[2]) as u32;
            Debugger::dump_memory(bus, start, length);
        } else if com == Some('v') {
            let reg = tokens[1];
            let value = Debugger::str_to_decimal(tokens[2]);
            if reg == "de" {
                self.value_bp_de.push(value);
            }
        } else if com == Some('p') {
            let op = bus.fetch_byte(cpu.pc);
            let current_instruction = match op {
                0xCB => &instructions2::Instruction::SECOND_SET[op as usize],
                _ => &instructions::Instruction::SET[op as usize],
            };
            println!("{:#x} : {}", op, current_instruction.disassembly);
            Debugger::dump_registers(cpu, bus);
        } else if com == Some('s') {
            // step through programm  
        } else {
            println!("Invalid command {}", tokens[0]);
        }
    }

    fn tick_devices(&self, cpu: &mut cpu::CPU, bus: &mut bus::Bus, gpu: &mut gpu::GPU, keys: &mut crate::Keys, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        keys.update_register(bus);
        if self.viewing_tileset() == true {
            gpu::GPU::dump_tileset(&bus, canvas);
        } else {
            gpu.tick(bus, canvas);
        }
        cpu.tick(bus);
}

    pub fn tick(&mut self, cpu: &mut cpu::CPU, bus: &mut bus::Bus, gpu: &mut gpu::GPU, keys: &mut crate::Keys, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.paused == false {
            // check breakpoints and stuff
            if self.is_a_breakpoint(cpu.pc) {
                self.paused = true;
                println!("Breakpoint {} reached !", cpu.pc);
            } else if self.is_a_de_val_bp(cpu.de.get_combined()) {
                self.paused = true;
                println!("Value breakpoint {} reached !", cpu.de.get_combined());
            }
            self.tick_devices(cpu, bus, gpu, keys, canvas);
        } else {
            if cpu.get_clock_cycles() == 0 {
                // command handling
                println!("Program is paused");
                let mut command = String::new();
                ::std::io::stdin().read_line(&mut command).expect("Unable to read from stind from debugger tick function");
                command.pop(); // remove newline character
                command.pop(); // remove carriage return on windows
                self.parse_command(command, bus, cpu);
            } else {
                self.tick_devices(cpu, bus, gpu, keys, canvas);
            }
        }
    }
}