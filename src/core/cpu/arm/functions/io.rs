use super::super::super::ArmCpu;
use super::super::super::alu::*;

fn sdt_reg_operands(cpu: &ArmCpu, instr: u32) -> (u32, u32) {
	let rm = instr & 0xf;
	// If a register is used to specify the shift amount the PC will be 12 bytes ahead.
	let _rm = if rm == 15 { cpu.rget(15) + 4 } else { cpu.rget(rm) };
	// Only the least significant byte of the contents of Rs is used to determine the shift amount. 
	// Rs can be any general register other than R15.
	let _rs = cpu.rget((instr >> 8) & 0xf) & 0xf;
	(_rm, _rs)
}

pub fn arm_fn_ldrb(cpu: &mut ArmCpu, address: u32, dest: u32) {
	let data = cpu.mread8(address) as u32;
	cpu.rset(dest, data);
}

pub fn arm_fn_ldr(cpu: &mut ArmCpu, address: u32, dest: u32) {
	let data = cpu.mread32(address);
	cpu.rset(dest, data);
}

pub fn arm_fn_strb(cpu: &mut ArmCpu, address: u32, src: u32) {
	// When R15 is the source register (Rd) of a register store (STR) instruction, 
	// the stored value will be address of the instruction plus 12
	let _src = if src == 15 {
		cpu.rget(src) + 4
	} else {
		cpu.rget(src)
	};
	let data = (_src & 0xff) as u8;
	cpu.mwrite8(address, data);

}

pub fn arm_fn_str(cpu: &mut ArmCpu, address: u32, src: u32) {
	// When R15 is the source register (Rd) of a register store (STR) instruction, 
	// the stored value will be address of the instruction plus 12
	let _src = if src == 15 {
		cpu.rget(src) + 4
	} else {
		cpu.rget(src)
	};
	let data = (_src & 0xff);
	cpu.mwrite32(address, data);
}

// the neg/pos versions of these functions
// are just used for the instructions that do not write back
// There is still some confusion so I'm keeping them for now
// and removing them when I have more information.
// They do the exact same thing as their non neg/pos counterparts though.

pub fn arm_fn_sdt_imm(cpu: &ArmCpu, instr: u32) -> u32 {
	instr & 0xFFF
}

pub fn arm_fn_sdt_neg_imm(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_imm(cpu, instr)
}

pub fn arm_fn_sdt_pos_imm(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_imm(cpu, instr)
}

pub fn arm_fn_sdt_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_lli(lhs, rhs)
}

pub fn arm_fn_sdt_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_lri(lhs, rhs)
}

pub fn arm_fn_sdt_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_ari(lhs, rhs)
}

pub fn arm_fn_sdt_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	let (lhs, rhs) = sdt_reg_operands(cpu, instr);
	arm_alu_rri(cpu, lhs, rhs)
}

pub fn arm_fn_sdt_pos_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsl(cpu, instr)
}

pub fn arm_fn_sdt_pos_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsr(cpu, instr)
}

pub fn arm_fn_sdt_pos_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_asr(cpu, instr)
}

pub fn arm_fn_sdt_pos_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_ror(cpu, instr)
}

pub fn arm_fn_sdt_neg_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsl(cpu, instr)
}

pub fn arm_fn_sdt_neg_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_lsr(cpu, instr)
}

pub fn arm_fn_sdt_neg_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_asr(cpu, instr)
}

pub fn arm_fn_sdt_neg_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	arm_fn_sdt_ror(cpu, instr)
}