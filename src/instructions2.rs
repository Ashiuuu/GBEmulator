use crate::cpu;
use crate::bus;
pub use crate::instructions::Instruction;

impl Instruction<'_> {
    pub const SECOND_SET: [Instruction<'static>; 256] = [
        Instruction {
            disassembly: "RLC B",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_b,
        },
        Instruction {
            disassembly: "RLC C",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_c,
        },
        Instruction {
            disassembly: "RLC D",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_d,
        },
        Instruction {
            disassembly: "RLC E",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_e,
        },
        Instruction {
            disassembly: "RLC H",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_h,
        },
        Instruction {
            disassembly: "RLC L",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_l,
        },
        Instruction {
            disassembly: "RLC (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_hl_ptr,
        },
        Instruction {
            disassembly: "RLC A",
            op_len: 2,
            clock_cycles: 2,
            execute: rlc_a,
        },
        Instruction {
            disassembly: "RRC B",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_b,
        },
        Instruction {
            disassembly: "RRC C",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_c,
        },
        Instruction {
            disassembly: "RRC D",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_d,
        },
        Instruction {
            disassembly: "RRC E",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_e,
        },
        Instruction {
            disassembly: "RRC H",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_h,
        },
        Instruction {
            disassembly: "RRC L",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_l,
        },
        Instruction {
            disassembly: "RRC (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_hl_ptr,
        },
        Instruction {
            disassembly: "RRC A",
            op_len: 2,
            clock_cycles: 2,
            execute: rrc_a,
        },
        Instruction {
            disassembly: "RL B",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_b,
        },
        Instruction {
            disassembly: "RL C",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_c,
        },
        Instruction {
            disassembly: "RL D",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_d,
        },
        Instruction {
            disassembly: "RL E",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_e,
        },
        Instruction {
            disassembly: "RL H",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_h,
        },
        Instruction {
            disassembly: "RL L",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_l,
        },
        Instruction {
            disassembly: "RL (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_hl_ptr,
        },
        Instruction {
            disassembly: "RL A",
            op_len: 2,
            clock_cycles: 2,
            execute: rl_a,
        },
        Instruction {
            disassembly: "RR B",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_b,
        },
        Instruction {
            disassembly: "RR C",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_c,
        },
        Instruction {
            disassembly: "RR D",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_d,
        },
        Instruction {
            disassembly: "RR E",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_e,
        },
        Instruction {
            disassembly: "RR H",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_h,
        },
        Instruction {
            disassembly: "RR L",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_l,
        },
        Instruction {
            disassembly: "RR (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_hl_ptr,
        },
        Instruction {
            disassembly: "RR A",
            op_len: 2,
            clock_cycles: 2,
            execute: rr_a,
        },
        Instruction {
            disassembly: "SLA B",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_b,
        },
        Instruction {
            disassembly: "SLA C",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_c,
        },
        Instruction {
            disassembly: "SLA D",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_d,
        },
        Instruction {
            disassembly: "SLA E",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_e,
        },
        Instruction {
            disassembly: "SLA H",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_h,
        },
        Instruction {
            disassembly: "SLA L",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_l,
        },
        Instruction {
            disassembly: "SLA (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_hl_ptr,
        },
        Instruction {
            disassembly: "SLA A",
            op_len: 2,
            clock_cycles: 2,
            execute: sla_a,
        },
        Instruction {
            disassembly: "SRA B",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_b,
        },
        Instruction {
            disassembly: "SRA C",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_c,
        },
        Instruction {
            disassembly: "SRA D",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_d,
        },
        Instruction {
            disassembly: "SRA E",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_e,
        },
        Instruction {
            disassembly: "SRA H",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_h,
        },
        Instruction {
            disassembly: "SRA L",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_l,
        },
        Instruction {
            disassembly: "SRA (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_hl_ptr,
        },
        Instruction {
            disassembly: "SRA A",
            op_len: 2,
            clock_cycles: 2,
            execute: sra_a,
        },
        Instruction {
            disassembly: "SWAP B",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_b,
        },
        Instruction {
            disassembly: "SWAP C",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_c,
        },
        Instruction {
            disassembly: "SWAP D",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_d,
        },
        Instruction {
            disassembly: "SWAP E",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_e,
        },
        Instruction {
            disassembly: "SWAP H",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_h,
        },
        Instruction {
            disassembly: "SWAP L",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_l,
        },
        Instruction {
            disassembly: "SWAP (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_hl_ptr,
        },
        Instruction {
            disassembly: "SWAP A",
            op_len: 2,
            clock_cycles: 2,
            execute: swap_a,
        },
        Instruction {
            disassembly: "SRL B",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_b,
        },
        Instruction {
            disassembly: "SRL C",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_c,
        },
        Instruction {
            disassembly: "SRL D",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_d,
        },
        Instruction {
            disassembly: "SRL E",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_e,
        },
        Instruction {
            disassembly: "SRL H",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_h,
        },
        Instruction {
            disassembly: "SRL L",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_l,
        },
        Instruction {
            disassembly: "SRL (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_hl_ptr,
        },
        Instruction {
            disassembly: "SRL A",
            op_len: 2,
            clock_cycles: 2,
            execute: srl_a,
        },
        Instruction {
            disassembly: "BIT 0 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_b,
        },
        Instruction {
            disassembly: "BIT 0 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_c,
        },
        Instruction {
            disassembly: "BIT 0 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_d,
        },
        Instruction {
            disassembly: "BIT 0 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_e,
        },
        Instruction {
            disassembly: "BIT 0 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_h,
        },
        Instruction {
            disassembly: "BIT 0 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_l,
        },
        Instruction {
            disassembly: "BIT 0 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 0 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_0_a,
        },
        Instruction {
            disassembly: "BIT 1 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_b,
        },
        Instruction {
            disassembly: "BIT 1 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_c,
        },
        Instruction {
            disassembly: "BIT 1 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_d,
        },
        Instruction {
            disassembly: "BIT 1 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_e,
        },
        Instruction {
            disassembly: "BIT 1 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_h,
        },
        Instruction {
            disassembly: "BIT 1 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_l,
        },
        Instruction {
            disassembly: "BIT 1 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 1 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_1_a,
        },
        Instruction {
            disassembly: "BIT 2 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_b,
        },
        Instruction {
            disassembly: "BIT 2 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_c,
        },
        Instruction {
            disassembly: "BIT 2 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_d,
        },
        Instruction {
            disassembly: "BIT 2 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_e,
        },
        Instruction {
            disassembly: "BIT 2 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_h,
        },
        Instruction {
            disassembly: "BIT 2 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_l,
        },
        Instruction {
            disassembly: "BIT 2 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 2 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_2_a,
        },
        Instruction {
            disassembly: "BIT 3 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_b,
        },
        Instruction {
            disassembly: "BIT 3 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_c,
        },
        Instruction {
            disassembly: "BIT 3 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_d,
        },
        Instruction {
            disassembly: "BIT 3 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_e,
        },
        Instruction {
            disassembly: "BIT 3 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_h,
        },
        Instruction {
            disassembly: "BIT 3 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_l,
        },
        Instruction {
            disassembly: "BIT 3 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 3 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_3_a,
        },
        Instruction {
            disassembly: "BIT 4 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_b,
        },
        Instruction {
            disassembly: "BIT 4 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_c,
        },
        Instruction {
            disassembly: "BIT 4 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_d,
        },
        Instruction {
            disassembly: "BIT 4 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_e,
        },
        Instruction {
            disassembly: "BIT 4 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_h,
        },
        Instruction {
            disassembly: "BIT 4 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_l,
        },
        Instruction {
            disassembly: "BIT 4 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 4 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_4_a,
        },
        Instruction {
            disassembly: "BIT 5 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_b,
        },
        Instruction {
            disassembly: "BIT 5 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_c,
        },
        Instruction {
            disassembly: "BIT 5 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_d,
        },
        Instruction {
            disassembly: "BIT 5 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_e,
        },
        Instruction {
            disassembly: "BIT 5 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_h,
        },
        Instruction {
            disassembly: "BIT 5 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_l,
        },
        Instruction {
            disassembly: "BIT 5 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 5 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_5_a,
        },
        Instruction {
            disassembly: "BIT 6 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_b,
        },
        Instruction {
            disassembly: "BIT 6 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_c,
        },
        Instruction {
            disassembly: "BIT 6 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_d,
        },
        Instruction {
            disassembly: "BIT 6 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_e,
        },
        Instruction {
            disassembly: "BIT 6 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_h,
        },
        Instruction {
            disassembly: "BIT 6 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_l,
        },
        Instruction {
            disassembly: "BIT 6 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 6 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_6_a,
        },
        Instruction {
            disassembly: "BIT 7 B",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_b,
        },
        Instruction {
            disassembly: "BIT 7 C",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_c,
        },
        Instruction {
            disassembly: "BIT 7 D",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_d,
        },
        Instruction {
            disassembly: "BIT 7 E",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_e,
        },
        Instruction {
            disassembly: "BIT 7 H",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_h,
        },
        Instruction {
            disassembly: "BIT 7 L",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_l,
        },
        Instruction {
            disassembly: "BIT 7 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_hl_ptr,
        },
        Instruction {
            disassembly: "BIT 7 A",
            op_len: 2,
            clock_cycles: 2,
            execute: bit_7_a,
        },
        Instruction {
            disassembly: "RES 0 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_b,
        },
        Instruction {
            disassembly: "RES 0 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_c,
        },
        Instruction {
            disassembly: "RES 0 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_d,
        },
        Instruction {
            disassembly: "RES 0 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_e,
        },
        Instruction {
            disassembly: "RES 0 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_h,
        },
        Instruction {
            disassembly: "RES 0 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_l,
        },
        Instruction {
            disassembly: "RES 0 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_hl_ptr,
        },
        Instruction {
            disassembly: "RES 0 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_0_a,
        },
        Instruction {
            disassembly: "RES 1 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_b,
        },
        Instruction {
            disassembly: "RES 1 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_c,
        },
        Instruction {
            disassembly: "RES 1 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_d,
        },
        Instruction {
            disassembly: "RES 1 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_e,
        },
        Instruction {
            disassembly: "RES 1 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_h,
        },
        Instruction {
            disassembly: "RES 1 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_l,
        },
        Instruction {
            disassembly: "RES 1 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_hl_ptr,
        },
        Instruction {
            disassembly: "RES 1 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_1_a,
        },
        Instruction {
            disassembly: "RES 2 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_b,
        },
        Instruction {
            disassembly: "RES 2 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_c,
        },
        Instruction {
            disassembly: "RES 2 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_d,
        },
        Instruction {
            disassembly: "RES 2 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_e,
        },
        Instruction {
            disassembly: "RES 2 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_h,
        },
        Instruction {
            disassembly: "RES 2 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_l,
        },
        Instruction {
            disassembly: "RES 2 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_hl_ptr,
        },
        Instruction {
            disassembly: "RES 2 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_2_a,
        },
        Instruction {
            disassembly: "RES 3 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_b,
        },
        Instruction {
            disassembly: "RES 3 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_c,
        },
        Instruction {
            disassembly: "RES 3 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_d,
        },
        Instruction {
            disassembly: "RES 3 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_e,
        },
        Instruction {
            disassembly: "RES 3 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_h,
        },
        Instruction {
            disassembly: "RES 3 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_l,
        },
        Instruction {
            disassembly: "RES 3 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_hl_ptr,
        },
        Instruction {
            disassembly: "RES 3 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_3_a,
        },
        Instruction {
            disassembly: "RES 4 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_b,
        },
        Instruction {
            disassembly: "RES 4 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_c,
        },
        Instruction {
            disassembly: "RES 4 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_d,
        },
        Instruction {
            disassembly: "RES 4 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_e,
        },
        Instruction {
            disassembly: "RES 4 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_h,
        },
        Instruction {
            disassembly: "RES 4 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_l,
        },
        Instruction {
            disassembly: "RES 4 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_hl_ptr,
        },
        Instruction {
            disassembly: "RES 4 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_4_a,
        },
        Instruction {
            disassembly: "RES 5 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_b,
        },
        Instruction {
            disassembly: "RES 5 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_c,
        },
        Instruction {
            disassembly: "RES 5 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_d,
        },
        Instruction {
            disassembly: "RES 5 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_e,
        },
        Instruction {
            disassembly: "RES 5 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_h,
        },
        Instruction {
            disassembly: "RES 5 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_l,
        },
        Instruction {
            disassembly: "RES 5 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_hl_ptr,
        },
        Instruction {
            disassembly: "RES 5 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_5_a,
        },
        Instruction {
            disassembly: "RES 6 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_b,
        },
        Instruction {
            disassembly: "RES 6 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_c,
        },
        Instruction {
            disassembly: "RES 6 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_d,
        },
        Instruction {
            disassembly: "RES 6 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_e,
        },
        Instruction {
            disassembly: "RES 6 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_h,
        },
        Instruction {
            disassembly: "RES 6 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_l,
        },
        Instruction {
            disassembly: "RES 6 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_hl_ptr,
        },
        Instruction {
            disassembly: "RES 6 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_6_a,
        },
        Instruction {
            disassembly: "RES 7 B",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_b,
        },
        Instruction {
            disassembly: "RES 7 C",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_c,
        },
        Instruction {
            disassembly: "RES 7 D",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_d,
        },
        Instruction {
            disassembly: "RES 7 E",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_e,
        },
        Instruction {
            disassembly: "RES 7 H",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_h,
        },
        Instruction {
            disassembly: "RES 7 L",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_l,
        },
        Instruction {
            disassembly: "RES 7 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_hl_ptr,
        },
        Instruction {
            disassembly: "RES 7 A",
            op_len: 2,
            clock_cycles: 2,
            execute: res_7_a,
        },
        Instruction {
            disassembly: "SET 0 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_b,
        },
        Instruction {
            disassembly: "SET 0 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_c,
        },
        Instruction {
            disassembly: "SET 0 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_d,
        },
        Instruction {
            disassembly: "SET 0 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_e,
        },
        Instruction {
            disassembly: "SET 0 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_h,
        },
        Instruction {
            disassembly: "SET 0 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_l,
        },
        Instruction {
            disassembly: "SET 0 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_hl_ptr,
        },
        Instruction {
            disassembly: "SET 0 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_0_a,
        },
        Instruction {
            disassembly: "SET 1 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_b,
        },
        Instruction {
            disassembly: "SET 1 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_c,
        },
        Instruction {
            disassembly: "SET 1 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_d,
        },
        Instruction {
            disassembly: "SET 1 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_e,
        },
        Instruction {
            disassembly: "SET 1 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_h,
        },
        Instruction {
            disassembly: "SET 1 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_l,
        },
        Instruction {
            disassembly: "SET 1 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_hl_ptr,
        },
        Instruction {
            disassembly: "SET 1 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_1_a,
        },
        Instruction {
            disassembly: "SET 2 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_b,
        },
        Instruction {
            disassembly: "SET 2 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_c,
        },
        Instruction {
            disassembly: "SET 2 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_d,
        },
        Instruction {
            disassembly: "SET 2 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_e,
        },
        Instruction {
            disassembly: "SET 2 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_h,
        },
        Instruction {
            disassembly: "SET 2 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_l,
        },
        Instruction {
            disassembly: "SET 2 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_hl_ptr,
        },
        Instruction {
            disassembly: "SET 2 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_2_a,
        },
        Instruction {
            disassembly: "SET 3 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_b,
        },
        Instruction {
            disassembly: "SET 3 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_c,
        },
        Instruction {
            disassembly: "SET 3 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_d,
        },
        Instruction {
            disassembly: "SET 3 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_e,
        },
        Instruction {
            disassembly: "SET 3 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_h,
        },
        Instruction {
            disassembly: "SET 3 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_l,
        },
        Instruction {
            disassembly: "SET 3 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_hl_ptr,
        },
        Instruction {
            disassembly: "SET 3 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_3_a,
        },
        Instruction {
            disassembly: "SET 4 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_b,
        },
        Instruction {
            disassembly: "SET 4 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_c,
        },
        Instruction {
            disassembly: "SET 4 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_d,
        },
        Instruction {
            disassembly: "SET 4 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_e,
        },
        Instruction {
            disassembly: "SET 4 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_h,
        },
        Instruction {
            disassembly: "SET 4 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_l,
        },
        Instruction {
            disassembly: "SET 4 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_hl_ptr,
        },
        Instruction {
            disassembly: "SET 4 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_4_a,
        },
        Instruction {
            disassembly: "SET 5 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_b,
        },
        Instruction {
            disassembly: "SET 5 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_c,
        },
        Instruction {
            disassembly: "SET 5 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_d,
        },
        Instruction {
            disassembly: "SET 5 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_e,
        },
        Instruction {
            disassembly: "SET 5 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_h,
        },
        Instruction {
            disassembly: "SET 5 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_l,
        },
        Instruction {
            disassembly: "SET 5 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_hl_ptr,
        },
        Instruction {
            disassembly: "SET 5 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_5_a,
        },
        Instruction {
            disassembly: "SET 6 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_b,
        },
        Instruction {
            disassembly: "SET 6 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_c,
        },
        Instruction {
            disassembly: "SET 6 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_d,
        },
        Instruction {
            disassembly: "SET 6 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_e,
        },
        Instruction {
            disassembly: "SET 6 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_h,
        },
        Instruction {
            disassembly: "SET 6 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_l,
        },
        Instruction {
            disassembly: "SET 6 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_hl_ptr,
        },
        Instruction {
            disassembly: "SET 6 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_6_a,
        },
        Instruction {
            disassembly: "SET 7 B",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_b,
        },
        Instruction {
            disassembly: "SET 7 C",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_c,
        },
        Instruction {
            disassembly: "SET 7 D",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_d,
        },
        Instruction {
            disassembly: "SET 7 E",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_e,
        },
        Instruction {
            disassembly: "SET 7 H",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_h,
        },
        Instruction {
            disassembly: "SET 7 L",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_l,
        },
        Instruction {
            disassembly: "SET 7 (HL)",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_hl_ptr,
        },
        Instruction {
            disassembly: "SET 7 A",
            op_len: 2,
            clock_cycles: 2,
            execute: set_7_a,
        },
    ];
}

// Instructions
// ======================================================
// 0x0X Instructions
// ======================================================
fn rlc_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.bc.high & 0b10000000;
    cpu.bc.high = cpu.bc.high << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.bc.high |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn rlc_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.bc.low & 0b10000000;
    cpu.bc.low = cpu.bc.low << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.bc.low |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.bc.low == 0);
}

fn rlc_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.de.high & 0b10000000;
    cpu.de.high = cpu.de.high << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.de.high |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.de.high == 0);
}

fn rlc_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.de.low & 0b10000000;
    cpu.de.low = cpu.de.low << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.de.low |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.de.low == 0);
}

fn rlc_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.hl.high & 0b10000000;
    cpu.hl.high = cpu.hl.high << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.hl.high |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.hl.high == 0);
}

fn rlc_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = cpu.hl.low & 0b10000000;
    cpu.hl.low = cpu.hl.low << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        cpu.hl.low |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.hl.low == 0);
}

fn rlc_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let final_bit = op;
    let mut new_op = op << 1;
    if final_bit == 1 {
        cpu.set_flag('c');
        new_op |= 1;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', new_op == 0);
    bus.set_byte(cpu.hl.get_combined(), new_op);
}

fn rlc_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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
    cpu.update_flag('z', cpu.af.high == 0);
}

fn rrc_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.bc.high & 1;
    cpu.bc.high = cpu.bc.high >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.bc.high |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn rrc_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.bc.low & 1;
    cpu.bc.low = cpu.bc.low >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.bc.low |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.bc.low == 0);
}

fn rrc_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.de.high & 1;
    cpu.de.high = cpu.de.high >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.de.high |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn rrc_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.de.low & 1;
    cpu.de.low = cpu.de.low >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.de.low |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.de.low == 0);
}

fn rrc_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.hl.high & 1;
    cpu.hl.high = cpu.hl.high >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.hl.high |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.hl.high == 0);
}

fn rrc_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = cpu.hl.low & 1;
    cpu.hl.low = cpu.hl.low >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        cpu.hl.low |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', cpu.hl.low == 0);
}

fn rrc_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    let first_bit = op & 1;
    let mut new_op = op >> 1;
    if first_bit == 1 {
        cpu.set_flag('c');
        new_op |= 0b10000000;
    } else {
        cpu.clear_flag('c');
    }
    cpu.update_flag('z', new_op == 0);
    bus.set_byte(cpu.hl.get_combined(), new_op);
}


fn rrc_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
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
    cpu.update_flag('z', cpu.af.high == 0);
}

// ======================================================
// 0x1X Instructions
// ======================================================
fn rl_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.bc.high & 0b1000000;
    cpu.bc.high = cpu.bc.high << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.bc.high |= 1; // set bit 0 to 1
    } else {
        cpu.bc.high &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.bc.high == 0);
    cpu.clear_flag('n');
}

fn rl_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.bc.low & 0b1000000;
    cpu.bc.low = cpu.bc.low << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.bc.low |= 1; // set bit 0 to 1
    } else {
        cpu.bc.low &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.bc.low == 0);
    cpu.clear_flag('n');
}

fn rl_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.de.high & 0b1000000;
    cpu.de.high = cpu.de.high << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.de.high |= 1; // set bit 0 to 1
    } else {
        cpu.de.high &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.de.high == 0);
    cpu.clear_flag('n');
}

fn rl_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.de.low & 0b1000000;
    cpu.de.low = cpu.de.low << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.de.low |= 1; // set bit 0 to 1
    } else {
        cpu.de.low &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.de.low == 0);
    cpu.clear_flag('n');
}

fn rl_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.hl.high & 0b1000000;
    cpu.hl.high = cpu.hl.high << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.hl.high |= 1; // set bit 0 to 1
    } else {
        cpu.hl.high &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.hl.high == 0);
    cpu.clear_flag('n');
}

fn rl_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.hl.low & 0b1000000;
    cpu.hl.low = cpu.hl.low << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.hl.low |= 1; // set bit 0 to 1
    } else {
        cpu.hl.low &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.hl.low == 0);
    cpu.clear_flag('n');
}

fn rl_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let highest_bit = op & 0b1000000;
    let mut new_op = op << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        new_op |= 1;
    } else {
        new_op &= 0b11111110;
    }
    bus.set_byte(cpu.hl.get_combined(), new_op);
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', new_op == 0);
    cpu.clear_flag('n');
}

fn rl_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.af.high & 0b1000000;
    cpu.af.high = cpu.af.high << 1;
    if cpu.extract_flag('c') == true {
        // if carry flag is set
        cpu.af.high |= 1; // set bit 0 to 1
    } else {
        cpu.af.high &= 0b11111110; // else discard first bit just in case
    }
    cpu.update_flag('c', highest_bit == 1);
    cpu.clear_flag('h');
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.clear_flag('n');
}

fn rr_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.bc.high & 1;
    cpu.bc.high = cpu.bc.high >> 1;
    if cpu.extract_flag('c') == true {
        cpu.bc.high |= 0b10000000;
    } else {
        cpu.bc.high &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.bc.high == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.bc.low & 1;
    cpu.bc.low = cpu.bc.low >> 1;
    if cpu.extract_flag('c') == true {
        cpu.bc.low |= 0b10000000;
    } else {
        cpu.bc.low &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.bc.low == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.de.high & 1;
    cpu.de.high = cpu.de.high >> 1;
    if cpu.extract_flag('c') == true {
        cpu.de.high |= 0b10000000;
    } else {
        cpu.de.high &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.de.high == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.de.low & 1;
    cpu.de.low = cpu.de.low >> 1;
    if cpu.extract_flag('c') == true {
        cpu.de.low |= 0b10000000;
    } else {
        cpu.de.low &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.de.low == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.hl.high & 1;
    cpu.hl.high = cpu.hl.high >> 1;
    if cpu.extract_flag('c') == true {
        cpu.hl.high |= 0b10000000;
    } else {
        cpu.hl.high &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.hl.high == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.hl.low & 1;
    cpu.hl.low = cpu.hl.low >> 1;
    if cpu.extract_flag('c') == true {
        cpu.hl.low |= 0b10000000;
    } else {
        cpu.hl.low &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.hl.low == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let lowest_bit = op & 1;
    let mut new_op = op >> 1;
    if cpu.extract_flag('c') == true {
        new_op |= 0b10000000;
    } else {
        new_op &= 0b01111111;
    }
    bus.set_byte(cpu.hl.get_combined(), new_op);
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', new_op == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

fn rr_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.af.high & 1;
    cpu.af.high = cpu.af.high >> 1;
    if cpu.extract_flag('c') == true {
        cpu.af.high |= 0b10000000;
    } else {
        cpu.af.high &= 0b01111111;
    }
    cpu.update_flag('c', lowest_bit == 1);
    cpu.update_flag('z', cpu.af.high == 0);
    cpu.clear_flag('n');
    cpu.clear_flag('h');
}

// ======================================================
// 0x2X Instructions
// ======================================================
fn sla_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.bc.high = cpu.bc.high << 1;
    cpu.bc.high &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn sla_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.bc.low & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.bc.low = cpu.bc.low << 1;
    cpu.bc.low &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.bc.low == 0);
}

fn sla_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.de.high & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.de.high = cpu.de.high << 1;
    cpu.de.high &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.de.high == 0);
}

fn sla_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.de.low & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.de.low = cpu.de.low << 1;
    cpu.de.low &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.de.low == 0);
}

fn sla_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.hl.high & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.hl.high = cpu.hl.high << 1;
    cpu.hl.high &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.hl.high == 0);
}

fn sla_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.hl.low & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.hl.low = cpu.hl.low << 1;
    cpu.hl.low &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.hl.low == 0);
}

fn sla_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let highest_bit = op & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);

    let new_op = (op << 1) & 0b11111110;
    bus.set_byte(cpu.hl.get_combined(), new_op);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', new_op == 0);
}

fn sla_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let highest_bit = cpu.af.high & 0b10000000;
    cpu.update_flag('c', highest_bit != 0);
    cpu.af.high = cpu.af.high << 1;
    cpu.af.high &= 0b11111110;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.af.high == 0);
}

fn sra_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.bc.high & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.bc.high = ((cpu.bc.high >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn sra_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.bc.low & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.bc.low = ((cpu.bc.low >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.bc.low == 0);
}

fn sra_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.de.high & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.de.high = ((cpu.de.high >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.de.high == 0);
}

fn sra_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.de.low & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.de.low = ((cpu.de.low >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.de.low == 0);
}

fn sra_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.hl.high & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.hl.high = ((cpu.hl.high >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.hl.high == 0);
}

fn sra_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.hl.low & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.hl.low = ((cpu.hl.low >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.hl.low == 0);
}

fn sra_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let lowest_bit = op & 1;
    let highest_bit = op & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    let new_op = ((op >> 1) & 0b01111111) | highest_bit;
    bus.set_byte(cpu.hl.get_combined(), new_op);
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', new_op == 0);
}

fn sra_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.af.high & 1;
    let highest_bit = cpu.bc.high & 0b10000000;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.af.high = ((cpu.af.high >> 1) & 0b01111111) | highest_bit;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.af.high == 0);
}

// ======================================================
// 0x3X Instructions
// ======================================================
fn swap_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.bc.high & 0b1111;
    let higher_bits = cpu.bc.high & 0b11110000;
    cpu.bc.high = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn swap_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.bc.low & 0b1111;
    let higher_bits = cpu.bc.low & 0b11110000;
    cpu.bc.low = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.bc.low == 0);
}

fn swap_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.de.high & 0b1111;
    let higher_bits = cpu.de.high & 0b11110000;
    cpu.de.high = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.de.high == 0);
}

fn swap_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.de.low & 0b1111;
    let higher_bits = cpu.de.low & 0b11110000;
    cpu.de.low = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.de.low == 0);
}

fn swap_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.hl.high & 0b1111;
    let higher_bits = cpu.hl.high & 0b11110000;
    cpu.hl.high = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.hl.high == 0);
}

fn swap_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.hl.low & 0b1111;
    let higher_bits = cpu.hl.low & 0b11110000;
    cpu.hl.low = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.hl.low == 0);
}

fn swap_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let lower_bits = op & 0b1111;
    let higher_bits = op & 0b11110000;
    let new_op = (higher_bits >> 4) + (lower_bits << 4);
    bus.set_byte(cpu.hl.get_combined(), new_op);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', new_op == 0);
}

fn swap_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lower_bits = cpu.af.high & 0b1111;
    let higher_bits = cpu.af.high & 0b11110000;
    cpu.af.high = (higher_bits >> 4) + (lower_bits << 4);

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.clear_flag('c');
    cpu.update_flag('z', cpu.af.high == 0);
}

fn srl_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.bc.high & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.bc.high = (cpu.bc.high >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.bc.high == 0);
}

fn srl_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.bc.low & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.bc.low = (cpu.bc.low >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.bc.low == 0);
}

fn srl_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.de.high & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.de.high = (cpu.de.high >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.de.high == 0);
}

fn srl_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.de.low & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.de.low = (cpu.de.low >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.de.low == 0);
}

fn srl_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.hl.high & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.hl.high = (cpu.hl.high >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.hl.high == 0);
}

fn srl_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.hl.low & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.hl.low = (cpu.hl.low >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.hl.low == 0);
}

fn srl_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let lowest_bit = op & 1;
    cpu.update_flag('c', lowest_bit != 0);
    let new_op = (op >> 1) & 0b01111111;
    bus.set_byte(cpu.hl.get_combined(), new_op);
    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', new_op == 0);
}

fn srl_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let lowest_bit = cpu.af.high & 1;
    cpu.update_flag('c', lowest_bit != 0);
    cpu.af.high = (cpu.af.high >> 1) & 0b01111111;

    cpu.clear_flag('h');
    cpu.clear_flag('n');
    cpu.update_flag('z', cpu.af.high == 0);
}

// ======================================================
// 0x4X Instructions
// ======================================================
fn bit_0_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_0_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 1;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_1_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b10;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

// ======================================================
// 0x5X Instructions
// ======================================================
fn bit_2_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_2_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b100;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_3_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b1000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

// ======================================================
// 0x6X Instructions
// ======================================================
fn bit_4_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_4_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b10000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_5_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

// ======================================================
// 0x7X Instructions
// ======================================================
fn bit_6_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_6_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b100000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.high & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.bc.low & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.high & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.de.low & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.high & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.hl.low & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    let bit = op & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

fn bit_7_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    let bit = cpu.af.high & 0b1000000;

    cpu.clear_flag('n');
    cpu.set_flag('h');
    cpu.update_flag('z', bit == 0);
}

// ======================================================
// 0x8X Instructions
// ======================================================
fn res_0_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b11111110;
}

fn res_0_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b11111110;
}

fn res_0_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b11111110;
}

fn res_0_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b11111110;
}

fn res_0_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b11111110;
}

fn res_0_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b11111110;
}

fn res_0_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b11111110);
}

fn res_0_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b11111110;
}

fn res_1_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b11111101;
}

fn res_1_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b11111101;
}

fn res_1_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b11111101;
}

fn res_1_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b11111101;
}

fn res_1_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b11111101;
}

fn res_1_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b11111101;
}

fn res_1_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b11111101);
}

fn res_1_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b11111101;
}

// ======================================================
// 0x9X Instructions
// ======================================================
fn res_2_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b11111011;
}

fn res_2_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b11111011;
}

fn res_2_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b11111011;
}

fn res_2_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b11111011;
}

fn res_2_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b11111011;
}

fn res_2_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b11111011;
}

fn res_2_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b11111011);
}

fn res_2_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b11111011;
}

fn res_3_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b11110111;
}

fn res_3_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b11110111;
}

fn res_3_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b11110111;
}

fn res_3_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b11110111;
}

fn res_3_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b11110111;
}

fn res_3_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b11110111;
}

fn res_3_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b11110111);
}

fn res_3_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b11110111;
}

// ======================================================
// 0xAX Instructions
// ======================================================
fn res_4_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b11101111;
}

fn res_4_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b11101111;
}

fn res_4_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b11101111;
}

fn res_4_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b11101111;
}

fn res_4_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b11101111;
}

fn res_4_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b11101111;
}

fn res_4_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b11101111);
}

fn res_4_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b11101111;
}

fn res_5_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b11011111;
}

fn res_5_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b11011111;
}

fn res_5_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b11011111;
}

fn res_5_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b11011111;
}

fn res_5_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b11011111;
}

fn res_5_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b11011111;
}

fn res_5_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b11011111);
}

fn res_5_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b11011111;
}

// ======================================================
// 0xBX Instructions
// ======================================================
fn res_6_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b10111111;
}

fn res_6_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b10111111;
}

fn res_6_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b10111111;
}

fn res_6_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b10111111;
}

fn res_6_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b10111111;
}

fn res_6_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b10111111;
}

fn res_6_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b10111111);
}

fn res_6_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b10111111;
}

fn res_7_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high &= 0b01111111;
}

fn res_7_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low &= 0b01111111;
}

fn res_7_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high &= 0b01111111;
}

fn res_7_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low &= 0b01111111;
}

fn res_7_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high &= 0b01111111;
}

fn res_7_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low &= 0b01111111;
}

fn res_7_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op & 0b01111111);
}

fn res_7_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.high &= 0b01111111;
}

// ======================================================
// 0xCX Instructions
// ======================================================
fn set_0_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 1;
}

fn set_0_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 1;
}

fn set_0_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 1;
}

fn set_0_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 1;
}

fn set_0_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 1;
}

fn set_0_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 1;
}

fn set_0_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 1);
}

fn set_0_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 1;
}

fn set_1_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b10;
}

fn set_1_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b10;
}

fn set_1_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b10;
}

fn set_1_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b10;
}

fn set_1_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b10;
}

fn set_1_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b10;
}

fn set_1_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b10);
}

fn set_1_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b10;
}

// ======================================================
// 0xDX Instructions
// ======================================================
fn set_2_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b100;
}

fn set_2_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b100;
}

fn set_2_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b100;
}

fn set_2_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b100;
}

fn set_2_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b100;
}

fn set_2_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b100;
}

fn set_2_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b100);
}

fn set_2_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b100;
}

fn set_3_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b1000;
}

fn set_3_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b1000;
}

fn set_3_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b1000;
}

fn set_3_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b1000;
}

fn set_3_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b1000;
}

fn set_3_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b1000;
}

fn set_3_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b1000);
}

fn set_3_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b1000;
}

// ======================================================
// 0xEX Instructions
// ======================================================
fn set_4_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b10000;
}

fn set_4_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b10000;
}

fn set_4_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b10000;
}

fn set_4_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b10000;
}

fn set_4_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b10000;
}

fn set_4_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b10000;
}

fn set_4_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b10000);
}

fn set_4_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b10000;
}

fn set_5_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b100000;
}

fn set_5_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b100000;
}

fn set_5_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b100000;
}

fn set_5_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b100000;
}

fn set_5_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b100000;
}

fn set_5_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b100000;
}

fn set_5_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b100000);
}

fn set_5_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b100000;
}

// ======================================================
// 0xFX Instructions
// ======================================================
fn set_6_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b1000000;
}

fn set_6_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b1000000;
}

fn set_6_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b1000000;
}

fn set_6_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b1000000;
}

fn set_6_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b1000000;
}

fn set_6_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b1000000;
}

fn set_6_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b1000000);
}

fn set_6_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b1000000;
}

fn set_7_b(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.high |= 0b10000000;
}

fn set_7_c(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.bc.low |= 0b10000000;
}

fn set_7_d(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.high |= 0b10000000;
}

fn set_7_e(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.de.low |= 0b10000000;
}

fn set_7_h(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.high |= 0b10000000;
}

fn set_7_l(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.hl.low |= 0b10000000;
}

fn set_7_hl_ptr(cpu: &mut cpu::CPU, bus: &mut bus::Bus) {
    let op = bus.fetch_byte(cpu.hl.get_combined());
    bus.set_byte(cpu.hl.get_combined(), op | 0b10000000);
}

fn set_7_a(cpu: &mut cpu::CPU, _: &mut bus::Bus) {
    cpu.af.low |= 0b10000000;
}