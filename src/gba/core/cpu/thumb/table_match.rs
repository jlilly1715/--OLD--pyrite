use super::super::ArmCpu;
// use super::super::super::memory::GbaMemory;
use super::thumb_impl::*;
use super::thumb_dp_impl::*;

pub fn to_index(row: u32, column: u32) -> u32 {
	(row * 16) + column
}

pub fn to_index_dp(row: u32, column: u32) -> u32 {
	(row * 4) + column
}

pub fn run_instr(cpu: &mut ArmCpu, instr_idx: u32, instr: u32) {
	match instr_idx {
		0x0000 ... 0x0007 => { thumb_lsl_imm(cpu, instr); },
		0x0008 ... 0x000f => { thumb_lsr_imm(cpu, instr); },
		0x0010 ... 0x0017 => { thumb_asr_imm(cpu, instr); },
		0x0018 ... 0x0019 => { thumb_add_reg(cpu, instr); },
		0x001a ... 0x001b => { thumb_sub_reg(cpu, instr); },
		0x001c ... 0x001d => { thumb_add_imm3(cpu, instr); },
		0x001e ... 0x001f => { thumb_sub_imm3(cpu, instr); },
		0x0020 => { thumb_mov_i8r0(cpu, instr); },
		0x0021 => { thumb_mov_i8r1(cpu, instr); },
		0x0022 => { thumb_mov_i8r2(cpu, instr); },
		0x0023 => { thumb_mov_i8r3(cpu, instr); },
		0x0024 => { thumb_mov_i8r4(cpu, instr); },
		0x0025 => { thumb_mov_i8r5(cpu, instr); },
		0x0026 => { thumb_mov_i8r6(cpu, instr); },
		0x0027 => { thumb_mov_i8r7(cpu, instr); },
		0x0028 => { thumb_cmp_i8r0(cpu, instr); },
		0x0029 => { thumb_cmp_i8r1(cpu, instr); },
		0x002a => { thumb_cmp_i8r2(cpu, instr); },
		0x002b => { thumb_cmp_i8r3(cpu, instr); },
		0x002c => { thumb_cmp_i8r4(cpu, instr); },
		0x002d => { thumb_cmp_i8r5(cpu, instr); },
		0x002e => { thumb_cmp_i8r6(cpu, instr); },
		0x002f => { thumb_cmp_i8r7(cpu, instr); },
		0x0030 => { thumb_add_i8r0(cpu, instr); },
		0x0031 => { thumb_add_i8r1(cpu, instr); },
		0x0032 => { thumb_add_i8r2(cpu, instr); },
		0x0033 => { thumb_add_i8r3(cpu, instr); },
		0x0034 => { thumb_add_i8r4(cpu, instr); },
		0x0035 => { thumb_add_i8r5(cpu, instr); },
		0x0036 => { thumb_add_i8r6(cpu, instr); },
		0x0037 => { thumb_add_i8r7(cpu, instr); },
		0x0038 => { thumb_sub_i8r0(cpu, instr); },
		0x0039 => { thumb_sub_i8r1(cpu, instr); },
		0x003a => { thumb_sub_i8r2(cpu, instr); },
		0x003b => { thumb_sub_i8r3(cpu, instr); },
		0x003c => { thumb_sub_i8r4(cpu, instr); },
		0x003d => { thumb_sub_i8r5(cpu, instr); },
		0x003e => { thumb_sub_i8r6(cpu, instr); },
		0x003f => { thumb_sub_i8r7(cpu, instr); },
		0x0040 => { thumb_dp_g1(cpu, instr); },
		0x0041 => { thumb_dp_g2(cpu, instr); },
		0x0042 => { thumb_dp_g3(cpu, instr); },
		0x0043 => { thumb_dp_g4(cpu, instr); },
		0x0044 => { thumb_addh(cpu, instr); },
		0x0045 => { thumb_cmph(cpu, instr); },
		0x0046 => { thumb_movh(cpu, instr); },
		0x0047 => { thumb_bx_reg(cpu, instr); },
		0x0048 => { thumb_ldrpc_r0(cpu, instr); },
		0x0049 => { thumb_ldrpc_r1(cpu, instr); },
		0x004a => { thumb_ldrpc_r2(cpu, instr); },
		0x004b => { thumb_ldrpc_r3(cpu, instr); },
		0x004c => { thumb_ldrpc_r4(cpu, instr); },
		0x004d => { thumb_ldrpc_r5(cpu, instr); },
		0x004e => { thumb_ldrpc_r6(cpu, instr); },
		0x004f => { thumb_ldrpc_r7(cpu, instr); },
		0x0050 ... 0x0051 => { thumb_str_reg(cpu, instr); },
		0x0052 ... 0x0053 => { thumb_strh_reg(cpu, instr); },
		0x0054 ... 0x0055 => { thumb_strb_reg(cpu, instr); },
		0x0056 ... 0x0057 => { thumb_ldrsb_reg(cpu, instr); },
		0x0058 ... 0x0059 => { thumb_ldr_reg(cpu, instr); },
		0x005a ... 0x005b => { thumb_ldrh_reg(cpu, instr); },
		0x005c ... 0x005d => { thumb_ldrb_reg(cpu, instr); },
		0x005e ... 0x005f => { thumb_ldrsh_reg(cpu, instr); },
		0x0060 ... 0x0067 => { thumb_str_imm5(cpu, instr); },
		0x0068 ... 0x006f => { thumb_ldr_imm5(cpu, instr); },
		0x0070 ... 0x0077 => { thumb_strb_imm5(cpu, instr); },
		0x0078 ... 0x007f => { thumb_ldrb_imm5(cpu, instr); },
		0x0080 ... 0x0087 => { thumb_strh_imm5(cpu, instr); },
		0x0088 ... 0x008f => { thumb_ldrh_imm5(cpu, instr); },
		0x0090 => { thumb_strsp_r0(cpu, instr); },
		0x0091 => { thumb_strsp_r1(cpu, instr); },
		0x0092 => { thumb_strsp_r2(cpu, instr); },
		0x0093 => { thumb_strsp_r3(cpu, instr); },
		0x0094 => { thumb_strsp_r4(cpu, instr); },
		0x0095 => { thumb_strsp_r5(cpu, instr); },
		0x0096 => { thumb_strsp_r6(cpu, instr); },
		0x0097 => { thumb_strsp_r7(cpu, instr); },
		0x0098 => { thumb_ldrsp_r0(cpu, instr); },
		0x0099 => { thumb_ldrsp_r1(cpu, instr); },
		0x009a => { thumb_ldrsp_r2(cpu, instr); },
		0x009b => { thumb_ldrsp_r3(cpu, instr); },
		0x009c => { thumb_ldrsp_r4(cpu, instr); },
		0x009d => { thumb_ldrsp_r5(cpu, instr); },
		0x009e => { thumb_ldrsp_r6(cpu, instr); },
		0x009f => { thumb_ldrsp_r7(cpu, instr); },
		0x00a0 => { thumb_addpc_r0(cpu, instr); },
		0x00a1 => { thumb_addpc_r1(cpu, instr); },
		0x00a2 => { thumb_addpc_r2(cpu, instr); },
		0x00a3 => { thumb_addpc_r3(cpu, instr); },
		0x00a4 => { thumb_addpc_r4(cpu, instr); },
		0x00a5 => { thumb_addpc_r5(cpu, instr); },
		0x00a6 => { thumb_addpc_r6(cpu, instr); },
		0x00a7 => { thumb_addpc_r7(cpu, instr); },
		0x00a8 => { thumb_addsp_r0(cpu, instr); },
		0x00a9 => { thumb_addsp_r1(cpu, instr); },
		0x00aa => { thumb_addsp_r2(cpu, instr); },
		0x00ab => { thumb_addsp_r3(cpu, instr); },
		0x00ac => { thumb_addsp_r4(cpu, instr); },
		0x00ad => { thumb_addsp_r5(cpu, instr); },
		0x00ae => { thumb_addsp_r6(cpu, instr); },
		0x00af => { thumb_addsp_r7(cpu, instr); },
		0x00b0 => { thumb_addsp_imm7(cpu, instr); },
		0x00b1 ... 0x00b3 | 0x00b6 ... 0x00bb | 0x00be ... 0x00bf | 0x00de | 0x00e8 ... 0x00ef => { thumb_undefined(cpu, instr); },
		0x00b4 => { thumb_push(cpu, instr); },
		0x00b5 => { thumb_push_lr(cpu, instr); },
		0x00bc => { thumb_pop(cpu, instr); },
		0x00bd => { thumb_pop_pc(cpu, instr); },
		0x00c0 => { thumb_stmia_r0(cpu, instr); },
		0x00c1 => { thumb_stmia_r1(cpu, instr); },
		0x00c2 => { thumb_stmia_r2(cpu, instr); },
		0x00c3 => { thumb_stmia_r3(cpu, instr); },
		0x00c4 => { thumb_stmia_r4(cpu, instr); },
		0x00c5 => { thumb_stmia_r5(cpu, instr); },
		0x00c6 => { thumb_stmia_r6(cpu, instr); },
		0x00c7 => { thumb_stmia_r7(cpu, instr); },
		0x00c8 => { thumb_ldmia_r0(cpu, instr); },
		0x00c9 => { thumb_ldmia_r1(cpu, instr); },
		0x00ca => { thumb_ldmia_r2(cpu, instr); },
		0x00cb => { thumb_ldmia_r3(cpu, instr); },
		0x00cc => { thumb_ldmia_r4(cpu, instr); },
		0x00cd => { thumb_ldmia_r5(cpu, instr); },
		0x00ce => { thumb_ldmia_r6(cpu, instr); },
		0x00cf => { thumb_ldmia_r7(cpu, instr); },
		0x00d0 => { thumb_beq(cpu, instr); },
		0x00d1 => { thumb_bne(cpu, instr); },
		0x00d2 => { thumb_bcs(cpu, instr); },
		0x00d3 => { thumb_bcc(cpu, instr); },
		0x00d4 => { thumb_bmi(cpu, instr); },
		0x00d5 => { thumb_bpl(cpu, instr); },
		0x00d6 => { thumb_bvs(cpu, instr); },
		0x00d7 => { thumb_bvc(cpu, instr); },
		0x00d8 => { thumb_bhi(cpu, instr); },
		0x00d9 => { thumb_bls(cpu, instr); },
		0x00da => { thumb_bge(cpu, instr); },
		0x00db => { thumb_blt(cpu, instr); },
		0x00dc => { thumb_bgt(cpu, instr); },
		0x00dd => { thumb_ble(cpu, instr); },
		0x00df => { thumb_swi(cpu, instr); },
		0x00e0 ... 0x00e7 => { thumb_b(cpu, instr); },
		0x00f0 ... 0x00f7 => { thumb_bl_setup(cpu, instr); },
		0x00f8 ... 0x00ff => { thumb_bl_off(cpu, instr); },
		_ => { unreachable!() }
	}
}

pub fn run_instr_dp(cpu: &mut ArmCpu, instr_idx: u32, instr: u32) {
	match instr_idx {
		0x0000 => { thumb_dp_and(cpu, instr); },
		0x0001 => { thumb_dp_eor(cpu, instr); },
		0x0002 => { thumb_dp_lsl(cpu, instr); },
		0x0003 => { thumb_dp_lsr(cpu, instr); },
		0x0004 => { thumb_dp_asr(cpu, instr); },
		0x0005 => { thumb_dp_adc(cpu, instr); },
		0x0006 => { thumb_dp_sbc(cpu, instr); },
		0x0007 => { thumb_dp_ror(cpu, instr); },
		0x0008 => { thumb_dp_tst(cpu, instr); },
		0x0009 => { thumb_dp_neg(cpu, instr); },
		0x000a => { thumb_dp_cmp(cpu, instr); },
		0x000b => { thumb_dp_cmn(cpu, instr); },
		0x000c => { thumb_dp_orr(cpu, instr); },
		0x000d => { thumb_dp_mul(cpu, instr); },
		0x000e => { thumb_dp_bic(cpu, instr); },
		0x000f => { thumb_dp_mvn(cpu, instr); },
		_ => { unreachable!() }
	}
}