use crate::cpu;
use crate::bus;

pub struct Instruction<'a> {
    pub disassembly: &'a str,
    pub op_len: u16,
    pub clock_cycles: u8,
    pub execute: fn(&mut cpu::CPU, &mut bus::Bus),
}

impl Instruction<'_> {
    pub const SET: [Instruction<'static>; 256] = [
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
        Instruction {
            //0x20
            disassembly: "JR NZ s8",
            op_len: 2,
            clock_cycles: 3,
            execute: jr_nz_s8,
        },
        Instruction {
            //0x21
            disassembly: "LD HL d16",
            op_len: 3,
            clock_cycles: 3,
            execute: load_imm_hl,
        },
        Instruction {
            //0x22
            disassembly: "LD (HL++) A",
            op_len: 1,
            clock_cycles: 2,
            execute: load_val_hl_ptr,
        },
        Instruction {
            //0x23
            disassembly: "INC HL",
            op_len: 1,
            clock_cycles: 2,
            execute: inc_hl,
        },
        Instruction {
            //0x24
            disassembly: "INC H",
            op_len: 1,
            clock_cycles: 1,
            execute: inc_h,
        },
        Instruction {
            //0x25
            disassembly: "DEC H",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_h,
        },
        Instruction {
            //x026
            disassembly: "LD H d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_h,
        },
        Instruction {
            //0x27
            disassembly: "DAA",
            op_len: 1,
            clock_cycles: 1,
            execute: daa,
        },
        Instruction {
            //0x28
            disassembly: "JR Z s8",
            op_len: 2,
            clock_cycles: 3,
            execute: jr_z_s8,
        },
        Instruction {
            //0x29
            disassembly: "ADD HL HL",
            op_len: 1,
            clock_cycles: 2,
            execute: add_hl_to_hl,
        },
        Instruction {
            //0x2a
            disassembly: "LD A (HL++)",
            op_len: 1,
            clock_cycles: 2,
            execute: load_hl_ptr_into_a,
        },
        Instruction {
            //0x2b
            disassembly: "DEC HL",
            op_len: 1,
            clock_cycles: 2,
            execute: dec_hl,
        },
        Instruction {
            //0x2c
            disassembly: "INC L",
            op_len: 1,
            clock_cycles: 1,
            execute: inc_l,
        },
        Instruction {
            //0x2d
            disassembly: "DEC L",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_l,
        },
        Instruction {
            //0x2e
            disassembly: "LD L d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_l,
        },
        Instruction {
            //0x2f
            disassembly: "CPL",
            op_len: 1,
            clock_cycles: 1,
            execute: cpl,
        },
        Instruction {
            //0x30
            disassembly: "JR NC s8",
            op_len: 2,
            clock_cycles: 3,
            execute: jr_nc_s8,
        },
        Instruction {
            //0x31
            disassembly: "LD SP d16",
            op_len: 3,
            clock_cycles: 3,
            execute: load_imm_sp,
        },
        Instruction {
            //0x32
            disassembly: "LD (HL--) A",
            op_len: 1,
            clock_cycles: 2,
            execute: load_val_hl_ptr_dec,
        },
        Instruction {
            //0x33
            disassembly: "INC SP",
            op_len: 1,
            clock_cycles: 2,
            execute: inc_sp,
        },
        Instruction {
            //0x34
            disassembly: "INC (HL)",
            op_len: 1,
            clock_cycles: 3,
            execute: inc_hl_ptr,
        },
        Instruction {
            //0x35
            disassembly: "DEC (HL)",
            op_len: 1,
            clock_cycles: 3,
            execute: dec_hl_ptr,
        },
        Instruction {
            //0x36
            disassembly: "LD (HL) d8",
            op_len: 2,
            clock_cycles: 3,
            execute: load_d8_into_hl_ptr,
        },
        Instruction {
            //0x37
            disassembly: "SCF",
            op_len: 1,
            clock_cycles: 1,
            execute: set_carry_flag,
        },
        Instruction {
            //0x38
            disassembly: "JR C s8",
            op_len: 2,
            clock_cycles: 3,
            execute: jr_c_s8,
        },
        Instruction {
            //0x39
            disassembly: "ADD HL SP",
            op_len: 1,
            clock_cycles: 2,
            execute: add_sp_to_hl,
        },
        Instruction {
            //0x3a
            disassembly: "LD A (HL--)",
            op_len: 1,
            clock_cycles: 2,
            execute: load_hl_ptr_into_a_dec,
        },
        Instruction {
            //0x3b
            disassembly: "DEC SP",
            op_len: 1,
            clock_cycles: 2,
            execute: dec_sp,
        },
        Instruction {
            //0x3c
            disassembly: "INC A",
            op_len: 1,
            clock_cycles: 1,
            execute: inc_a,
        },
        Instruction {
            //0x3d
            disassembly: "DEC A",
            op_len: 1,
            clock_cycles: 1,
            execute: dec_a,
        },
        Instruction {
            //0x3e
            disassembly: "LD A d8",
            op_len: 2,
            clock_cycles: 2,
            execute: load_imm_a,
        },
        Instruction {
            //0x3f
            disassembly: "CCF",
            op_len: 1,
            clock_cycles: 1,
            execute: ccf,
        },
        Instruction {
            //0x40
            disassembly: "LD B B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_b,
        },
        Instruction {
            //0x41
            disassembly: "LD B C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_c,
        },
        Instruction {
            //0x42
            disassembly: "LD B D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_d,
        },
        Instruction {
            //0x43
            disassembly: "LD B E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_e,
        },
        Instruction {
            //0x44
            disassembly: "LD B H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_h,
        },
        Instruction {
            //0x45
            disassembly: "LD B L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_l,
        },
        Instruction {
            //0x46
            disassembly: "LD B (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_hl_ptr,
        },
        Instruction {
            //0x47
            disassembly: "LD B A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_b_a,
        },
        Instruction {
            //0x48
            disassembly: "LD C B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_b,
        },
        Instruction {
            //0x49
            disassembly: "LD C C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_c,
        },
        Instruction {
            //0x4a
            disassembly: "LD C D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_d,
        },
        Instruction {
            //0x4b
            disassembly: "LD C E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_e,
        },
        Instruction {
            //0x4c
            disassembly: "LD C H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_h,
        },
        Instruction {
            //0x4d
            disassembly: "LD C L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_l,
        },
        Instruction {
            //0x4e
            disassembly: "LD C (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_hl_ptr,
        },
        Instruction {
            //0x4f
            disassembly: "LD C A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_c_a,
        },
        Instruction {
            //0x50
            disassembly: "LD D B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_b,
        },
        Instruction {
            //0x51
            disassembly: "LD D C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_c,
        },
        Instruction {
            //0x52
            disassembly: "LD D D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_d,
        },
        Instruction {
            //0x53
            disassembly: "LD D E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_e,
        },
        Instruction {
            //0x54
            disassembly: "LD D H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_h,
        },
        Instruction {
            //0x55
            disassembly: "LD D L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_l,
        },
        Instruction {
            //0x56
            disassembly: "LD D (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_hl_ptr,
        },
        Instruction {
            //0x57
            disassembly: "LD D A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_d_a,
        },
        Instruction {
            //0x58
            disassembly: "LD E B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_b,
        },
        Instruction {
            //0x59
            disassembly: "LD E C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_c,
        },
        Instruction {
            //0x5a
            disassembly: "LD E D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_d,
        },
        Instruction {
            //0x5b
            disassembly: "LD E E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_e,
        },
        Instruction {
            //0x5c
            disassembly: "LD E H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_h,
        },
        Instruction {
            //0x5d
            disassembly: "LD E L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_l,
        },
        Instruction {
            //0x5e
            disassembly: "LD E (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_hl_ptr,
        },
        Instruction {
            //0x5f
            disassembly: "LD E A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_e_a,
        },
        Instruction {
            //0x60
            disassembly: "LD H B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_b,
        },
        Instruction {
            //0x61
            disassembly: "LD H C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_c,
        },
        Instruction {
            //0x62
            disassembly: "LD H D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_d,
        },
        Instruction {
            //0x63
            disassembly: "LD H E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_e,
        },
        Instruction {
            //0x64
            disassembly: "LD H H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_h,
        },
        Instruction {
            //0x65
            disassembly: "LD H L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_l,
        },
        Instruction {
            //0x66
            disassembly: "LD H (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_hl_ptr,
        },
        Instruction {
            //0x67
            disassembly: "LD H A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_h_a,
        },
        Instruction {
            //0x68
            disassembly: "LD L B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_b,
        },
        Instruction {
            //0x69
            disassembly: "LD L C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_c,
        },
        Instruction {
            //0x6a
            disassembly: "LD L D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_d,
        },
        Instruction {
            //0x6b
            disassembly: "LD L E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_e,
        },
        Instruction {
            //0x6c
            disassembly: "LD L H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_h,
        },
        Instruction {
            //0x6d
            disassembly: "LD L L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_l,
        },
        Instruction {
            //0x6e
            disassembly: "LD L (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_hl_ptr,
        },
        Instruction {
            //0x6f
            disassembly: "LD L A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_l_a,
        },
        Instruction {
            //0x70
            disassembly: "LD (HL) B",
            op_len: 1,
            clock_cycles: 2,
            execute: load_b_into_hl_ptr,
        },
        Instruction {
            //0x71
            disassembly: "LD (HL) C",
            op_len: 1,
            clock_cycles: 2,
            execute: load_c_into_hl_ptr,
        },
        Instruction {
            //0x72
            disassembly: "LD (HL) D",
            op_len: 1,
            clock_cycles: 2,
            execute: load_d_into_hl_ptr,
        },
        Instruction {
            //0x73
            disassembly: "LD (HL) E",
            op_len: 1,
            clock_cycles: 2,
            execute: load_e_into_hl_ptr,
        },
        Instruction {
            //0x74
            disassembly: "LD (HL) H",
            op_len: 1,
            clock_cycles: 2,
            execute: load_h_into_hl_ptr,
        },
        Instruction {
            //0x75
            disassembly: "LD (HL) L",
            op_len: 1,
            clock_cycles: 2,
            execute: load_l_into_hl_ptr,
        },
        Instruction {
            //0x76
            disassembly: "HALT",
            op_len: 1,
            clock_cycles: 1,
            execute: halt,
        },
        Instruction {
            //0x77
            disassembly: "LD (HL) A",
            op_len: 1,
            clock_cycles: 2,
            execute: load_a_into_hl_ptr,
        },
        Instruction {
            //0x78
            disassembly: "LD A B",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_b,
        },
        Instruction {
            //0x79
            disassembly: "LD A C",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_c,
        },
        Instruction {
            //0x7a
            disassembly: "LD A D",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_d,
        },
        Instruction {
            //0x7b
            disassembly: "LD A E",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_e,
        },
        Instruction {
            //0x7c
            disassembly: "LD A H",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_h,
        },
        Instruction {
            //0x7d
            disassembly: "LD A L",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_l,
        },
        Instruction {
            //0x7e
            disassembly: "LD A (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_hl_ptr,
        },
        Instruction {
            //0x7f
            disassembly: "LD A A",
            op_len: 1,
            clock_cycles: 1,
            execute: ld_a_a,
        },
        Instruction {
            //0x80
            disassembly: "ADD A B",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_b,
        },
        Instruction {
            //0x81
            disassembly: "ADD A C",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_c,
        },
        Instruction {
            //0x82
            disassembly: "ADD A D",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_d,
        },
        Instruction {
            //0x83
            disassembly: "ADD A E",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_e,
        },
        Instruction {
            //0x84
            disassembly: "ADD A H",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_h,
        },
        Instruction {
            //0x85
            disassembly: "ADD A L",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_l,
        },
        Instruction {
            //0x86
            disassembly: "ADD A (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: add_hl_ptr_to_a,
        },
        Instruction {
            //0x87
            disassembly: "ADD A A",
            op_len: 1,
            clock_cycles: 1,
            execute: add_a_a,
        },
        Instruction {
            //0x88
            disassembly: "ADC A B",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_b,
        },
        Instruction {
            //0x89
            disassembly: "ADC A C",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_c,
        },
        Instruction {
            //0x8a
            disassembly: "ADC A D",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_d,
        },
        Instruction {
            //0x8b
            disassembly: "ADC A E",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_e,
        },
        Instruction {
            //0x8c
            disassembly: "ADC A H",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_h,
        },
        Instruction {
            //0x8d
            disassembly: "ADC A L",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_l,
        },
        Instruction {
            //0x8e
            disassembly: "ADC A (HL)",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_hl_ptr_into_a,
        },
        Instruction {
            //0x8f
            disassembly: "ADC A A",
            op_len: 1,
            clock_cycles: 1,
            execute: adc_a_a,
        },
        Instruction {
            //0x90
            disassembly: "SUB B",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_b,
        },
        Instruction {
            //0x91
            disassembly: "SUB C",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_c,
        },
        Instruction {
            //0x92
            disassembly: "SUB D",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_d,
        },
        Instruction {
            //0x93
            disassembly: "SUB E",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_e,
        },
        Instruction {
            //0x94
            disassembly: "SUB H",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_h,
        },
        Instruction {
            //0x95
            disassembly: "SUB L",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_l,
        },
        Instruction {
            //0x96
            disassembly: "SUB (HL)",
            op_len: 1,
            clock_cycles: 2,
            execute: sub_hl_ptr,
        },
        Instruction {
            //0x97
            disassembly: "SUB A",
            op_len: 1,
            clock_cycles: 1,
            execute: sub_a,
        },
        Instruction {
            //0x98
            disassembly: "SBC A B",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_b,
        },
        Instruction {
            //0x99
            disassembly: "SBC A C",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_c,
        },
        Instruction {
            //0x9a
            disassembly: "SBC A D",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_d,
        },
        Instruction {
            //0x9b
            disassembly: "SBC A E",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_e,
        },
        Instruction {
            //0x9c
            disassembly: "SBC A H",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_h,
        },
        Instruction {
            //0x9d
            disassembly: "SBC A L",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_l,
        },
        Instruction {
            //0x9e
            disassembly: "SBC A (HL)",
            op_len: 1,
            clock_cycles: 2,
            execute: sbc_hl_ptr,
        },
        Instruction {
            //0x9f
            disassembly: "SBC A A",
            op_len: 1,
            clock_cycles: 1,
            execute: sbc_a_a,
        },
        Instruction {
            //0xa0
            disassembly: "AND B",
            op_len: 1,
            clock_cycles: 1,
            execute: and_b,
        },
        Instruction {
            //0xa1
            disassembly: "AND C",
            op_len: 1,
            clock_cycles: 1,
            execute: and_c,
        },
        Instruction {
            //0xa2
            disassembly: "AND D",
            op_len: 1,
            clock_cycles: 1,
            execute: and_d,
        },
        Instruction {
            //0xa3
            disassembly: "AND E",
            op_len: 1,
            clock_cycles: 1,
            execute: and_e,
        },
        Instruction {
            //0xa4
            disassembly: "AND H",
            op_len: 1,
            clock_cycles: 1,
            execute: and_h,
        },
        Instruction {
            //0xa5
            disassembly: "AND L",
            op_len: 1,
            clock_cycles: 1,
            execute: and_l,
        },
        Instruction {
            //0xa6
            disassembly: "AND (HL)",
            op_len: 1,
            clock_cycles: 2,
            execute: and_hl_ptr,
        },
        Instruction {
            //0xa7
            disassembly: "AND A",
            op_len: 1,
            clock_cycles: 1,
            execute: and_a,
        },
        Instruction {
            //0xa8
            disassembly: "XOR B",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_b,
        },
        Instruction {
            //0xa9
            disassembly: "XOR C",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_c,
        },
        Instruction {
            //0xaa
            disassembly: "XOR D",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_d,
        },
        Instruction {
            //0xab
            disassembly: "XOR E",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_e,
        },
        Instruction {
            //0xac
            disassembly: "XOR H",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_h,
        },
        Instruction {
            //0xad
            disassembly: "XOR L",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_l,
        },
        Instruction {
            //0xae
            disassembly: "XOR (HL)",
            op_len: 1,
            clock_cycles: 2,
            execute: xor_hl_ptr,
        },
        Instruction {
            //0xaf
            disassembly: "XOR A",
            op_len: 1,
            clock_cycles: 1,
            execute: xor_a,
        },
        Instruction {
            //0xb0
            disassembly: "OR B",
            op_len: 1,
            clock_cycles: 1,
            execute: or_b,
        },
        Instruction {
            //0xb1
            disassembly: "OR C",
            op_len: 1,
            clock_cycles: 1,
            execute: or_c,
        },
        Instruction {
            //0xb2
            disassembly: "OR D",
            op_len: 1,
            clock_cycles: 1,
            execute: or_d,
        },
        Instruction {
            //0xb3
            disassembly: "OR E",
            op_len: 1,
            clock_cycles: 1,
            execute: or_e,
        },
        Instruction {
            //0xb4
            disassembly: "OR H",
            op_len: 1,
            clock_cycles: 1,
            execute: or_h,
        },
        Instruction {
            //0xb5
            disassembly: "OR L",
            op_len: 1,
            clock_cycles: 1,
            execute: or_l,
        },
        Instruction {
            //0xb6
            disassembly: "OR (HL)",
            op_len: 1,
            clock_cycles: 2,
            execute: or_hl_ptr,
        },
        Instruction {
            //0xb7
            disassembly: "OR A",
            op_len: 1,
            clock_cycles: 1,
            execute: or_a,
        },
        Instruction {
            //0xb8
            disassembly: "CP B",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_b,
        },
        Instruction {
            //0xb9
            disassembly: "CP C",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_c,
        },
        Instruction {
            //0xba
            disassembly: "CP D",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_d,
        },
        Instruction {
            //0xbb
            disassembly: "CP E",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_e,
        },
        Instruction {
            //0xbc
            disassembly: "CP H",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_h,
        },
        Instruction {
            //0xbd
            disassembly: "CP L",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_l,
        },
        Instruction {
            //0xbe
            disassembly: "CP (HL)",
            op_len: 1,
            clock_cycles: 2,
            execute: cp_hl_ptr,
        },
        Instruction {
            //0xbf
            disassembly: "CP A",
            op_len: 1,
            clock_cycles: 1,
            execute: cp_a,
        },
        Instruction {
            //0xc0
            disassembly: "RET NZ",
            op_len: 1,
            clock_cycles: 5,
            execute: ret_nz,
        },
        Instruction {
            //0xc1
            disassembly: "POP BC",
            op_len: 1,
            clock_cycles: 3,
            execute: pop_bc,
        },
        Instruction {
            //0xc2
            disassembly: "JP NZ a16",
            op_len: 3,
            clock_cycles: 4,
            execute: jp_nz_a16,
        },
        Instruction {
            //0xc3
            disassembly: "JP a16",
            op_len: 3,
            clock_cycles: 4,
            execute: jp_a16,
        },
        Instruction {
            //0xc4
            disassembly: "CALL NZ a16",
            op_len: 3,
            clock_cycles: 6,
            execute: call_nz_a16,
        },
        Instruction {
            //0xc5
            disassembly: "PUSH BC",
            op_len: 1,
            clock_cycles: 4,
            execute: push_bc,
        },
        Instruction {
            //0xc6
            disassembly: "ADD A d8",
            op_len: 2,
            clock_cycles: 2,
            execute: add_a_d8,
        },
        Instruction {
            //0xc7
            disassembly: "RST 0",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_0,
        },
        Instruction {
            //0xc8
            disassembly: "RET Z",
            op_len: 1,
            clock_cycles: 5,
            execute: ret_z,
        },
        Instruction {
            //0xc9
            disassembly: "RET",
            op_len: 1,
            clock_cycles: 4,
            execute: ret,
        },
        Instruction {
            //0xca
            disassembly: "JP Z a16",
            op_len: 3,
            clock_cycles: 4,
            execute: jp_z_a16,
        },
        Instruction {
            //0xcb
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: special_cb,
        },
        Instruction {
            //0xcc
            disassembly: "CALL Z a16",
            op_len: 3,
            clock_cycles: 6,
            execute: call_z_a16,
        },
        Instruction {
            //0xcd
            disassembly: "CALL a16",
            op_len: 3,
            clock_cycles: 6,
            execute: call_a16,
        },
        Instruction {
            //0xce
            disassembly: "ADC A d8",
            op_len: 2,
            clock_cycles: 2,
            execute: adc_a_d8,
        },
        Instruction {
            //0xcf
            disassembly: "RST 1",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_1,
        },
        Instruction {
            //0xd0
            disassembly: "RET NC",
            op_len: 1,
            clock_cycles: 5,
            execute: ret_nc,
        },
        Instruction {
            //0xd1
            disassembly: "POP DE",
            op_len: 1,
            clock_cycles: 3,
            execute: pop_de,
        },
        Instruction {
            //0xd2
            disassembly: "JP NC a16",
            op_len: 3,
            clock_cycles: 4,
            execute: jp_nc_a16,
        },
        Instruction {
            //0xd3
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xd4
            disassembly: "CALL NC a16",
            op_len: 3,
            clock_cycles: 6,
            execute: call_nc_a16
        },
        Instruction {
            //0xd5
            disassembly: "PUSH DE",
            op_len: 1,
            clock_cycles: 4,
            execute: push_de,
        },
        Instruction {
            //0xd6
            disassembly: "SUB d8",
            op_len: 2,
            clock_cycles: 2,
            execute: sub_d8,
        },
        Instruction {
            //0xd7
            disassembly: "RST 2",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_2,
        },
        Instruction {
            //0xd8
            disassembly: "RET C",
            op_len: 1,
            clock_cycles: 5,
            execute: ret_c,
        },
        Instruction {
            //0xd9
            disassembly: "RETI",
            op_len: 1,
            clock_cycles: 4,
            execute: ret_i,
        },
        Instruction {
            //0xda
            disassembly: "JP C a16",
            op_len: 3,
            clock_cycles: 4,
            execute: jp_c_a16,
        },
        Instruction {
            //0xdb
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xdc
            disassembly: "CALL C a16",
            op_len: 3,
            clock_cycles: 6,
            execute: call_c_a16,
        },
        Instruction {
            //0xdd
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xde
            disassembly: "SBC A d8",
            op_len: 2,
            clock_cycles: 2,
            execute: sbc_a_d8,
        },
        Instruction {
            //0xdf
            disassembly: "RST 3",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_3,
        },
        Instruction {
            //0xe0
            disassembly: "LD (a8) A",
            op_len: 2,
            clock_cycles: 3,
            execute: ld_a_to_ffa8,
        },
        Instruction {
            //0xe1
            disassembly: "POP HL",
            op_len: 1,
            clock_cycles: 3,
            execute: pop_hl,
        },
        Instruction {
            //0xe2
            disassembly: "LD (C) A",
            op_len: 1,
            clock_cycles: 2,
            execute: ld_a_to_ffc,
        },
        Instruction {
            //0xe3
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xe4
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xe5
            disassembly: "PUSH HL",
            op_len: 1,
            clock_cycles: 4,
            execute: push_hl,
        },
        Instruction {
            //0xe6
            disassembly: "AND d8",
            op_len: 2,
            clock_cycles: 2,
            execute: and_d8,
        },
        Instruction {
            //0xe7
            disassembly: "RST 4",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_4,
        },
        Instruction {
            //0xe8
            disassembly: "ADD SP s8",
            op_len: 2,
            clock_cycles: 4,
            execute: add_sp_s8,
        },
        Instruction {
            //0xe9
            disassembly: "JP HL",
            op_len: 1,
            clock_cycles: 1,
            execute: jp_hl,
        },
        Instruction {
            //0xea
            disassembly: "LD (a16) A",
            op_len: 3,
            clock_cycles: 4,
            execute: ld_a16_a,
        },
        Instruction {
            //0xeb
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xec
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xed
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xee
            disassembly: "XOR d8",
            op_len: 2,
            clock_cycles: 2,
            execute: xor_d8,
        },
        Instruction {
            //0xef
            disassembly: "RST 5",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_5,
        },
        Instruction {
            //0xf0
            disassembly: "LD A (a8)",
            op_len: 2,
            clock_cycles: 3,
            execute: ld_ffa8_to_a,
        },
        Instruction {
            //0xf1
            disassembly: "POP AF",
            op_len: 1,
            clock_cycles: 3,
            execute: pop_af,
        },
        Instruction {
            //0xf2
            disassembly: "LD A (C)",
            op_len: 1,
            clock_cycles: 2,
            execute: ld_ffc_to_a,
        },
        Instruction {
            //0xf3
            disassembly: "DI",
            op_len: 1,
            clock_cycles: 1,
            execute: di,
        },
        Instruction {
            //0xf4
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xf5
            disassembly: "PUSH AF",
            op_len: 1,
            clock_cycles: 4,
            execute: push_af,
        },
        Instruction {
            //0xf6
            disassembly: "OR d8",
            op_len: 2,
            clock_cycles: 2,
            execute: or_d8,
        },
        Instruction {
            //0xf7
            disassembly: "RST 6",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_6,
        },
        Instruction {
            //0xf8
            disassembly: "LD HL SP+s8",
            op_len: 2,
            clock_cycles: 3,
            execute: ld_sp_s8_to_hl,
        },
        Instruction {
            //0xf9
            disassembly: "LD SP HL",
            op_len: 1,
            clock_cycles: 2,
            execute: ld_sp_hl,
        },
        Instruction {
            //0xfa
            disassembly: "LD A (a16)",
            op_len: 3,
            clock_cycles: 4,
            execute: ld_a_a16,
        },
        Instruction {
            //0xfb
            disassembly: "EI",
            op_len: 1,
            clock_cycles: 1,
            execute: ei,
        },
        Instruction {
            //0xfc
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },Instruction {
            //0xfd
            disassembly: "NOT VALID",
            op_len: 0,
            clock_cycles: 0,
            execute: unimplemented_opcode,
        },
        Instruction {
            //0xfe
            disassembly: "CP d8",
            op_len: 2,
            clock_cycles: 2,
            execute: cp_d8,
        },
        Instruction {
            //0xff
            disassembly: "RST 7",
            op_len: 1,
            clock_cycles: 4,
            execute: rst_7,
        },
    ];
}

fn unimplemented_opcode(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // for opcodes that do nothing
    panic!("Unimplemetend Op code reached ! {}", bus.fetch_byte(cpu.pc - 1));
}

// Instructions
// ======================================================
// 0x0X Instructions
// ======================================================
fn nop(_: &mut cpu::CPU, _: &mut bus::Bus) { //does nothing
}

fn load_imm_bc(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load 16 bits data into BC register
    let n1 = bus.fetch_byte(cpu.pc);
    let n2 = bus.fetch_byte(cpu.pc + 1);
    cpu.bc.low = n1;
    cpu.bc.high = n2;
}

fn load_val_bc_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load 8 bit data into address pointed by BC
    bus.set_byte(cpu.bc.get_combined(), cpu.af.high);
}

fn inc_bc(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment 16 bits registry BC ; need to check for carry from low to high
    if cpu.bc.low == 255 {
        cpu.bc.high = cpu.bc.high.wrapping_add(1);
    }
    cpu.bc.low = cpu.bc.low.wrapping_add(1);
}

fn inc_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment 8 bits register B
    cpu.bc.high = cpu.bc.high.wrapping_add(1);
    cpu.update_flag('z', cpu.bc.high == 0);
    cpu.update_flag('h', cpu.bc.high & 0b1111 == 0);
    cpu.clear_flag('n'); // operation was addition
}

fn dec_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 8 bits register B
    cpu.bc.high = cpu.bc.high.wrapping_sub(1);
    cpu.update_flag('h', cpu.bc.high & 0b1111 == 0);
    cpu.update_flag('z', cpu.bc.high == 0);
    cpu.set_flag('n');
}

fn load_imm_b(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load immediate value into 8 bits register B
    let op = bus.fetch_byte(cpu.pc);
    cpu.bc.high = op;
}

fn rlca(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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

fn load_sp_imm_address(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // store SP at spot pointer by immediate address
    let op1 = bus.fetch_byte(cpu.pc);
    let op2 = bus.fetch_byte(cpu.pc + 1);
    let address: u16 = ((op2 as u16) << 8) + (op1 as u16);

    bus.set_word(address, cpu.sp);
}

fn add_bc_to_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // add BC to HL and store into HL
    let bc = cpu.bc.get_combined();
    let hl = cpu.hl.get_combined();
    let result = bc.wrapping_add(hl);
    cpu.update_flag('c', result == 0);
    cpu.update_flag('h', bc & 0xFF + hl & 0xFF > 255);
    cpu.hl.set_word(result);
    cpu.clear_flag('n');
}

fn load_bc_ptr_into_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load value pointed by BC into A
    let address = cpu.bc.get_combined();
    cpu.af.high = bus.fetch_byte(address);
}

fn dec_bc(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 16 bits register BC
    if cpu.bc.low == 0 {
        cpu.bc.high = cpu.bc.high.wrapping_sub(1);
    }
    cpu.bc.low = cpu.bc.low.wrapping_sub(1);
}

fn inc_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment C register
    cpu.bc.low = cpu.bc.low.wrapping_add(1);
    cpu.update_flag('z', cpu.bc.low == 0);
    cpu.update_flag('h', cpu.bc.low & 0b1111 == 0);
    cpu.clear_flag('n'); // operation was addition
}

fn dec_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 8 bits register C
    cpu.bc.low = cpu.bc.low.wrapping_sub(1);
    cpu.update_flag('z', cpu.bc.low == 0);
    cpu.update_flag('h', cpu.bc.low & 0b1111 == 0);
    cpu.set_flag('n');
}

fn load_imm_c(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load immediate value into 8 bits register C
    let op = bus.fetch_byte(cpu.pc);
    cpu.bc.low = op;
}

fn rrca(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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
fn stop(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // stops the CPU ; only reverted by reset signal
    cpu.stopped = true;
}

fn load_imm_de(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load 16 bits data into DE register
    let n1 = bus.fetch_byte(cpu.pc);
    let n2 = bus.fetch_byte(cpu.pc + 1);
    cpu.de.low = n1;
    cpu.de.high = n2;
}

fn load_val_de_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load 8 bit data into address pointed by DE
    bus.set_byte(cpu.de.get_combined(), cpu.af.high);
}

fn inc_de(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment 16 bits registry DE ; need to check for carry from low to high
    if cpu.de.low == 255 {
        cpu.de.high = cpu.de.high.wrapping_add(1);
    }
    cpu.de.low = cpu.de.low.wrapping_add(1);
}

fn inc_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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

fn dec_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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

fn load_imm_d(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load immediate value into 8 bits register D
    let op = bus.fetch_byte(cpu.pc);
    cpu.de.high = op;
}

fn rla(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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

fn jr_s8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump relative to pc ; s8 is signed
    let f = bus.fetch_byte(cpu.pc);
    //let op: i8 = bus.fetch_byte(cpu.pc) as i8;
    let op = f as i8;
    cpu.pc = (((cpu.pc as u32 as i32) + (op as i32)) as u16) + 1;
}

fn add_de_to_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // add DE to HL and store into HL
    let de = cpu.de.get_combined();
    let hl = cpu.hl.get_combined();
    let result = de.wrapping_add(hl);
    cpu.update_flag('c', result == 0);
    cpu.update_flag('h', (de & 0xFF) + (hl & 0xFF) > 255);
    cpu.hl.set_word(result);
    cpu.set_flag('n');
}

fn load_de_ptr_into_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load value pointed by DE into A
    let address = cpu.de.get_combined();
    cpu.af.high = bus.fetch_byte(address);
}

fn dec_de(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 16 bits register DE
    if cpu.de.low == 0 {
        cpu.de.high -= 1;
    }
    cpu.de.low -= 1;
}

fn inc_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment E register
    cpu.de.low = cpu.de.low.wrapping_add(1);
    cpu.update_flag('z', cpu.de.low == 0);
    cpu.update_flag('h', cpu.de.low & 0b1111 == 0);
    cpu.set_flag('n'); // operation was addition
}

fn dec_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 8 bits register E
    cpu.update_flag('h', cpu.de.low & 0b1111 == 0);
    cpu.de.low = cpu.de.low.wrapping_sub(1);
    cpu.update_flag('z', cpu.de.low == 0);
    cpu.clear_flag('n');
}

fn load_imm_e(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load immediate value into 8 bits register E
    let op = bus.fetch_byte(cpu.pc);
    cpu.de.low = op;
}

fn rra(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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
fn jr_nz_s8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump s8 bytes from pc if z flag is off
    if cpu.extract_flag('z') == false {
        jr_s8(cpu, bus);
    }
}

fn load_imm_hl(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load 16 bits data into HL register
    let n1 = bus.fetch_byte(cpu.pc);
    let n2 = bus.fetch_byte(cpu.pc + 1);
    cpu.hl.low = n1;
    cpu.hl.high = n2;
}

fn load_val_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load 8 bit data into address pointed by HL and increments HL
    bus.set_byte(cpu.hl.get_combined(), cpu.af.high);
    inc_hl(cpu, bus);
}

fn inc_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment 16 bits registry HL ; need to check for carry from low to high
    if cpu.hl.low == 255 {
        cpu.hl.high = cpu.hl.high.wrapping_add(1);
    }
    cpu.hl.low = cpu.hl.low.wrapping_add(1);
}

fn inc_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment 8 bits register H
    cpu.hl.high += 1;
    if cpu.hl.high == 0 {
        cpu.set_flag('z'); // zero flag
    }
    if cpu.hl.high & 0b1111 == 0 {
        // if the first 4 bytes resulted in a carry
        cpu.set_flag('h'); // set half carry flag
    }
    cpu.set_flag('n'); // operation was addition
}

fn dec_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 8 bits register H
    if cpu.hl.high & 0b1111 == 0 {
        cpu.set_flag('h');
    }
    cpu.hl.high -= 1;
    if cpu.hl.high == 0 {
        cpu.set_flag('z');
    }
    cpu.clear_flag('n');
}

fn load_imm_h(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load immediate value into 8 bits register H
    let op = bus.fetch_byte(cpu.pc);
    cpu.hl.high = op;
}

fn daa(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // bdc things
    if cpu.extract_flag('n') == false {
        if cpu.extract_flag('c') == true || cpu.af.high > 0x99 {
            cpu.af.high = cpu.af.high.wrapping_add(0x60);
            cpu.set_flag('c');
        }
        if cpu.extract_flag('h') == true || (cpu.af.high & 0x0f) > 0x09 {
            cpu.af.high = cpu.af.high.wrapping_add(0x6);
        }
    } else {
        if cpu.extract_flag('c') == true {
            cpu.af.high = cpu.af.high.wrapping_sub(0x60);
        }
        if cpu.extract_flag('h') == true {
            cpu.af.high = cpu.af.high.wrapping_sub(0x6);
        }
    }

    if cpu.af.high == 0 {
        cpu.set_flag('z');
    } else {
        cpu.clear_flag('z');
    }
    cpu.clear_flag('h');
}

fn jr_z_s8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump relative of s8 if z flag is true
    if cpu.extract_flag('z') == true {
        jr_s8(cpu, bus);
    }
}

fn add_hl_to_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // add HL to HL and store into HL
    let hl = cpu.hl.get_combined();
    let result = hl.wrapping_add(hl);
    if result == 0 {
        cpu.set_flag('c');
    }
    if hl & 0xFF + hl & 0xFF > 255 {
        // checking half carry
        cpu.set_flag('h');
    }
    cpu.hl.set_word(result);
    cpu.set_flag('n');
}

fn load_hl_ptr_into_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load value pointed by HL into A and increment HL
    let address = cpu.hl.get_combined();
    cpu.af.high = bus.fetch_byte(address);
    inc_hl(cpu, bus);
}

fn dec_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 16 bits register HL
    if cpu.hl.low == 0 {
        cpu.hl.high = cpu.hl.high.wrapping_sub(1);
    }
    cpu.hl.low = cpu.hl.low.wrapping_sub(1);
}

fn inc_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment L register
    cpu.hl.low = cpu.hl.low.wrapping_add(1);
    cpu.update_flag('z', cpu.hl.low == 0);
    cpu.update_flag('h', cpu.hl.low & 0b1111 == 0);
    cpu.set_flag('n'); // operation was addition
}

fn dec_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement 8 bits register L
    if cpu.hl.low & 0b1111 == 0 {
        cpu.set_flag('h');
    }
    cpu.hl.low -= 1;
    if cpu.hl.low == 0 {
        cpu.set_flag('z');
    }
    cpu.clear_flag('n');
}

fn load_imm_l(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load immediate value into 8 bits register L
    let op = bus.fetch_byte(cpu.pc);
    cpu.hl.low = op;
}

fn cpl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // take comeplent of A
    cpu.af.high = !cpu.af.high;
    cpu.set_flag('h');
    cpu.set_flag('n');
}

// ======================================================
// 0x3X Instructions
// ======================================================
fn jr_nc_s8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump s8 bytes if carry flag is 0
    if cpu.extract_flag('c') == false {
        jr_s8(cpu, bus);
    }
}

fn load_imm_sp(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load d16 value into SP register
    let n1 = bus.fetch_byte(cpu.pc);
    let n2 = bus.fetch_byte(cpu.pc + 1);
    cpu.sp = ((n2 as u16) << 8) + n1 as u16;
}

fn load_val_hl_ptr_dec(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load A into address pointed by HL and decrements HL
    bus.set_byte(cpu.hl.get_combined(), cpu.af.high);
    dec_hl(cpu, bus);
}

fn inc_sp(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increments SP
    cpu.pc += 1;
}

fn inc_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // increment value at memory pointed by HL register
    let adr = cpu.hl.get_combined();
    let val = bus.fetch_byte(adr);
    let res = val.wrapping_add(1);
    bus.set_byte(adr, res);

    cpu.update_flag('z', res == 0);
    cpu.update_flag('h', res & 0b1111 == 0);
    cpu.clear_flag('n');
}

fn dec_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // decrement value at memory pointed by HL register
    let adr = cpu.hl.get_combined();
    let val = bus.fetch_byte(adr);
    let res = val.wrapping_sub(1);
    bus.set_byte(adr, res);

    cpu.update_flag('z', res == 0);
    cpu.update_flag('h', res & 0b1111 == 0);
    cpu.set_flag('n');
}

fn load_d8_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load d8 value into memory pointed by HL register
    let op = bus.fetch_byte(cpu.pc);
    bus.set_byte(cpu.hl.get_combined(), op);
}

fn set_carry_flag(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // set carry flag
    cpu.set_flag('c');
}

fn jr_c_s8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump if carry flag is set
    if cpu.extract_flag('c') == true {
        jr_s8(cpu, bus);
    }
}

fn add_sp_to_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // add SP to HL register
    let hl = cpu.hl.get_combined();
    let result = cpu.sp + hl;
    cpu.update_flag('c', result == 0);
    cpu.update_flag('h', cpu.sp & 0xFF + hl & 0xFF > 255);
    cpu.sp = result;
    cpu.set_flag('n');
}

fn load_hl_ptr_into_a_dec(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load value pointed by HL register into A then decrement HL
    let address = cpu.hl.get_combined();
    cpu.af.high = bus.fetch_byte(address);
    dec_hl(cpu, bus);
}

fn dec_sp(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // dec SP register
    cpu.sp -= 1;
}

fn inc_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // increment A
    cpu.clear_flag('n');
    cpu.af.high += 1;
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('z', cpu.af.high == 0);
}

fn dec_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // decrement A
    cpu.set_flag('n');
    cpu.af.high -= 1;
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('z', cpu.af.high == 0);
}

fn load_imm_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load d8 value into A register
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high = op;
}

fn ccf(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // flip carry flag
    cpu.update_flag('c', !cpu.extract_flag('c'));
    cpu.clear_flag('h');
    cpu.clear_flag('n');
}

// ======================================================
// 0x4X Instructions
// ======================================================
fn ld_b_b(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

fn ld_b_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high = cpu.bc.low;
}

fn ld_b_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high = cpu.de.high;
}

fn ld_b_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high = cpu.de.low;
}

fn ld_b_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high = cpu.hl.high;
}

fn ld_b_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high = cpu.hl.low;
}

fn ld_b_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.bc.high = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_b_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high = cpu.af.high;
}

fn ld_c_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low = cpu.bc.high;
}

fn ld_c_c(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

fn ld_c_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low = cpu.de.high;
}

fn ld_c_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low = cpu.de.low;
}

fn ld_c_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low = cpu.hl.high;
}

fn ld_c_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low = cpu.hl.low;
}

fn ld_c_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.bc.low = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_c_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low = cpu.af.high;
}

// ======================================================
// 0x5X Instructions
// ======================================================
fn ld_d_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high = cpu.bc.high;
}

fn ld_d_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high = cpu.bc.low;
}

fn ld_d_d(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

fn ld_d_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high = cpu.de.low;
}

fn ld_d_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high = cpu.hl.high;
}

fn ld_d_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high = cpu.hl.low;
}

fn ld_d_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.de.high = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_d_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high = cpu.af.high;
}

fn ld_e_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low = cpu.bc.high;
}

fn ld_e_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low = cpu.bc.low;
}

fn ld_e_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low = cpu.de.high;
}

fn ld_e_e(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

fn ld_e_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low = cpu.hl.high;
}

fn ld_e_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low = cpu.hl.low;
}

fn ld_e_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.de.low = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_e_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low = cpu.af.high;
}

// ======================================================
// 0x6X Instructions
// ======================================================
fn ld_h_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high = cpu.bc.high;
}

fn ld_h_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high = cpu.bc.low;
}

fn ld_h_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high = cpu.de.high;
}

fn ld_h_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high = cpu.de.low;
}

fn ld_h_h(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

fn ld_h_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high = cpu.hl.low;
}

fn ld_h_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.hl.high = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_h_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high = cpu.af.high;
}

fn ld_l_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low = cpu.bc.high;
}

fn ld_l_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low = cpu.bc.low;
}

fn ld_l_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low = cpu.de.high;
}

fn ld_l_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low = cpu.de.low;
}

fn ld_l_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low = cpu.hl.high;
}

fn ld_l_l(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

fn ld_l_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.hl.low = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_l_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low = cpu.af.high;
}

// ======================================================
// 0x7X Instructions
// ======================================================
fn load_b_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load content of B into memory pointed by HL
    bus.set_byte(cpu.hl.get_combined(), cpu.bc.high);
}

fn load_c_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(cpu.hl.get_combined(), cpu.bc.low);
}

fn load_d_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(cpu.hl.get_combined(), cpu.de.high);
}

fn load_e_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(cpu.hl.get_combined(), cpu.de.low);
}

fn load_h_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(cpu.hl.get_combined(), cpu.hl.high);
}

fn load_l_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(cpu.hl.get_combined(), cpu.hl.low);
}

fn halt(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // halt cpu ; wait for interrupt
    cpu.halted = true;
}

fn load_a_into_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(cpu.hl.get_combined(), cpu.af.high);
}

fn ld_a_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high = cpu.bc.high;
}

fn ld_a_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high = cpu.bc.low;
}

fn ld_a_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high = cpu.de.high;
}

fn ld_a_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high = cpu.de.low;
}

fn ld_a_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high = cpu.hl.high;
}

fn ld_a_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high = cpu.hl.low;
}

fn ld_a_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.af.high = bus.fetch_byte(cpu.hl.get_combined());
}

fn ld_a_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    nop(cpu, bus);
}

// ======================================================
// 0x8X Instructions
// ======================================================
fn add_a_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // add B to A
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.bc.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn add_a_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.bc.low);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn add_a_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.de.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn add_a_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.de.low);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn add_a_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.hl.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn add_a_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.hl.low);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn add_hl_ptr_to_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(bus.fetch_byte(cpu.hl.get_combined()));
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn add_a_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high = cpu.af.high.wrapping_add(cpu.af.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // add B to A with carry
    cpu.clear_flag('n');
    cpu.af.high += cpu.bc.high + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += cpu.bc.low + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += cpu.de.high + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += cpu.de.low + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += cpu.hl.high + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += cpu.hl.low + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_hl_ptr_into_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += bus.fetch_byte(cpu.hl.get_combined()) + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn adc_a_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.af.high += cpu.af.high + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

// ======================================================
// 0x9X Instructions
// ======================================================
fn sub_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // sub B to A
    cpu.set_flag('n');
    cpu.af.high = cpu.af.high.wrapping_sub(cpu.bc.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.bc.high > cpu.af.high);
}

fn sub_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    
    cpu.af.high = cpu.af.high.wrapping_sub(cpu.bc.low);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.bc.low > cpu.af.high);
}

fn sub_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    cpu.af.high = cpu.af.high.wrapping_sub(cpu.de.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.de.high > cpu.af.high);
}

fn sub_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    cpu.af.high = cpu.af.high.wrapping_sub(cpu.de.low);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.de.low > cpu.af.high);
}

fn sub_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    cpu.af.high -= cpu.hl.high;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.hl.high > cpu.af.high);
}

fn sub_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    cpu.af.high -= cpu.hl.low;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.hl.low > cpu.af.high);
}

fn sub_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.set_flag('n');
    let op = bus.fetch_byte(cpu.hl.get_combined());
    cpu.af.high -= op;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', op > cpu.af.high);
}

fn sub_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    cpu.af.high -= cpu.af.high;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > cpu.af.high);
}

fn sbc_a_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // sub B to A with carry
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.af.high -= cpu.bc.high + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.bc.high + cf > cpu.af.high);
}

fn sbc_a_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.update_flag('c', cpu.bc.low + cf > cpu.af.high);
    cpu.af.high -= cpu.bc.low + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn sbc_a_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.update_flag('c', cpu.de.high + cf > cpu.af.high);
    cpu.af.high -= cpu.de.high + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn sbc_a_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.update_flag('c', cpu.de.low + cf > cpu.af.high);
    cpu.af.high -= cpu.de.low + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn sbc_a_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.update_flag('c', cpu.hl.high + cf > cpu.af.high);
    cpu.af.high -= cpu.hl.high + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn sbc_a_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.update_flag('c', cpu.hl.low + cf > cpu.af.high);
    cpu.af.high -= cpu.hl.low + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn sbc_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    let op = bus.fetch_byte(cpu.hl.get_combined());
    cpu.update_flag('c', op + cf > cpu.af.high);
    cpu.af.high -= op + cf;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn sbc_a_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.update_flag('c', cpu.af.high + cf > cpu.af.high);
    cpu.af.high -= cpu.af.high + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

// ======================================================
// 0xAX Instructions
// ======================================================
fn and_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // logical AND of B and A, stored into A
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.bc.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.bc.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.de.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.de.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.hl.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.hl.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= bus.fetch_byte(cpu.hl.get_combined());
    cpu.update_flag('z', cpu.af.high == 0);
}

fn and_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    cpu.af.high &= cpu.af.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // XOR of B and A, stored into A
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.bc.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.bc.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.de.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.de.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.hl.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.hl.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= bus.fetch_byte(cpu.hl.get_combined());
    cpu.update_flag('z', cpu.af.high == 0);
}

fn xor_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.af.high ^= cpu.af.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

// ======================================================
// 0xBX Instructions
// ======================================================
fn or_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // or B and A, stored into A
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.bc.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.bc.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.de.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.de.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.hl.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.hl.low;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= bus.fetch_byte(cpu.hl.get_combined());
    cpu.update_flag('z', cpu.af.high == 0);
}

fn or_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.af.high |= cpu.af.high;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn cp_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // compare A and B by calculating A - B and setting Z flag (does not affect A)
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.bc.high;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.bc.high > cpu.af.high);
}

fn cp_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.bc.low;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.bc.low > cpu.af.high);
}

fn cp_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.de.high;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.de.high > cpu.af.high);
}

fn cp_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.de.low;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.de.low > cpu.af.high);
}

fn cp_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.hl.high;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.hl.high > cpu.af.high);
}

fn cp_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.hl.low;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.hl.low > cpu.af.high);
}

fn cp_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.set_flag('n');
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let diff = cpu.af.high.wrapping_sub(op);
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', op > cpu.af.high);
}

fn cp_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.set_flag('n');
    let diff = cpu.af.high - cpu.af.high;
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > cpu.af.high);
}

// ======================================================
// 0xCX Instructions
// ======================================================
fn ret_nz(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // if flag Z is unset, ret from subroutine
    if cpu.extract_flag('z') == false {
        ret(cpu, bus);
    }
}

fn pop_bc(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // pop value on top of the stack into BC
    cpu.bc.low = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
    cpu.bc.high = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
}

fn jp_nz_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump to immediate address a16 if Z flag is unset
    if cpu.extract_flag('z') == false {
        jp_a16(cpu, bus);
    }
}

fn jp_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump to immediate address a16
    let op1 = bus.fetch_byte(cpu.pc);
    let op2 = bus.fetch_byte(cpu.pc + 1);
    cpu.pc = ((op2 as u16) << 8) + (op1 as u16);
}


fn call_nz_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // if Z flag is unset, push PC on stack and set PC to a16 address
    if cpu.extract_flag('z') == false {
        call_a16(cpu, bus);
    }
}

fn push_bc(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // push BC content to stack
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.bc.high);
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.bc.low);
}

fn add_a_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // add d8 value to A
    cpu.clear_flag('n');
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high = cpu.af.high.wrapping_add(op);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn rst_0(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // push PC on stack and jump to 0x00 memory address
    bus.set_byte(cpu.sp, (cpu.pc & 0b1111) as u8);
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, ((cpu.pc & 0b11110000) >> 8) as u8);
    cpu.sp -= 1;
    cpu.pc = 0;
}

fn ret_z(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // return from subroutine if Z flag is set
    if cpu.extract_flag('z') == true {
        ret(cpu, bus);
    }
}

fn ret(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // return from subroutine
    cpu.pc = cpu.pop_stack_d16(bus);
    cpu.pc += 3; // skip call a16 instruction
}

fn jp_z_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // jump to a16 address if Z flag is set
    if cpu.extract_flag('z') == true {
        jp_a16(cpu, bus);
    }
}

fn special_cb(_cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // shouldn't be called, as CB opcode is used for 2 opcodes long instructions
    panic!("Instruction 0xCB called !");
}

fn call_z_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // call subroutine at a16 address if Z flag is set
    if cpu.extract_flag('z') == true {
        call_a16(cpu, bus);
    }
}

fn call_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // call subroutine at address a16
    let op1 = bus.fetch_byte(cpu.pc);
    let op2 = bus.fetch_byte(cpu.pc + 1);
    cpu.push_stack(bus, cpu.pc - 1);
    cpu.pc = ((op2 as u16) << 8) + (op1 as u16);
}

fn adc_a_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // add d8 to A with carry
    let op = bus.fetch_byte(cpu.pc);
    cpu.clear_flag('n');
    cpu.af.high += op + (cpu.extract_flag('c') as u8);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.update_flag('c', cpu.af.high > 0);
}

fn rst_1(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 8);
}

// ======================================================
// 0xDX Instructions
// ======================================================
fn ret_nc(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // return from subroutine if C is unset
    if cpu.extract_flag('c') == false {
        ret(cpu, bus);
    }
}

fn pop_de(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.de.low = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
    cpu.de.high = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
}

fn jp_nc_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    if cpu.extract_flag('c') == false {
        jp_a16(cpu, bus);
    }
}

fn call_nc_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    if cpu.extract_flag('c') == false {
        call_a16(cpu, bus);
    }
}

fn push_de(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.de.high);
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.de.low);
}

fn sub_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // sub d8 value to A
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high = cpu.af.high.wrapping_sub(op);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
    cpu.set_flag('n');
}

fn rst_2(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 0x10);
}

fn ret_c(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    if cpu.extract_flag('c') == true {
        ret(cpu, bus);
    }
}

fn ret_i(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.ime = true;
    cpu.pc = cpu.pop_stack_d16(bus);
}

fn jp_c_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    if cpu.extract_flag('c') == true {
        jp_a16(cpu, bus);
    }
}

fn call_c_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    if cpu.extract_flag('c') == true {
        call_a16(cpu, bus);
    }
}

fn sbc_a_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.pc);
    cpu.set_flag('n');
    let cf = cpu.extract_flag('c') as u8;
    cpu.af.high = cpu.af.high.wrapping_sub(op + cf);
    cpu.update_flag('c', op + cf > cpu.af.high);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.update_flag('h', cpu.af.high & 0b1111 == 0);
}

fn rst_3(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 0x18);
}

// ======================================================
// 0xEX Instructions
// ======================================================
fn ld_a_to_ffa8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // store content of A register to address FFa8
    let op = bus.fetch_byte(cpu.pc);
    bus.set_byte(0xFF00 + (op as u16), cpu.af.high);
}

fn pop_hl(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.hl.low = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
    cpu.hl.high = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
}

fn ld_a_to_ffc(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    bus.set_byte(0xFF00 + (cpu.bc.low as u16), cpu.af.high);
}

fn push_hl(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.hl.high);
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.hl.low);
}

fn and_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.clear_flag('c');
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high &= op;
    cpu.update_flag('z', cpu.af.high == 0);
}

fn rst_4(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 0x20);
}

fn add_sp_s8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // add s8 operand to SP register
    let op = bus.fetch_byte(cpu.pc) as i8;
    if op < 0 {
        cpu.sp -= (-op) as u16;
    } else {
        cpu.sp += op as u16;
    }
}

fn jp_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // jump to address in HL register
    cpu.pc = cpu.hl.get_combined();
}

fn ld_a16_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // store A in memory location a16
    let op1 = bus.fetch_byte(cpu.pc);
    let op2 = bus.fetch_byte(cpu.pc + 1);
    bus.set_byte(((op2 as u16) << 8) + (op1 as u16), cpu.af.high);
}

fn xor_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high ^= op;
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.af.high == 0);
}

fn rst_5(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 0x28);
}

// ======================================================
// 0xFX Instructions
// ======================================================
fn ld_ffa8_to_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load content of memory location FFa8 into A register
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high = bus.fetch_byte(0xFF00 + (op as u16));
}

fn pop_af(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.af.low = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
    cpu.af.high = bus.fetch_byte(cpu.sp);
    cpu.sp += 1;
}

fn ld_ffc_to_a(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.af.high = bus.fetch_byte(0xFF00 + (cpu.bc.low as u16));
}

fn di(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // disable interrupts
    cpu.ime = false;
}

fn push_af(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.af.high);
    cpu.sp -= 1;
    bus.set_byte(cpu.sp, cpu.af.low);
}

fn or_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.pc);
    cpu.af.high |= op;
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
    cpu.clear_flag('c');
}

fn rst_6(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 0x30);
}

fn ld_sp_s8_to_hl(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    // load content of location SP + s8 into register HL
    let op = bus.fetch_byte(cpu.pc) as i8;
    cpu.clear_flag('z');
    cpu.clear_flag('n');
    if op < 0 {
        cpu.hl.set_word(cpu.sp + ((-op) as u16));
    } else {
        cpu.hl.set_word(cpu.sp + (op as u16));
    }
    cpu.clear_flag('c');
    cpu.update_flag('h', cpu.hl.get_combined() & 0b11111111 == 0);
}

fn ld_sp_hl(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.sp = cpu.hl.get_combined();
}

fn ld_a_a16(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op1 = bus.fetch_byte(cpu.pc);
    let op2 = bus.fetch_byte(cpu.pc + 1);
    cpu.af.high = bus.fetch_byte(((op2 as u16) << 8) + (op1 as u16));
}

fn ei(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    // enable interrupts
    cpu.ime = true;
}

fn cp_d8(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    cpu.set_flag('n');
    let op = bus.fetch_byte(cpu.pc);
    let diff = cpu.af.high.wrapping_sub(op);
    cpu.update_flag('z', diff == 0);
    cpu.update_flag('h', diff & 0b1111 == 0);
    cpu.update_flag('c', cpu.bc.low > cpu.af.high);
}

fn rst_7(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    rst_general(cpu, bus, 0x38)
}

fn rst_general(cpu: &mut cpu::CPU, bus: &mut bus::Bus, address: u16) {
    cpu.push_stack(bus, cpu.pc);
    cpu.pc = address;
}