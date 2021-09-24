use crate::bus;
use crate::instructions;
use crate::instructions2;

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

    pub fn print(&self) -> String {
        format!("{:#04x}:{:#04x}     {:#06x}", self.high, self.low, self.get_combined())
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
}

impl CPU {
    pub fn new_cpu() -> CPU {
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
            ime: false,
        }
    }

    pub fn tick(&mut self, bus: &mut bus::Bus) {
        // execute a tick of the CPU
        /*let timer = bus.fetch_byte(0xFF04); // timer register to be incremented
        bus.set_byte(0xFF04, timer.wrapping_add(1));*/

        if self.clock_cycles_to_go > 0 {
            self.clock_cycles_to_go -= 1;
        } else {
            self.execute_instruction(bus);
        }
    }

    pub fn get_clock_cycles(&self) -> u8 {
        self.clock_cycles_to_go
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
            _ => println!("Invalid flag character : {}", c),
        };
    }

    pub fn clear_flag(&mut self, c: char) {
        match c {
            'z' => self.af.low &= 0b01111111,
            'n' => self.af.low &= 0b10111111,
            'h' => self.af.low &= 0b11011111,
            'c' => self.af.low &= 0b11101111,
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

    fn execute_instruction(&mut self, bus: &mut bus::Bus) {
        // fetch instruction byte on bus based on pc register
        let op = bus.fetch_byte(self.pc);
        let current_instruction = match op {
            0xCB => &instructions2::Instruction::SECOND_SET[bus.fetch_byte(self.pc + 1) as usize],
            _ => &instructions::Instruction::SET[op as usize],
        };

        // identify instruction and execute it
        self.pc += 1;
        let previous_pc = self.pc;
        (current_instruction.execute)(self, bus);
        if previous_pc == self.pc {
            // TEMPORARY ; CAN POTENTIALLY CAUSE BUGS IF JUMP OF 0 (although nonsensical)
            //if a jump did not occurr
            self.pc += current_instruction.op_len - 1;
        }
        self.clock_cycles_to_go += current_instruction.clock_cycles;

        // interrupts
        if self.ime == true {
            let enabled = bus.fetch_byte(0xFFFF);
            let requested = bus.fetch_byte(0xFF0F);
            if enabled & 1 == 1 && requested & 1 == 1 { // if vblank interrupt is enabled and requested
                self.ime = false;
                bus.set_byte(0xFF0F, requested ^ 1);
                self.push_stack(bus, self.pc);
                self.pc = 0x40;
            } else if enabled & 0b10 == 0b10 && requested & 0b10 == 0b10 { // LCD STAT interrupt
                self.ime = false;
                bus.set_byte(0xFF0F, requested ^ 0b10);
                self.push_stack(bus, self.pc);
                self.pc = 0x48;
            } else if enabled & 0b100 == 0b100 && requested & 0b100 == 0b100 { // Timer interrupt
                self.ime = false;
                bus.set_byte(0xFF0F, requested ^ 0b100);
                self.push_stack(bus, self.pc);
                self.pc = 0x50;
            } else if enabled & 0b1000 == 0b1000 && requested & 0b1000 == 0b1000 { // serial interrupt
                self.ime = false;
                bus.set_byte(0xFF0F, requested ^ 0b1000);
                self.push_stack(bus, self.pc);
                self.pc = 0x58;
            } else if enabled & 0b10000 == 0b10000 && requested & 0b10000 == 0b10000 { // joypad interrupt
                self.ime = false;
                bus.set_byte(0xFF0F, requested ^ 0b10000);
                self.push_stack(bus, self.pc);
                self.pc = 0x60;
            }
        }
    }

    pub fn push_stack(&mut self, bus: &mut bus::Bus, data: u16) {
        self.sp -= 2;
        bus.set_word(self.sp, data);
    }

    pub fn pop_stack_d16(&mut self, bus: &mut bus::Bus) -> u16 {
        let data = bus.fetch_word(self.sp);
        self.sp += 2;
        data
    }
}
