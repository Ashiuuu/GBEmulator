use crate::cpu;
use crate::bus;
use crate::gpu;

pub struct Debugger {
    breakpoints: Vec<u16>,
    paused: bool,
}

impl Debugger {
    pub fn new_debugger() -> Debugger {
        Debugger {breakpoints: vec![0], paused: false, }
    }

    pub fn start_paused(&mut self){
        self.paused = true;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
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

    fn remove_breakpoint(&mut self, bp: u16) {
        if self.is_a_breakpoint(bp) == false {
            return;
        }
        let index = self.breakpoints.iter().position(|i| *i == bp).unwrap();
        self.breakpoints.remove(index);
    }

    fn print_help() {
        println!("Commands :");
        println!("b: breakpoint manipulation");
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

    fn parse_command(&mut self, command: String) {
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
        } else if com == Some('h') {
            Debugger::print_help();
        } else {
            println!("Invalid command {}", tokens[0]);
        }
    }

    pub fn tick(&mut self, cpu: &cpu::CPU, bus: &bus::Bus, gpu: &gpu::GPU) {
        if self.paused == false {
            // check breakpoints and stuff
        }
        let mut command = String::new();
        ::std::io::stdin().read_line(&mut command).expect("Unable to read from stind from debugger tick function");

        self.parse_command(command);
    }
}