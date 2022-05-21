use std::fmt;
use std::fmt::{Display, Formatter};

use crate::bus::*;

use super::cpu::*;
use super::sized::*;

#[derive(Clone, Copy)]
pub enum Target {
    HalfRegister(Registers),
    Register(Registers),
    ImmediateByte,
    ImmediateWord,
    IndirectRegister(Registers),
    IndirectImmediate,
    IndirectIOPort,
    None,
}

impl Target {
    pub fn fetch(self, bus: &Bus, cpu: &CPU) -> Sized {
        match self {
            Target::HalfRegister(reg) => cpu.get_register_byte(reg).into(),
            Target::Register(reg) => cpu.get_register_word(reg).into(),
            Target::ImmediateByte => cpu.peek_bus_byte(bus).into(),
            Target::ImmediateWord => cpu.peek_bus_word(bus).into(),
            Target::IndirectRegister(reg) => bus.fetch_byte(cpu.get_register_word(reg)).into(),
            Target::IndirectImmediate => bus.fetch_byte(cpu.peek_bus_word(bus)).into(),
            Target::IndirectIOPort => bus
                .fetch_byte((cpu.peek_bus_byte(bus) as u16) + 0xFF00)
                .into(),
            Target::None => Sized::Zero,
        }
    }

    pub fn write(self, bus: &mut Bus, cpu: &mut CPU, data: Sized) {
        match self {
            Target::HalfRegister(reg) => cpu.set_half_register(reg, data.into()),
            Target::Register(reg) => cpu.set_register(reg, data.into()),
            Target::ImmediateByte | Target::ImmediateWord => {
                panic!("Immediate target write, doesn't make sense")
            }
            Target::IndirectRegister(reg) => bus.set_byte(cpu.get_register_word(reg), data.into()),
            Target::IndirectImmediate => bus.set_byte(cpu.peek_bus_word(bus), data.into()),
            Target::IndirectIOPort => {
                bus.set_byte((cpu.peek_bus_byte(bus) as u16) + 0xFF00, data.into())
            }
            Target::None => {}
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Target::HalfRegister(reg) => reg.as_str(),
                Target::Register(reg) => reg.as_str(),
                Target::ImmediateByte => "XX",
                Target::ImmediateWord => "XXXX",
                Target::IndirectRegister(reg) => reg.as_str(),
                Target::IndirectImmediate => "(XXXX)",
                Target::IndirectIOPort => "0xFFXX",
                Target::None => "",
            }
        )
    }
}
