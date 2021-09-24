use crate::bus;
use crate::cpu;
use crate::gpu;
use crate::instructions;
use crate::instructions2;

use std::env;

use std::io::stdout;
use std::io::Write;

enum CommandType {
    Breakpoint,
    Continue,
    Step,
    Dump,
    ValueBp,
    Print,
    Help,
    Invalid,
}

impl PartialEq for CommandType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct Command {
    name: CommandType,
    args: Vec<String>,
}

impl Command {
    pub fn new_command() -> Command {
        let mut ret = Command {
            name: CommandType::Invalid,
            args: vec!["".to_string()],
        };
        ret.args.pop();
        ret
    }
}

pub struct Debugger {
    breakpoints: Vec<u16>,
    value_bp_de: Vec<u16>,
    paused: bool,
    stepping: bool,
}

impl Debugger {
    pub fn new_debugger() -> Debugger {
        let mut ret = Debugger {
            breakpoints: vec![0],
            value_bp_de: vec![0],
            paused: false,
            stepping: false,
        };
        ret.value_bp_de.pop();
        ret.breakpoints.pop();
        ret
    }

    fn add_breakpoint(&mut self, bp: u16) -> i16 {
        if self.is_a_breakpoint(bp) {
            return -1; // don't allow multiple breakpoints at same address
        }
        self.breakpoints.push(bp);
        (self.breakpoints.len() as i16) - 1
    }

    fn is_a_breakpoint(&self, bp: u16) -> bool {
        self.breakpoints.iter().any(|&i| i == bp)
    }

    fn is_a_de_val_bp(&self, val: u16) -> bool {
        self.value_bp_de.iter().any(|&i| i == val)
    }

    fn remove_breakpoint(&mut self, bp: u16) -> i16 {
        if self.is_a_breakpoint(bp) == false {
            return -1;
        }
        let index = self.breakpoints.iter().position(|i| *i == bp).unwrap();
        self.breakpoints.remove(index);
        index as i16
    }

    pub fn set_paused(&mut self, new: bool) {
        self.paused = new;
    }

    fn print_help() {
        println!("Commands :");
        println!("b: breakpoint manipulation");
        println!("p: print cpu state");
        println!("v: add value based breakpoint of a register");
        println!("c: continue running");
        println!("d: dump memory");
        println!("s: perform one program step");
    }

    fn print_b_help() {
        println!("b - Breakpoint manipulation commands");
        println!("Subcommand list :");
        println!("add n : add breakpoint at address n");
        println!("         n can be in decimal (n) format or hexadecinal (0xn) format");
        println!("rem n : remove breakpoint at address n");
        println!("list : list all set breakpoints");
        println!("clear : remove all breakpoints");
        println!("Addresses can be written in either decimal or hexadecimal format with a 0x prefix");
    }

    fn string_to_decimal(number: &String) -> u16 {
        if number.starts_with("0x") {
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
        println!(
            "F: {:#04x}   |  Z: {}   H: {}   N: {}   C: {}",
            cpu.af.low,
            cpu.extract_flag('z'),
            cpu.extract_flag('h'),
            cpu.extract_flag('n'),
            cpu.extract_flag('c')
        );
        println!("PC: {:#06x}", cpu.pc);
        println!("SP: {:#06x}", cpu.sp);
        println!(
            "Memory: {:#04x} {:#04x}",
            bus.fetch_byte(cpu.pc + 1),
            bus.fetch_byte(cpu.pc + 2)
        );
        //println!("Stack: {:#04x} {:#04x} {:#04x} {:#04x}", bus.fetch_byte(self.sp - 2), bus.fetch_byte(self.sp - 1), bus.fetch_byte(self.sp), bus.fetch_byte(self.sp + 1));
        println!("");
    }

    fn exec_command(&mut self, command: &Command, bus: &bus::Bus, cpu: &cpu::CPU) {
        if command.name == CommandType::Breakpoint {
            let sub_co = &command.args[0];
            if sub_co == "rem" {
                let address = Debugger::string_to_decimal(&command.args[1]);
                let pos = self.remove_breakpoint(address);
                if pos == -1 {
                    println!("Breakpoint at address {:#04x} does not exist", address);
                } else {
                    println!("Removed breakpoint #{} at address {:#04x}", pos, address);
                }
            } else if sub_co == "add" {
                let address = Debugger::string_to_decimal(&command.args[1]);
                let pos = self.add_breakpoint(address);
                if pos == -1 {
                    println!("Breakpoint at address {:#04x} already exists", address);
                } else {
                    println!("Added breakpoint #{} at address {:#04x}", pos, address);
                }
            } else if sub_co == "list" {
                if self.breakpoints.len() == 0 {
                    println!("Breakpoint list is empty");
                } else {
                    println!("Breakpoints :");
                    let mut i = 0;
                    for b in &self.breakpoints {
                        println!("{}: {:#06x}", i, b);
                        i += 1;
                    }
                }
            } else if sub_co == "clear" {
                self.breakpoints.clear();
            } else {
                println!("Invalid breakpoint command : {}", command.args[0]);
            }
        } else if command.name == CommandType::Continue {
            self.paused = false;
            self.stepping = false;
        } else if command.name == CommandType::Help {
            if command.args.len() == 0 {
                Debugger::print_help();
            } else if command.args.len() != 1 {
                println!("Too many arguments. Usage : help [command]");
            } {
                let sub_co = &command.args[0];
                if sub_co == "b" {
                    Debugger::print_b_help();
                } else {
                    println!("No available help for command {}", sub_co);
                }
            }
        } else if command.name == CommandType::Dump {
            let start = Debugger::string_to_decimal(&command.args[0]) as u32;
            let length = Debugger::string_to_decimal(&command.args[1]) as u32;
            print!("{:#04x}[0..{}]: ", start, length);
            Debugger::dump_memory(bus, start, length);
        } else if command.name == CommandType::ValueBp {
            let reg = &command.args[0];
            let value = Debugger::string_to_decimal(&command.args[1]);
            if reg == "de" {
                self.value_bp_de.push(value);
            }
        } else if command.name == CommandType::Print {
            let op = bus.fetch_byte(cpu.pc);
            let current_instruction = match op {
                0xCB => &instructions2::Instruction::SECOND_SET[bus.fetch_byte(cpu.pc + 1) as usize],
                _ => &instructions::Instruction::SET[op as usize],
            };
            println!("{:#x} : {}", op, current_instruction.disassembly);
            Debugger::dump_registers(cpu, bus);
        } else if command.name == CommandType::Step {
            if self.stepping == false {
                println!("Entering step mode");
            }
            self.stepping = true;
        }
    }

    fn parse_command(&mut self, command: String) -> Command {
        let tokens: Vec<&str> = command.split(' ').collect();
        let mut ret = Command::new_command();

        for i in 1..tokens.len() {
            ret.args.push(tokens[i].to_string());
        }

        ret.name = match tokens[0] {
            "b" => CommandType::Breakpoint,
            "c" => CommandType::Continue,
            "h" => CommandType::Help,
            "d" => CommandType::Dump,
            "v" => CommandType::ValueBp,
            "p" => CommandType::Print,
            "s" => CommandType::Step,
            _ => { println!("Invalid command : {}", tokens[0]); CommandType::Invalid },
        };

        return ret;
    }

    fn tick_devices(
        &self,
        cpu: &mut cpu::CPU,
        bus: &mut bus::Bus,
        gpu: &mut gpu::GPU,
        keys: &mut crate::Keys,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        keys.update_register(bus);
        gpu.tick(bus, canvas);
        cpu.tick(bus);
    }

    fn handle_command(&mut self, bus: &bus::Bus, cpu: &cpu::CPU) -> CommandType {
        print!("> ");
        stdout().flush().unwrap();
        let mut command = String::new();
        ::std::io::stdin()
            .read_line(&mut command)
            .expect("Unable to read from stind from debugger tick function");
        command.pop(); // remove newline character
        if env::consts::OS == "windows" {
            command.pop(); // remove carriage return on windows
        }

        let com = self.parse_command(command);
        if com.name != CommandType::Invalid {
            self.exec_command(&com, bus, cpu);
        }

        com.name
    }

    pub fn tick(&mut self, cpu: &mut cpu::CPU, bus: &mut bus::Bus, gpu: &mut gpu::GPU, keys: &mut crate::Keys, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        //check for breakpoints
        if self.is_a_breakpoint(cpu.pc) {
            if self.paused == false {
                println!("Breakpoint at address {:#04x} reached !", cpu.pc);
            }
            self.paused = true;
        } else if self.is_a_de_val_bp(cpu.de.get_combined()) {
            if self.paused == false {
                println!("Value breakpoint {:#04x} reached !", cpu.de.get_combined());
            }
            self.paused = true;
        }
        //if stopped
        if self.paused == false && self.stepping == false {
            self.tick_devices(cpu, bus, gpu, keys, canvas);
        }
        //else
        else {
            if cpu.get_clock_cycles() == 0 {
                let com = self.handle_command(bus, cpu);

                if com == CommandType::Step || com == CommandType::Continue {
                    self.tick_devices(cpu, bus, gpu, keys, canvas);
                }
            } else {
                self.tick_devices(cpu, bus, gpu, keys, canvas);
            }
        }
    }
}
