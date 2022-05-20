use derive_more::Display;

use crate::bus::Bus;
use crate::instructions::Instruction;
use crate::registers::*;

struct InterruptFlags {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl InterruptFlags {
    pub fn from_byte(data: u8) -> Self {
        InterruptFlags {
            vblank: data & 0b1 != 0,
            lcd_stat: data & 0b10 != 0,
            timer: data & 0b100 != 0,
            serial: data & 0b1000 != 0,
            joypad: data & 0b10000 != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut result = 0;

        if self.vblank {
            result |= 0b1;
        };
        if self.lcd_stat {
            result |= 0b10;
        };
        if self.timer {
            result |= 0b100;
        };
        if self.serial {
            result |= 0b1000;
        };
        if self.joypad {
            result |= 0b10000;
        };

        result
    }
}

#[derive(Debug, Display, Clone, Copy)]
pub enum Registers {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Registers {
    pub fn as_str(&self) -> &str {
        match self {
            AF => "AF",
            BC => "BC",
            DE => "DE",
            HL => "HL",
            SP => "SP",
            PC => "PC",
        }
    }
}

pub struct CPU {
    pub af: AFRegister,
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
            af: AFRegister::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: 0,
            pc: 0x100,
            clock_cycles_to_go: 0,
            stopped: false,
            halted: false,
            ime: false,
        }
    }

    pub fn get_register_word(&self, reg: Registers) -> u16 {
        match reg {
            Registers::BC => self.bc.get_combined(),
            Registers::DE => self.de.get_combined(),
            Registers::HL => self.hl.get_combined(),
            Registers::AF => self.af.get_combined(),
            Registers::SP => self.sp,
            Registers::PC => self.pc,
        }
    }

    pub fn get_register_byte(&self, reg: Registers, part: RegisterPart) -> u8 {
        match reg {
            Registers::BC => self.bc.get_part(part),
            Registers::DE => self.de.get_part(part),
            Registers::HL => self.hl.get_part(part),
            Registers::AF => self.af.get_part(part),
            _ => panic!("Invalid register for get_register_byte: {}", reg),
        }
    }

    pub fn set_half_register(&mut self, reg: Registers, part: RegisterPart, data: u8) {
        match reg {
            Registers::BC => self.bc.set_part(part, data),
            Registers::DE => self.de.set_part(part, data),
            Registers::HL => self.hl.set_part(part, data),
            Registers::AF => self.af.set_part(part, data),
            _ => panic!("Invalid register for set_half_register: {}", reg),
        }
    }

    pub fn set_register(&mut self, reg: Registers, data: u16) {
        match reg {
            Registers::BC => self.bc.set_word(data),
            Registers::DE => self.de.set_word(data),
            Registers::HL => self.hl.set_word(data),
            Registers::AF => self.af.set_word(data),
            Registers::SP => self.sp = data,
            Registers::PC => self.pc = data,
        }
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        // execute a tick of the CPU
        /*let timer = bus.fetch_byte(0xFF04); // timer register to be incremented
        bus.set_byte(0xFF04, timer.wrapping_add(1));*/

        if self.clock_cycles_to_go > 0 {
            self.clock_cycles_to_go -= 1;
        } else {
            self.execute_instruction(bus);
        }
    }

    fn execute_instruction(&mut self, bus: &mut Bus) {
        // fetch and execute instruction at program counter
        let instruction = Instruction::fetch_new(bus, self.pc);
        self.clock_cycles_to_go += instruction.execute(bus, self);

        // interrupts
        if self.ime == true {
            self.check_for_interrupts(bus);
        }
    }

    fn check_for_interrupts(&mut self, bus: &mut Bus) {
        let enabled = InterruptFlags::from_byte(bus.fetch_byte(0xFFFF));
        let mut requested = InterruptFlags::from_byte(bus.fetch_byte(0xFF0F));
        if enabled.vblank && requested.vblank {
            self.ime = false;
            requested.vblank = false;
            self.push_word_to_stack(bus, self.pc);
            self.pc = 0x40;
        } else if enabled.lcd_stat && requested.lcd_stat {
            self.ime = false;
            requested.lcd_stat = false;
            self.push_word_to_stack(bus, self.pc);
            self.pc = 0x48;
        } else if enabled.timer && requested.timer {
            self.ime = false;
            requested.timer = false;
            self.push_word_to_stack(bus, self.pc);
            self.pc = 0x50;
        } else if enabled.serial && requested.serial {
            self.ime = false;
            requested.serial = false;
            self.push_word_to_stack(bus, self.pc);
            self.pc = 0x58;
        } else if enabled.joypad && requested.joypad {
            self.ime = false;
            requested.joypad = false;
            self.push_word_to_stack(bus, self.pc);
            self.pc = 0x60;
        }
        bus.set_byte(0xFF0F, requested.to_byte());
    }

    pub fn push_word_to_stack(&mut self, bus: &mut Bus, data: u16) {
        self.sp -= 2;
        bus.set_word(self.sp, data);
    }

    pub fn pop_word_from_stack(&mut self, bus: &mut Bus) -> u16 {
        let data = bus.fetch_word(self.sp);
        self.sp += 2;
        data
    }

    pub fn peek_bus_byte(&self, bus: &Bus) -> u8 {
        bus.fetch_byte(self.pc)
    }

    pub fn peek_bus_word(&self, bus: &Bus) -> u16 {
        bus.fetch_word(self.pc)
    }
}
