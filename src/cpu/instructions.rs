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

    IncByte,
    IncWord,

    CPL,

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
            Self::CPL => cpl(cpu),
            Self::IncByte => inc(cpu, source, true),
            Self::IncWord => inc(cpu, source, false),

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
                Self::CPL => "CPL",
                Self::IncByte | Self::IncWord => "INC",

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
    opcode: u8,
    op: Operation,
    source: Target,
    dest: Target,
    clock_cycles: u8,
    op_byte_len: u16,
}

impl Instruction {
    pub fn fetch_new(bus: &Bus, cpu: &CPU) -> Self {
        Self::from_opcode(bus.fetch_byte(cpu.pc))
    }

    // Execute an instruction. Returns the number of clock cycles to wait
    pub fn execute(self, bus: &mut Bus, cpu: &mut CPU) -> u8 {
        cpu.pc += self.op_byte_len;
        let mut instruction_length = 0;
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
        let mut result = match opcode {
            0x00 => Instruction {
                opcode: 0,
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

            0x06 => Self::ld_d8_imm(B),
            0x0E => Self::ld_d8_imm(C),
            0x16 => Self::ld_d8_imm(D),
            0x1E => Self::ld_d8_imm(E),
            0x26 => Self::ld_d8_imm(H),
            0x2E => Self::ld_d8_imm(L),

            0x40..=0x47 => Self::ld_d8_reg_to_reg_match(B, opcode),
            0x48..=0x4F => Self::ld_d8_reg_to_reg_match(C, opcode),
            0x50..=0x57 => Self::ld_d8_reg_to_reg_match(D, opcode),
            0x58..=0x5F => Self::ld_d8_reg_to_reg_match(E, opcode),
            0x60..=0x67 => Self::ld_d8_reg_to_reg_match(H, opcode),
            0x68..=0x6F => Self::ld_d8_reg_to_reg_match(L, opcode),

            0x03 => Self::inc_reg16(BC),
            0x13 => Self::inc_reg16(DE),
            0x23 => Self::inc_reg16(HL),
            0x33 => Self::inc_reg16(SP),

            0x04 => Self::inc_reg8(B),
            0x0C => Self::inc_reg8(C),
            0x14 => Self::inc_reg8(D),
            0x1C => Self::inc_reg8(E),
            0x24 => Self::inc_reg8(H),
            0x2C => Self::inc_reg8(L),

            0x2F => Self::cpl(),

            0xC1 => Self::pop(BC),
            0xD1 => Self::pop(DE),
            0xE1 => Self::pop(HL),
            0xF1 => Self::pop(AF),

            0xC2 => Self::jmp(Condition::NonZero),
            0xC3 => Self::jmp(Condition::None),
            0xD2 => Self::jmp(Condition::NoCarry),

            _ => panic!("Opcode non implemented: {:#04x}", opcode),
        };

        result.opcode = opcode;
        result
    }

    // OPCODE HELPERS
    fn ld_d16(reg: Registers) -> Self {
        Instruction {
            opcode: 0,
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
            opcode: 0,
            op: Operation::LD,
            source: Target::HalfRegister(source),
            dest: Target::HalfRegister(dest),
            clock_cycles: 4,
            op_byte_len: 1,
        }
    }

    fn ld_d8_from_hl_ptr(dest: Registers) -> Self {
        Instruction {
            opcode: 0,
            op: Operation::LD,
            source: Target::IndirectRegister(Registers::HL),
            dest: Target::HalfRegister(dest),
            clock_cycles: 8,
            op_byte_len: 1,
        }
    }

    fn ld_d8_imm(dest: Registers) -> Self {
        Instruction {
            opcode: 0,
            op: Operation::LD,
            source: Target::ImmediateByte,
            dest: Target::HalfRegister(dest),
            clock_cycles: 8,
            op_byte_len: 1,
        }
    }

    fn jmp(condition: Condition) -> Self {
        Instruction {
            opcode: 0,
            op: Operation::Jmp(condition),
            source: Target::ImmediateWord,
            dest: Target::None,
            clock_cycles: 16,
            op_byte_len: 1,
        }
    }

    fn pop(dest: Registers) -> Self {
        Instruction {
            opcode: 0,
            op: Operation::Pop,
            source: Target::None,
            dest: Target::Register(dest),
            clock_cycles: 12,
            op_byte_len: 1,
        }
    }

    fn cpl() -> Self {
        Instruction {
            opcode: 0,
            op: Operation::CPL,
            source: Target::None,
            dest: Target::None,
            clock_cycles: 4,
            op_byte_len: 1,
        }
    }

    fn inc_reg16(reg: Registers) -> Self {
        Instruction {
            opcode: 0,
            op: Operation::IncWord,
            source: Target::Register(reg),
            dest: Target::Register(reg),
            clock_cycles: 8,
            op_byte_len: 1,
        }
    }

    fn inc_reg8(reg: Registers) -> Self {
        Instruction {
            opcode: 0,
            op: Operation::IncByte,
            source: Target::Register(reg),
            dest: Target::Register(reg),
            clock_cycles: 4,
            op_byte_len: 1,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "[{:#04x}] {} {} {}",
            self.opcode, self.op, self.dest, self.source
        )
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            opcode: 0,
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

fn cpl(cpu: &mut CPU) -> Sized {
    cpu.af.a = !cpu.af.a;
    cpu.set_flag('h');
    cpu.set_flag('n');

    Sized::Zero
}

fn inc(cpu: &mut CPU, source: Sized, should_set_flags: bool) -> Sized {
    let result = match source {
        Sized::Word(value) => Sized::Word(value.wrapping_add(1)),
        Sized::Byte(value) => Sized::Byte(value.wrapping_add(1)),
        Sized::Zero => panic!("Calling inc on zero sized value"),
    };

    if should_set_flags {
        cpu.clear_flag('n');
        cpu.update_flag('z', result.is_value_zero());
        cpu.update_flag('h', result.check_value_for_half_carry());
    }

    result
}
