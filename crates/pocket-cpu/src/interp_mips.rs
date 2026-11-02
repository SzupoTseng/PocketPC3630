//! MIPS for the portable interpreter.
//!
//! Pocket PC 2000 software also shipped as MIPS builds for machines such as the
//! Casio Cassiopeia E-115/E-125 (NEC VR41xx). Windows CE runs them as 32-bit,
//! little-endian MIPS II code without a floating-point unit, so this covers the
//! integer instruction set, branch delay slots and unaligned loads and stores.
//! MIPS16, the compressed encoding some VR41xx builds use, is reported as
//! unsupported. `tests/interp_diff.rs` checks this against Unicorn's MIPS.

use super::{bad, InterpCpu, DEFAULT_SLICE, IMPLICIT_CALL_HIGH, IMPLICIT_CALL_LOW, PAGE_MASK, PAGE_SHIFT, RETIRED};
use crate::{regs::ArmReg, CpuError, StopReason};
use std::sync::atomic::Ordering;

/// MIPS registers the kernel names with ARM register names, the same mapping
/// the Unicorn backend uses: arguments, saved registers, then gp, sp, ra.
pub(super) fn index_of(reg: ArmReg) -> Option<usize> {
    use ArmReg::*;
    Some(match reg {
        R0 => 4,
        R1 => 5,
        R2 => 6,
        R3 => 7,
        R4 => 16,
        R5 => 17,
        R6 => 18,
        R7 => 19,
        R8 => 20,
        R9 => 21,
        R10 => 22,
        R11 => 23,
        R12 => 28,
        Sp => 29,
        Lr => 31,
        Pc | Cpsr => return None,
    })
}

#[derive(Default)]
pub(super) struct MipsState {
    pub gpr: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    pub pc: u32,
    /// What the kernel stores in the status slot; MIPS has no flags register.
    pub status: u32,
}

/// Where execution goes after an instruction.
enum Flow {
    Next,
    /// A taken branch or jump: the delay slot runs, then the target.
    Branch(u32),
    /// A branch-likely that was not taken skips its delay slot.
    SkipDelaySlot,
}

fn trap(kind: &str, pc: u32) -> CpuError {
    CpuError::Backend(format!("emu_start: EXCEPTION ({kind}) at guest address 0x{pc:08x}"))
}

impl InterpCpu {
    #[inline(always)]
    fn g(&self, n: u32) -> u32 {
        self.mips.gpr[n as usize]
    }

    #[inline(always)]
    fn set(&mut self, n: u32, value: u32) {
        if n != 0 {
            self.mips.gpr[n as usize] = value;
        }
    }

    fn fetch(&self, pc: u32) -> Result<u32, CpuError> {
        if pc & 3 != 0 {
            return Err(trap("FETCH unaligned", pc));
        }
        match &self.pages[(pc >> PAGE_SHIFT) as usize] {
            Some(page) => {
                let at = (pc & PAGE_MASK) as usize;
                Ok(u32::from_le_bytes([page[at], page[at + 1], page[at + 2], page[at + 3]]))
            }
            None => Err(bad("FETCH unmapped", pc)),
        }
    }

    fn mips_step(&mut self, i: u32, pc: u32) -> Result<Flow, CpuError> {
        let op = i >> 26;
        let rs = (i >> 21) & 31;
        let rt = (i >> 16) & 31;
        let rd = (i >> 11) & 31;
        let sa = (i >> 6) & 31;
        let imm = i & 0xFFFF;
        let simm = imm as u16 as i16 as i32 as u32;
        let branch_target = pc.wrapping_add(4).wrapping_add(simm << 2);
        let (s, t) = (self.g(rs), self.g(rt));
        let branch = |taken: bool, likely: bool| {
            if taken {
                Flow::Branch(branch_target)
            } else if likely {
                Flow::SkipDelaySlot
            } else {
                Flow::Next
            }
        };
        let address = s.wrapping_add(simm);
        Ok(match op {
            0 => match i & 63 {
                0 => {
                    self.set(rd, t << sa);
                    Flow::Next
                }
                2 => {
                    self.set(rd, t >> sa);
                    Flow::Next
                }
                3 => {
                    self.set(rd, ((t as i32) >> sa) as u32);
                    Flow::Next
                }
                4 => {
                    self.set(rd, t << (s & 31));
                    Flow::Next
                }
                6 => {
                    self.set(rd, t >> (s & 31));
                    Flow::Next
                }
                7 => {
                    self.set(rd, ((t as i32) >> (s & 31)) as u32);
                    Flow::Next
                }
                8 => Flow::Branch(s),
                9 => {
                    self.set(rd, pc.wrapping_add(8));
                    Flow::Branch(s)
                }
                0x0C => return Err(trap("syscall", pc)),
                0x0D => return Err(trap("break", pc)),
                0x0F => Flow::Next,
                0x10 => {
                    self.set(rd, self.mips.hi);
                    Flow::Next
                }
                0x11 => {
                    self.mips.hi = s;
                    Flow::Next
                }
                0x12 => {
                    self.set(rd, self.mips.lo);
                    Flow::Next
                }
                0x13 => {
                    self.mips.lo = s;
                    Flow::Next
                }
                0x18 => {
                    let product = i64::from(s as i32) * i64::from(t as i32);
                    self.mips.lo = product as u32;
                    self.mips.hi = (product >> 32) as u32;
                    Flow::Next
                }
                0x19 => {
                    let product = u64::from(s) * u64::from(t);
                    self.mips.lo = product as u32;
                    self.mips.hi = (product >> 32) as u32;
                    Flow::Next
                }
                0x1A => {
                    // Division by zero leaves HI and LO unpredictable; the
                    // compiler guards it with a trap, so leave them alone.
                    if t != 0 {
                        let (a, b) = (s as i32, t as i32);
                        self.mips.lo = a.wrapping_div(b) as u32;
                        self.mips.hi = a.wrapping_rem(b) as u32;
                    }
                    Flow::Next
                }
                0x1B => {
                    if t != 0 {
                        self.mips.lo = s / t;
                        self.mips.hi = s % t;
                    }
                    Flow::Next
                }
                0x20 => {
                    let (sum, overflow) = (s as i32).overflowing_add(t as i32);
                    if overflow {
                        return Err(trap("integer overflow", pc));
                    }
                    self.set(rd, sum as u32);
                    Flow::Next
                }
                0x21 => {
                    self.set(rd, s.wrapping_add(t));
                    Flow::Next
                }
                0x22 => {
                    let (difference, overflow) = (s as i32).overflowing_sub(t as i32);
                    if overflow {
                        return Err(trap("integer overflow", pc));
                    }
                    self.set(rd, difference as u32);
                    Flow::Next
                }
                0x23 => {
                    self.set(rd, s.wrapping_sub(t));
                    Flow::Next
                }
                0x24 => {
                    self.set(rd, s & t);
                    Flow::Next
                }
                0x25 => {
                    self.set(rd, s | t);
                    Flow::Next
                }
                0x26 => {
                    self.set(rd, s ^ t);
                    Flow::Next
                }
                0x27 => {
                    self.set(rd, !(s | t));
                    Flow::Next
                }
                0x2A => {
                    self.set(rd, u32::from((s as i32) < (t as i32)));
                    Flow::Next
                }
                0x2B => {
                    self.set(rd, u32::from(s < t));
                    Flow::Next
                }
                0x30..=0x36 => {
                    let taken = match i & 63 {
                        0x30 => (s as i32) >= (t as i32),
                        0x31 => s >= t,
                        0x32 => (s as i32) < (t as i32),
                        0x33 => s < t,
                        0x34 => s == t,
                        0x36 => s != t,
                        _ => return Err(trap("reserved instruction", pc)),
                    };
                    if taken {
                        return Err(trap("trap", pc));
                    }
                    Flow::Next
                }
                _ => return Err(trap("reserved instruction", pc)),
            },
            1 => {
                let negative = (s as i32) < 0;
                match rt {
                    0 => branch(negative, false),
                    1 => branch(!negative, false),
                    2 => branch(negative, true),
                    3 => branch(!negative, true),
                    0x10..=0x13 => {
                        let taken = if rt & 1 == 0 { negative } else { !negative };
                        self.set(31, pc.wrapping_add(8));
                        branch(taken, rt & 2 != 0)
                    }
                    0x08..=0x0E => {
                        let (a, b) = (s, simm);
                        let taken = match rt {
                            0x08 => (a as i32) >= (b as i32),
                            0x09 => a >= b,
                            0x0A => (a as i32) < (b as i32),
                            0x0B => a < b,
                            0x0C => a == b,
                            0x0E => a != b,
                            _ => return Err(trap("reserved instruction", pc)),
                        };
                        if taken {
                            return Err(trap("trap", pc));
                        }
                        Flow::Next
                    }
                    _ => return Err(trap("reserved instruction", pc)),
                }
            }
            2 | 3 => {
                if op == 3 {
                    self.set(31, pc.wrapping_add(8));
                }
                Flow::Branch((pc.wrapping_add(4) & 0xF000_0000) | ((i & 0x03FF_FFFF) << 2))
            }
            4 => branch(s == t, false),
            5 => branch(s != t, false),
            6 => branch((s as i32) <= 0, false),
            7 => branch((s as i32) > 0, false),
            0x14 => branch(s == t, true),
            0x15 => branch(s != t, true),
            0x16 => branch((s as i32) <= 0, true),
            0x17 => branch((s as i32) > 0, true),
            8 => {
                let (sum, overflow) = (s as i32).overflowing_add(simm as i32);
                if overflow {
                    return Err(trap("integer overflow", pc));
                }
                self.set(rt, sum as u32);
                Flow::Next
            }
            9 => {
                self.set(rt, s.wrapping_add(simm));
                Flow::Next
            }
            0x0A => {
                self.set(rt, u32::from((s as i32) < (simm as i32)));
                Flow::Next
            }
            0x0B => {
                self.set(rt, u32::from(s < simm));
                Flow::Next
            }
            0x0C => {
                self.set(rt, s & imm);
                Flow::Next
            }
            0x0D => {
                self.set(rt, s | imm);
                Flow::Next
            }
            0x0E => {
                self.set(rt, s ^ imm);
                Flow::Next
            }
            0x0F => {
                self.set(rt, imm << 16);
                Flow::Next
            }
            0x1D => return Err(CpuError::Unsupported("MIPS16 code (JALX)")),
            0x20 => {
                let value = self.read8(address)? as i8 as i32 as u32;
                self.set(rt, value);
                Flow::Next
            }
            0x24 => {
                let value = u32::from(self.read8(address)?);
                self.set(rt, value);
                Flow::Next
            }
            0x21 | 0x25 => {
                if address & 1 != 0 {
                    return Err(trap("READ unaligned", address));
                }
                let half = self.read16(address)?;
                let value = if op == 0x21 { half as i16 as i32 as u32 } else { u32::from(half) };
                self.set(rt, value);
                Flow::Next
            }
            0x23 | 0x30 => {
                if address & 3 != 0 {
                    return Err(trap("READ unaligned", address));
                }
                let value = self.read32(address)?;
                self.set(rt, value);
                Flow::Next
            }
            0x22 => {
                // LWL, little-endian: the bytes from the aligned word's start up
                // to `address` fill the top of the register.
                let byte = address & 3;
                let word = self.read32(address & !3)?;
                let shift = 8 * (3 - byte);
                let keep = if byte == 3 { 0 } else { u32::MAX >> (8 * (byte + 1)) };
                self.set(rt, (t & keep) | (word << shift));
                Flow::Next
            }
            0x26 => {
                // LWR, little-endian: the bytes from `address` to the aligned
                // word's end fill the bottom of the register.
                let byte = address & 3;
                let word = self.read32(address & !3)?;
                let keep = if byte == 0 { 0 } else { !(u32::MAX >> (8 * byte)) };
                self.set(rt, (t & keep) | (word >> (8 * byte)));
                Flow::Next
            }
            0x28 => {
                self.write8(address, t as u8)?;
                Flow::Next
            }
            0x29 => {
                if address & 1 != 0 {
                    return Err(trap("WRITE unaligned", address));
                }
                self.write8(address, t as u8)?;
                self.write8(address.wrapping_add(1), (t >> 8) as u8)?;
                Flow::Next
            }
            0x2B | 0x38 => {
                if address & 3 != 0 {
                    return Err(trap("WRITE unaligned", address));
                }
                self.write32(address, t)?;
                if op == 0x38 {
                    // SC on a single processor always succeeds.
                    self.set(rt, 1);
                }
                Flow::Next
            }
            0x2A => {
                // SWL, little-endian: the top of the register goes to the
                // aligned word's start up to `address`.
                let byte = address & 3;
                let at = address & !3;
                let word = self.read32(at)?;
                let shift = 8 * (3 - byte);
                let keep = if byte == 3 { 0 } else { !(u32::MAX >> shift) };
                self.write32(at, (word & keep) | (t >> shift))?;
                Flow::Next
            }
            0x2E => {
                // SWR, little-endian: the bottom of the register goes to
                // `address` up to the aligned word's end.
                let byte = address & 3;
                let at = address & !3;
                let word = self.read32(at)?;
                let keep = if byte == 0 { 0 } else { u32::MAX >> (8 * (4 - byte)) };
                self.write32(at, (word & keep) | (t << (8 * byte)))?;
                Flow::Next
            }
            0x2F | 0x33 => Flow::Next,
            _ => return Err(trap("reserved instruction", pc)),
        })
    }

    pub(super) fn run_mips(&mut self, start_va: u32, max_instructions: u64) -> Result<StopReason, CpuError> {
        self.stop = false;
        let budget = if max_instructions == 0 { DEFAULT_SLICE } else { max_instructions };
        let mut pc = start_va;
        let mut executed = 0u64;
        let result = 'run: loop {
            if executed >= budget {
                break 'run Ok(StopReason::InstructionLimit);
            }
            if self.is_hook(pc) {
                break 'run Ok(StopReason::Hook(pc));
            }
            // An implicit kernel call: MIPS and SuperH Windows CE reach
            // some kernel services by jumping to an address just below
            // 0xFFFF_FE01, which faults on a device and is dispatched by
            // the kernel. Pocket Slay's C runtime ends the program this
            // way (`jalr` to 0xFFFF_F3FA). Return to the caller, as the
            // ARM trap page does, with the first argument left as the
            // result.
            if (IMPLICIT_CALL_LOW..=IMPLICIT_CALL_HIGH).contains(&pc) {
                self.mips.gpr[2] = self.mips.gpr[4];
                pc = self.mips.gpr[31];
                continue;
            }
            let instruction = match self.fetch(pc) {
                Ok(i) => i,
                Err(error) => break 'run Err(error),
            };
            executed += 1;
            match self.mips_step(instruction, pc) {
                Ok(Flow::Next) => pc = pc.wrapping_add(4),
                Ok(Flow::SkipDelaySlot) => pc = pc.wrapping_add(8),
                Ok(Flow::Branch(target)) => {
                    // The delay slot runs before the branch lands. A branch in
                    // a delay slot is undefined, so it is refused.
                    let slot_pc = pc.wrapping_add(4);
                    let slot = match self.fetch(slot_pc) {
                        Ok(i) => i,
                        Err(error) => break 'run Err(error),
                    };
                    executed += 1;
                    match self.mips_step(slot, slot_pc) {
                        Ok(Flow::Next) => pc = target,
                        Ok(_) => break 'run Err(trap("branch in a delay slot", slot_pc)),
                        Err(error) => break 'run Err(error),
                    }
                }
                Err(error) => break 'run Err(error),
            }
            if self.stop {
                break 'run Ok(StopReason::Requested);
            }
        };
        self.mips.pc = pc;
        RETIRED.fetch_add(executed, Ordering::Relaxed);
        result
    }
}
