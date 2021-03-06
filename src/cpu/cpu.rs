use derive_more::Display;

use super::instructions::*;
use super::registers::*;

use crate::bus::Bus;

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
    A,
    BC,
    B,
    C,
    DE,
    D,
    E,
    HL,
    H,
    L,
    SP,
}

impl Registers {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AF => "AF",
            Self::A => "A",
            Self::BC => "BC",
            Self::B => "B",
            Self::C => "C",
            Self::DE => "DE",
            Self::D => "D",
            Self::E => "E",
            Self::HL => "HL",
            Self::H => "H",
            Self::L => "L",
            Self::SP => "SP",
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
            Registers::AF => self.af.get_combined(),
            Registers::BC => self.bc.get_combined(),
            Registers::DE => self.de.get_combined(),
            Registers::HL => self.hl.get_combined(),
            Registers::SP => self.sp,
            _ => panic!(
                "Trying to access word sized data on byte sized register {}",
                reg
            ),
        }
    }

    pub fn get_register_byte(&self, reg: Registers) -> u8 {
        match reg {
            Registers::A => self.af.a,
            Registers::B => self.bc.low,
            Registers::C => self.bc.high,
            Registers::D => self.de.low,
            Registers::E => self.de.high,
            Registers::H => self.hl.low,
            Registers::L => self.hl.high,
            _ => panic!(
                "Trying to access byte sized data on word sized register {}",
                reg
            ),
        }
    }

    pub fn set_half_register(&mut self, reg: Registers, data: u8) {
        match reg {
            Registers::A => self.af.a = data,
            Registers::B => self.bc.low = data,
            Registers::C => self.bc.high = data,
            Registers::D => self.de.low = data,
            Registers::E => self.de.high = data,
            Registers::H => self.hl.low = data,
            Registers::L => self.hl.high = data,
            _ => panic!(
                "Trying to set byte sized data in word sized register {}",
                reg
            ),
        }
    }

    pub fn set_register(&mut self, reg: Registers, data: u16) {
        match reg {
            Registers::AF => self.af.set_word(data),
            Registers::BC => self.bc.set_word(data),
            Registers::DE => self.de.set_word(data),
            Registers::HL => self.hl.set_word(data),
            Registers::SP => self.sp = data,
            _ => panic!(
                "Trying to set word sized data in byte sized register {}",
                reg
            ),
        }
    }

    pub fn get_flag(&self, flag: char) -> bool {
        self.af.flags.get(flag)
    }

    pub fn set_flag(&mut self, flag: char) {
        self.af.flags.set(flag)
    }

    pub fn clear_flag(&mut self, flag: char) {
        self.af.flags.clear(flag)
    }

    pub fn update_flag(&mut self, flag: char, value: bool) {
        self.af.flags.update(flag, value)
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
        let instruction = Instruction::fetch_new(bus, self);
        println!("{:#06x}:  {}", self.pc, instruction);
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

    pub fn pop_word_from_stack(&mut self, bus: &Bus) -> u16 {
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
