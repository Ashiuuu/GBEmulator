use crate::cpu;

pub struct Instruction<'a> {
    pub disassembly: &'a str,
    pub op_len: u16,
    pub clock_cycles: u8,
    pub execute: fn(&mut cpu::CPU),
}

impl Instruction<'_> {
    pub const SET: [Instruction<'static>; 32] = [
        Instruction {
            //0x00
            disassembly: "NOP",
            op_len: 1,
            clock_cycles: 0,
            execute: nop,
        },
        Instruction {
            //0x01
            disassembly: "LD BC d16",
            op_len: 3,
            clock_cycles: 3,
            execute: load_imm_bc,
        },
        Instruction {
            //0x02
            disassembly: "LD (BC) A",
            op_len: 1,
            clock_cycles: 2,
            execute: load_val_bc_ptr,
        },
        Instruction {
            //0x03
            disassembly: "INC BC",
            op_len: 1,
            clock_cycles: 2,
            execute: inc_bc,
        },
        Instruction {
            //0x04
            disassembly: "INC B",
            op_len: 1,
            clock_cycles: 2,
            execute: inc_b,
        },
        Instruction {
            //0x05
            disassembly: "DEC B",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_b,
        },
        Instruction {
            //0x06
            disassembly: "LD B d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_b,
        },
        Instruction {
            //0x07
            disassembly: "RLCA",
            op_len: 1,
            clock_cycles: 1,
            execute: rlca,
        },
        Instruction {
            //0x08
            disassembly: "LD (a16) SP",
            op_len: 3,
            clock_cycles: 5,
            execute: load_sp_imm_address,
        },
        Instruction {
            //0x09
            disassembly: "ADD HL BC",
            op_len: 1,
            clock_cycles: 2,
            execute: add_bc_to_hl,
        },
        Instruction {
            //0x0a
            disassembly: "LD A (BC)",
            op_len: 1,
            clock_cycles: 2,
            execute: load_bc_ptr_into_a,
        },
        Instruction {
            //0x0b
            disassembly: "DEC BC",
            op_len: 1,
            clock_cycles: 2,
            execute: dec_bc,
        },
        Instruction {
            //0x0c
            disassembly: "INC C",
            op_len: 1,
            clock_cycles: 1,
            execute: inc_c,
        },
        Instruction {
            //0x0d
            disassembly: "DEC C",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_c,
        },
        Instruction {
            //0x0e
            disassembly: "LD C d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_c,
        },
        Instruction {
            //0x0f
            disassembly: "RRCA",
            op_len: 1,
            clock_cycles: 1,
            execute: rrca,
        },
        Instruction {
            //0x1000 SPECIAL CASE!!
            disassembly: "STOP",
            op_len: 2,
            clock_cycles: 1,
            execute: stop,
        },
        Instruction {
            //0x11
            disassembly: "LD DE d16",
            op_len: 3,
            clock_cycles: 3,
            execute: load_imm_de,
        },
        Instruction {
            //0x12
            disassembly: "LD (DE) A",
            op_len: 1,
            clock_cycles: 2,
            execute: load_val_de_ptr,
        },
        Instruction {
            //0x13
            disassembly: "INC DE",
            op_len: 1,
            clock_cycles: 2,
            execute: inc_de,
        },
        Instruction {
            //0x14
            disassembly: "INC D",
            op_len: 1,
            clock_cycles: 1,
            execute: inc_d,
        },
        Instruction {
            //0x15
            disassembly: "DEC D",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_d,
        },
        Instruction {
            //0x16
            disassembly: "LD D d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_d,
        },
        Instruction {
            //0x17
            disassembly: "RLA",
            op_len: 1,
            clock_cycles: 1,
            execute: rla,
        },
        Instruction {
            //0x18
            disassembly: "JR s8",
            op_len: 2,
            clock_cycles: 3,
            execute: jr_s8,
        },
        Instruction {
            //0x19
            disassembly: "ADD HL DE",
            op_len: 1,
            clock_cycles: 2,
            execute: add_de_to_hl,
        },
        Instruction {
            //0x1a
            disassembly: "LD A (DE)",
            op_len: 1,
            clock_cycles: 2,
            execute: load_de_ptr_into_a,
        },
        Instruction {
            //0x1b
            disassembly: "DEC DE",
            op_len: 1,
            clock_cycles: 2,
            execute: dec_de,
        },
        Instruction {
            //0x1c
            disassembly: "INC E",
            op_len: 1,
            clock_cycles: 1,
            execute: inc_e,
        },
        Instruction {
            //0x1d
            disassembly: "DEC E",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_e,
        },
        Instruction {
            //0x1e
            disassembly: "LD E d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_e,
        },
        Instruction {
            //0x1f
            disassembly: "RRA",
            op_len: 1,
            clock_cycles: 1,
            execute: rra,
        },
    ];
}

// Instructions
// ======================================================
// 0x0X Instructions
// ======================================================
fn nop(_: &mut cpu::CPU) { //does nothing
}

fn load_imm_bc(cpu: &mut cpu::CPU) {
    // load 16 bits data into BC register
    let n1 = cpu.fetch_byte(cpu.pc);
    let n2 = cpu.fetch_byte(cpu.pc + 1);
    cpu.bc.low = n1;
    cpu.bc.high = n2;
}

fn load_val_bc_ptr(cpu: &mut cpu::CPU) {
    // load 8 bit data into address pointed by BC
    let val = cpu.fetch_byte(cpu.pc);
    cpu.set_byte(cpu.bc.get_combined(), val);
}

fn inc_bc(cpu: &mut cpu::CPU) {
    // increment 16 bits registry BC ; need to check for carry from low to high
    if cpu.bc.low == 255 {
        cpu.bc.high += 1;
    }
    cpu.bc.low += 1;
}

fn inc_b(cpu: &mut cpu::CPU) {
    // increment 8 bits register B
    cpu.bc.high += 1;
    if cpu.bc.high == 0 {
        cpu.set_flag('z'); // zero flag
    }
    if cpu.bc.high & 0b1111 == 0 {
        // if the first 4 bytes resulted in a carry
        cpu.set_flag('h'); // set half carry flag
    }
    cpu.set_flag('n'); // operation was addition
}

fn dec_b(cpu: &mut cpu::CPU) {
    // decrement 8 bits register B
    if cpu.bc.high & 0b1111 == 0 {
        cpu.set_flag('h');
    }
    cpu.bc.high -= 1;
    if cpu.bc.high == 0 {
        cpu.set_flag('z');
    }
    cpu.clear_flag('n');
}

fn load_imm_b(cpu: &mut cpu::CPU) {
    // load immediate value into 8 bits register B
    let op = cpu.fetch_byte(cpu.pc);
    cpu.bc.high = op;
}

fn rlca(cpu: &mut cpu::CPU) {
    // rotate A to the left with 7th bit going to 0th bit and carry flag
    cpu.clear_flag('z');
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.af.high & 0b10000000;
    cpu.af.high = cpu.af.high << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.af.high |= 1;
    } else {
        cpu.clear_flag('c');
    }
}

fn load_sp_imm_address(cpu: &mut cpu::CPU) {
    // store SP at spot pointer by immediate address
    let op1 = cpu.fetch_byte(cpu.pc);
    let op2 = cpu.fetch_byte(cpu.pc + 1);
    let address: u16 = (op1 as u16) << 8 + op2;

    cpu.set_word(address, cpu.sp);
}

fn add_bc_to_hl(cpu: &mut cpu::CPU) {
    // add BC to HL and store into HL
    let bc = cpu.bc.get_combined();
    let hl = cpu.hl.get_combined();
    let result = bc + hl;
    if result == 0 {
        cpu.set_flag('c');
    }
    if bc & 0xFF + hl & 0xFF > 255 {
        // checking half carry
        cpu.set_flag('h');
    }
    cpu.hl.set_word(result);
    cpu.set_flag('n');
}

fn load_bc_ptr_into_a(cpu: &mut cpu::CPU) {
    // load value pointed by BC into A
    let address = cpu.bc.get_combined();
    cpu.af.high = cpu.fetch_byte(address);
}

fn dec_bc(cpu: &mut cpu::CPU) {
    // decrement 16 bits register BC
    if cpu.bc.low == 0 {
        cpu.bc.high -= 1;
    }
    cpu.bc.low -= 1;
}

fn inc_c(cpu: &mut cpu::CPU) {
    // increment C register
    cpu.bc.low += 1;
    if cpu.bc.low == 0 {
        cpu.set_flag('z'); // zero flag
    }
    if cpu.bc.low & 0b1111 == 0 {
        // if the first 4 bytes resulted in a carry
        cpu.set_flag('h'); // set half carry flag
    }
    cpu.set_flag('n'); // operation was addition
}

fn dec_c(cpu: &mut cpu::CPU) {
    // decrement 8 bits register C
    if cpu.bc.low & 0b1111 == 0 {
        cpu.set_flag('h');
    }
    cpu.bc.low -= 1;
    if cpu.bc.low == 0 {
        cpu.set_flag('z');
    }
    cpu.clear_flag('n');
}

fn load_imm_c(cpu: &mut cpu::CPU) {
    // load immediate value into 8 bits register C
    let op = cpu.fetch_byte(cpu.pc);
    cpu.bc.low = op;
}

fn rrca(cpu: &mut cpu::CPU) {
    // rotate A to the left with 7th bit going to 0th bit and carry flag
    cpu.clear_flag('z');
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.af.high & 1;
    cpu.af.high = cpu.af.high >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.af.high |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
}

// ======================================================
// 0x1X Instructions
// ======================================================
fn stop(cpu: &mut cpu::CPU) {
    // stops the CPU ; only reverted by reset signal
    cpu.stopped = true;
}

fn load_imm_de(cpu: &mut cpu::CPU) {
    // load 16 bits data into DE register
    let n1 = cpu.fetch_byte(cpu.pc);
    let n2 = cpu.fetch_byte(cpu.pc + 1);
    cpu.de.low = n1;
    cpu.de.high = n2;
}

fn load_val_de_ptr(cpu: &mut cpu::CPU) {
    // load 8 bit data into address pointed by DE
    let val = cpu.fetch_byte(cpu.pc);
    cpu.set_byte(cpu.de.get_combined(), val);
}

fn inc_de(cpu: &mut cpu::CPU) {
    // increment 16 bits registry DE ; need to check for carry from low to high
    if cpu.de.low == 255 {
        cpu.de.high += 1;
    }
    cpu.de.low += 1;
}

fn inc_d(cpu: &mut cpu::CPU) {
    // increment 8 bits register D
    cpu.de.high += 1;
    if cpu.de.high == 0 {
        cpu.set_flag('z'); // zero flag
    }
    if cpu.de.high & 0b1111 == 0 {
        // if the first 4 bytes resulted in a carry
        cpu.set_flag('h'); // set half carry flag
    }
    cpu.set_flag('n'); // operation was addition
}

fn dec_d(cpu: &mut cpu::CPU) {
    // decrement 8 bits register D
    if cpu.de.high & 0b1111 == 0 {
        cpu.set_flag('h');
    }
    cpu.de.high -= 1;
    if cpu.de.high == 0 {
        cpu.set_flag('z');
    }
    cpu.clear_flag('n');
}

fn load_imm_d(cpu: &mut cpu::CPU) {
    // load immediate value into 8 bits register D
    let op = cpu.fetch_byte(cpu.pc);
    cpu.de.high = op;
}

fn rla(cpu: &mut cpu::CPU) {
    // rotates A register to the left through carry flag, and A0 gets previous carry flag
    let highest_bit = cpu.af.high & 0b1000000;
    cpu.af.high = cpu.af.high << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.af.high |= 1; // set bit 0 to 1
    } else {
        cpu.af.high &= 0b11111110; // else discard first bit just in case
    }
    if highest_bit == 1 {
        cpu.set_flag('c');
    } else {
        cpu.clear_flag('c');
    }
    cpu.clear_flag('h');
    cpu.clear_flag('z');
    cpu.clear_flag('n');
}

fn jr_s8(cpu: &mut cpu::CPU) {
    // jump relative to pc ; s8 is signed
    let op: i8 = cpu.fetch_byte(cpu.pc) as i8;
    if op < 0 {
        cpu.pc -= (-op) as u16;
    } else {
        cpu.pc += op as u16;
    }
}

fn add_de_to_hl(cpu: &mut cpu::CPU) {
    // add DE to HL and store into HL
    let de = cpu.de.get_combined();
    let hl = cpu.hl.get_combined();
    let result = de + hl;
    if result == 0 {
        cpu.set_flag('c');
    }
    if de & 0xFF + hl & 0xFF > 255 {
        // checking half carry
        cpu.set_flag('h');
    }
    cpu.hl.set_word(result);
    cpu.set_flag('n');
}

fn load_de_ptr_into_a(cpu: &mut cpu::CPU) {
    // load value pointed by DE into A
    let address = cpu.de.get_combined();
    cpu.af.high = cpu.fetch_byte(address);
}

fn dec_de(cpu: &mut cpu::CPU) {
    // decrement 16 bits register DE
    if cpu.de.low == 0 {
        cpu.de.high -= 1;
    }
    cpu.de.low -= 1;
}

fn inc_e(cpu: &mut cpu::CPU) {
    // increment E register
    cpu.de.low += 1;
    if cpu.de.low == 0 {
        cpu.set_flag('z'); // zero flag
    }
    if cpu.de.low & 0b1111 == 0 {
        // if the first 4 bytes resulted in a carry
        cpu.set_flag('h'); // set half carry flag
    }
    cpu.set_flag('n'); // operation was addition
}

fn dec_e(cpu: &mut cpu::CPU) {
    // decrement 8 bits register E
    if cpu.de.low & 0b1111 == 0 {
        cpu.set_flag('h');
    }
    cpu.de.low -= 1;
    if cpu.de.low == 0 {
        cpu.set_flag('z');
    }
    cpu.clear_flag('n');
}

fn load_imm_e(cpu: &mut cpu::CPU) {
    // load immediate value into 8 bits register E
    let op = cpu.fetch_byte(cpu.pc);
    cpu.de.low = op;
}

fn rra(cpu: &mut cpu::CPU) {
    // rotates A register to the left through carry flag, and A0 gets previous carry flag
    let lowest_bit = cpu.af.high & 1;
    cpu.af.high = cpu.af.high >> 1;
    if cpu.extract_flag('c') == true {
        cpu.af.high |= 0b10000000;
    } else {
        cpu.af.high &= 0b01111111;
    }
    if lowest_bit == 1 {
        cpu.set_flag('c');
    } else {
        cpu.clear_flag('c');
    }
    cpu.clear_flag('z');
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

// ======================================================
// 0x2X Instructions
// ======================================================
