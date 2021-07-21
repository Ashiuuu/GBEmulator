use crate::bus;
use crate::instructions;

pub struct Register {
    pub low: u8,
    pub high: u8,
}

impl Register {
    fn new_register() -> Register {
        Register { low: 0, high: 0 }
    }

    pub fn get_combined(&self) -> u16 {
        ((self.high as u16) << 8) + (self.low as u16)
    }

    pub fn set_low_word(&mut self, data: u16) {
        self.low = (data & 0xFF) as u8;
    }

    pub fn set_high_word(&mut self, data: u16) {
        self.high = ((data & 0xFF00) >> 8) as u8;
    }

    pub fn set_word(&mut self, data: u16) {
        self.set_low_word(data);
        self.set_high_word(data);
    }
}

pub struct CPU {
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: u16,
    pub pc: u16,
    clock_cycles_to_go: u8,
    pub stopped: bool,
    pub halted: bool,
    pub ime: bool,
    bus: bus::Bus,
}

impl CPU {
    pub fn new_cpu(filename: &String) -> CPU {
        CPU {
            af: Register::new_register(),
            bc: Register::new_register(),
            de: Register::new_register(),
            hl: Register::new_register(),
            sp: 0,
            pc: 0x100,
            clock_cycles_to_go: 0,
            stopped: false,
            halted: false,
            ime: true,
            bus: bus::Bus::new_bus(filename),
        }
    }

    pub fn tick(&mut self, debugging: bool) {
        // execute a tick of the CPU
        if self.clock_cycles_to_go > 0 {
            self.clock_cycles_to_go -= 1;
        } else {
            self.execute_instruction(debugging);
        }
    }

    pub fn extract_flag(&self, c: char) -> bool {
        match c {
            'z' => self.af.low & 0b10000000 != 0,
            'n' => self.af.low & 0b1000000 != 0,
            'h' => self.af.low & 0b100000 != 0,
            'c' => self.af.low & 0b10000 != 0,
            _ => {
                eprintln!("Invalid flag character : {}", c);
                false
            }
        }
    }

    pub fn set_flag(&mut self, c: char) {
        match c {
            'z' => self.af.low |= 0b10000000,
            'n' => self.af.low |= 0b1000000,
            'h' => self.af.low |= 0b100000,
            'c' => self.af.low |= 0b10000,
            _ => eprintln!("Invalid flag character : {}", c),
        };
    }

    pub fn clear_flag(&mut self, c: char) {
        match c {
            'z' => self.af.low &= 0b01111111,
            'n' => self.af.low &= 0b0111111,
            'h' => self.af.low &= 0b011111,
            'c' => self.af.low &= 0b01111,
            _ => eprintln!("Invalid flag character : {}", c),
        };
    }

    pub fn update_flag(&mut self, c: char, val: bool) {
        if val == true {
            self.set_flag(c);
        } else {
            self.clear_flag(c);
        }
    }

    fn execute_instruction(&mut self, debugging: bool) {
        // fetch instruction byte on bus based on pc register
        let op = self.bus.fetch_byte(self.pc);
        let current_instruction = &instructions::Instruction::SET[op as usize];
        if debugging {
            println!("{:#x} : {}", op, current_instruction.disassembly);
            println!("HL: {:#04x}\nBC: {:#04x}\nDE: {:#04x}\nA: {:#02x}\nPC: {:#04x}\nSP: {:#04x}", self.hl.get_combined(), self.bc.get_combined(), self.de.get_combined(), self.af.high, self.pc, self.sp);
            println!("Z: {}\nH: {}\nN: {}\nC: {}", self.extract_flag('z'), self.extract_flag('h'), self.extract_flag('n'), self.extract_flag('c'));
            println!("Memory: {:#02x}    {:#02x}", self.fetch_byte(self.pc + 1), self.fetch_byte(self.pc + 2));
            let mut cont = String::new();
            std::io::stdin().read_line(&mut cont).expect("Unable to read from stdin !");
        }
        // identify instruction and execute it
        self.pc += 1;
        let previous_pc = self.pc;
        (current_instruction.execute)(self);
        if previous_pc == self.pc {
            // TEMPORARY ; CAN POTENTIALLY CAUSE BUGS IF JUMP OF 0 (although nonsensical)
            //if a jump did not occurr
            self.pc += current_instruction.op_len - 1;
        }
        self.clock_cycles_to_go += current_instruction.clock_cycles;
    }

    pub fn fetch_byte(&self, address: u16) -> u8 {
        self.bus.fetch_byte(address)
    }

    pub fn set_byte(&mut self, address: u16, data: u8) {
        self.bus.set_byte(address, data);
    }

    pub fn set_word(&mut self, address: u16, data: u16) {
        self.bus.set_word(address, data);
    }
}
