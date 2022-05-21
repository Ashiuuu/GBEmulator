use std::fmt;
use std::fmt::{Display, Formatter};

use super::cpu::*;
use super::sized::*;
use super::target::*;

use crate::bus::Bus;

#[derive(Copy, Clone)]
pub enum Operation {
    Nop,
    LD,
    Jmp(Condition),
    Pop,

    Illegal,
}

impl Operation {
    pub fn should_advance_pc(&self) -> bool {
        match self {
            Operation::Jmp(_) => false,
            _ => true,
        }
    }

    pub fn execute(&self, bus: &Bus, cpu: &mut CPU, source: Sized) -> Sized {
        match self {
            Self::Nop => Sized::Zero,
            Self::LD => source,
            Self::Jmp(cond) => jmp(cpu, source, cond),
            Self::Pop => pop(bus, cpu),

            Self::Illegal => panic!("Illegal instruction reached"),
            _ => unimplemented!(),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nop => "NOP",
                Self::LD => "LD",
                Self::Jmp(cond) => match cond {
                    Condition::None => "JMP",
                    Condition::NonZero => "JNZ",
                    Condition::NoCarry => "JNC",
                },
                Self::Pop => "POP",

                Self::Illegal => "Illegal",
            }
        )
    }
}

#[derive(Clone, Copy)]
pub enum Condition {
    None,
    NonZero,
    NoCarry,
}

pub struct Instruction {
    op: Operation,
    source: Target,
    dest: Target,
    clock_cycles: u8,
    op_byte_len: u16,
}

impl Instruction {
    pub fn fetch_new(bus: &Bus, pc: u16) -> Self {
        Self::from_opcode(bus.fetch_byte(pc))
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

    fn from_opcode(opcode: u8) -> Self {
        use Registers::*;
        match opcode {
            0x00 => Instruction {
                op: Operation::Nop,
                source: Target::None,
                dest: Target::None,
                clock_cycles: 4,
                op_byte_len: 1,
            },

            0x01 => Self::ld_d16(BC),
            0x11 => Self::ld_d16(DE),
            0x21 => Self::ld_d16(HL),
            0x31 => Self::ld_d16(SP),

            0x40..=0x47 => Self::ld_d8_reg_to_reg_match(B, opcode),
            0x48..=0x4F => Self::ld_d8_reg_to_reg_match(C, opcode),
            0x50..=0x57 => Self::ld_d8_reg_to_reg_match(D, opcode),
            0x58..=0x5F => Self::ld_d8_reg_to_reg_match(E, opcode),
            0x60..=0x67 => Self::ld_d8_reg_to_reg_match(H, opcode),
            0x68..=0x6F => Self::ld_d8_reg_to_reg_match(L, opcode),

            0xC1 => Self::pop(BC),
            0xD1 => Self::pop(DE),
            0xE1 => Self::pop(HL),
            0xF1 => Self::pop(AF),

            0xC2 => Self::jmp(Condition::NonZero),
            0xC3 => Self::jmp(Condition::None),
            0xD2 => Self::jmp(Condition::NoCarry),

            _ => panic!("Opcode non implemented: {:#02x}", opcode),
        }
    }

    // OPCODE HELPERS
    fn ld_d16(reg: Registers) -> Self {
        Instruction {
            op: Operation::LD,
            source: Target::ImmediateWord,
            dest: Target::Register(reg),
            clock_cycles: 12,
            op_byte_len: 1,
        }
    }

    // TODO: Write tests to ensure the correct instructions are generated
    fn ld_d8_reg_to_reg_match(dest: Registers, opcode: u8) -> Self {
        use Registers::*;
        match (opcode & 0x0F) % 0x8 {
            0x0 => Self::ld_d8_reg_to_reg(B, dest),
            0x1 => Self::ld_d8_reg_to_reg(C, dest),
            0x2 => Self::ld_d8_reg_to_reg(D, dest),
            0x3 => Self::ld_d8_reg_to_reg(E, dest),
            0x4 => Self::ld_d8_reg_to_reg(H, dest),
            0x5 => Self::ld_d8_reg_to_reg(L, dest),
            0x6 => Self::ld_d8_from_hl_ptr(dest),
            0x7 => Self::ld_d8_reg_to_reg(A, dest),
            _ => panic!("SHOULDN'T EVER HAPPEN"),
        }
    }

    fn ld_d8_reg_to_reg(source: Registers, dest: Registers) -> Self {
        Instruction {
            op: Operation::LD,
            source: Target::HalfRegister(source),
            dest: Target::HalfRegister(dest),
            clock_cycles: 4,
            op_byte_len: 1,
        }
    }

    fn ld_d8_from_hl_ptr(dest: Registers) -> Self {
        Instruction {
            op: Operation::LD,
            source: Target::IndirectRegister(Registers::HL),
            dest: Target::HalfRegister(dest),
            clock_cycles: 8,
            op_byte_len: 1,
        }
    }

    fn jmp(condition: Condition) -> Self {
        Instruction {
            op: Operation::Jmp(condition),
            source: Target::ImmediateWord,
            dest: Target::None,
            clock_cycles: 16,
            op_byte_len: 1,
        }
    }

    fn pop(dest: Registers) -> Self {
        Instruction {
            op: Operation::Pop,
            source: Target::None,
            dest: Target::Register(dest),
            clock_cycles: 12,
            op_byte_len: 1,
        }
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
fn jmp(cpu: &mut CPU, source: Sized, condition: &Condition) -> Sized {
    match condition {
        Condition::None => cpu.pc = source.into(),
        Condition::NonZero if !cpu.get_flag('z') => cpu.pc = source.into(),
        Condition::NoCarry if !cpu.get_flag('c') => cpu.pc = source.into(),
        _ => {}
    }

    Sized::Zero
}

fn pop(bus: &Bus, cpu: &mut CPU) -> Sized {
    Sized::Word(cpu.pop_word_from_stack(bus))
}
