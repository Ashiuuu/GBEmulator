use derive_more::Display;

use std::fmt;
use std::fmt::{Display, Formatter};

use crate::bus::Bus;
use crate::cpu::{Registers, CPU};
use crate::registers::RegisterPart;

#[derive(Clone, Copy)]
enum Target {
    HalfRegister(Registers, RegisterPart),
    Register(Registers),
    ImmediateByte,
    ImmediateWord,
    IndirectRegister(Registers),
    IndirectImmediate,
    IndirectIOPort,
    None,
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Target::HalfRegister(reg, _) => reg.as_str(), // TODO CHANGE
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

#[derive(Clone, Copy)]
enum Sized {
    Zero,
    Byte(u8),
    Word(u16),
}

impl Sized {
    pub fn number_of_bytes(&self) -> u16 {
        match self {
            Sized::Zero => 0,
            Sized::Byte(_) => 1,
            Sized::Word(_) => 2,
        }
    }
}

impl From<u8> for Sized {
    fn from(data: u8) -> Self {
        Sized::Byte(data)
    }
}

impl From<Sized> for u8 {
    fn from(data: Sized) -> Self {
        match data {
            Sized::Byte(d) => d,
            _ => panic!("Trying to cast non byte Sized to u8"),
        }
    }
}

impl From<u16> for Sized {
    fn from(data: u16) -> Self {
        Sized::Word(data)
    }
}

impl From<Sized> for u16 {
    fn from(data: Sized) -> Self {
        match data {
            Sized::Word(d) => d,
            _ => panic!("Trying to cast non word Sized to u16"),
        }
    }
}

impl Target {
    pub fn fetch(self, bus: &Bus, cpu: &CPU) -> Sized {
        match self {
            Target::HalfRegister(reg, part) => cpu.get_register_byte(reg, part).into(),
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
            Target::HalfRegister(reg, part) => cpu.set_half_register(reg, part, data.into()),
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

#[derive(Copy, Clone, Display)]
pub enum Operation {
    Nop,
    Jmp,

    Illegal,
}

impl Operation {
    pub fn should_advance_pc(&self) -> bool {
        match self {
            Operation::Jmp => false,
            _ => true,
        }
    }

    pub fn execute(&self, bus: &Bus, cpu: &mut CPU, source: Sized) -> Sized {
        match self {
            Self::Nop => Sized::Zero,
            Self::Jmp => jmp(bus, cpu, source),

            Self::Illegal => panic!("Illegal instruction reached"),
            _ => unimplemented!(),
        }
    }
}

pub struct Instruction {
    op: Operation,
    source: Target,
    dest: Target,
    clock_cycles: u8,
    op_byte_len: u16,
}

impl Instruction {
    const OPCODE_TABLE: [(Operation, Target, Target, u8); 1] =
        [(Operation::Nop, Target::None, Target::None, 4)];

    pub fn fetch_new(bus: &Bus, pc: u16) -> Self {
        let opcode = bus.fetch_byte(pc) as usize;
        if opcode >= Instruction::OPCODE_TABLE.len() {
            panic!("Instruction is not implemented: {:#02x}", opcode);
        }

        let op_byte_len = match opcode {
            0xCB => 2,
            _ => 1,
        };

        let (op, source, dest, clock_cycles) = Instruction::OPCODE_TABLE[opcode];
        Instruction {
            op,
            source,
            dest,
            clock_cycles,
            op_byte_len,
        }
    }

    // Execute an instruction. Returns the number of clock cycles to wait
    pub fn execute(self, bus: &mut Bus, cpu: &mut CPU) -> u8 {
        let mut instruction_length = self.op_byte_len;
        let source = self.source.fetch(bus, cpu);
        instruction_length += source.number_of_bytes();

        let result = self.op.execute(bus, cpu, source);

        self.dest.write(bus, cpu, result);

        if self.op.should_advance_pc() {
            cpu.pc += instruction_length;
        }

        self.clock_cycles
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.op, self.dest, self.source)
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            op: Operation::Illegal,
            source: Target::None,
            dest: Target::None,
            clock_cycles: 0,
            op_byte_len: 0,
        }
    }
}

// INSTRUCTION FUNCTIONS

fn jmp(bus: &Bus, cpu: &mut CPU, source: Sized) -> Sized {
    cpu.pc = source.into();

    Sized::Zero
}
