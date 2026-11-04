//! A portable ARMv4 interpreter.
//!
//! Unicorn generates host machine code, which WebAssembly cannot execute, so
//! the browser build of EverQuest needs a backend written purely in Rust.
//! Pocket PC 2002 titles are plain ARMv4 with no Thumb or VFP, so this stays
//! small. `tests/interp_diff.rs` checks it against Unicorn instruction by
//! instruction.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::{regs::ArmReg, Arch, Cpu, CpuError, Intrinsic, Prot, StopReason};

/// The range MIPS and SuperH Windows CE use for implicit kernel calls:
/// `FIRST_METHOD` (0xFFFF_FE01) minus the method number, two bytes each.
const IMPLICIT_CALL_LOW: u32 = 0xFFFF_0000;
const IMPLICIT_CALL_HIGH: u32 = 0xFFFF_FE01;

#[path = "interp_mips.rs"]
mod mips;
#[path = "interp_sh.rs"]
mod sh;

const PAGE_SHIFT: u32 = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PAGE_MASK: u32 = PAGE_SIZE as u32 - 1;
const PAGE_COUNT: usize = 1 << (32 - PAGE_SHIFT);
/// Instructions per slice when the caller sets no limit, so the host still
/// gets to poll input during a long stretch without an API call.
const DEFAULT_SLICE: u64 = 4_000_000;
/// Mode bits a user-mode Windows CE process starts with, as Unicorn reports.
const INITIAL_CPSR_LOW: u32 = 0x1D3;

type Page = Box<[u8; PAGE_SIZE]>;

/// Guest instructions executed by every interpreter, for performance reports.
pub static RETIRED: AtomicU64 = AtomicU64::new(0);

pub struct InterpCpu {
    pages: Vec<Option<Page>>,
    r: [u32; 16],
    n: bool,
    z: bool,
    c: bool,
    v: bool,
    cpsr_low: u32,
    hook_pages: Vec<u64>,
    hook_ranges: Vec<(u32, u32)>,
    intrinsics: Vec<(u32, Intrinsic)>,
    stop: bool,
    arch: Arch,
    mips: mips::MipsState,
    sh: sh::ShState,
}

impl Default for InterpCpu {
    fn default() -> Self {
        Self::new()
    }
}

fn bad(kind: &str, va: u32) -> CpuError {
    CpuError::Backend(format!(
        "emu_start: EXCEPTION ({kind}) at guest address 0x{va:08x}"
    ))
}

fn add_flags(a: u32, b: u32, carry_in: bool) -> (u32, bool, bool) {
    let wide = u64::from(a) + u64::from(b) + u64::from(carry_in);
    let result = wide as u32;
    let overflow = ((a ^ result) & (b ^ result)) >> 31 != 0;
    (result, wide >> 32 != 0, overflow)
}

impl InterpCpu {
    pub fn new() -> Self {
        Self {
            pages: (0..PAGE_COUNT).map(|_| None).collect(),
            r: [0; 16],
            n: false,
            z: false,
            c: false,
            v: false,
            cpsr_low: INITIAL_CPSR_LOW,
            hook_pages: vec![0; PAGE_COUNT / 64],
            hook_ranges: Vec::new(),
            intrinsics: Vec::new(),
            stop: false,
            arch: Arch::Arm,
            mips: mips::MipsState::default(),
            sh: sh::ShState::default(),
        }
    }

    /// An interpreter for `arch`: ARM, or MIPS for Pocket PC 2000 builds.
    pub fn new_for(arch: Arch) -> Self {
        Self {
            arch,
            ..Self::new()
        }
    }

    /// SuperH general register `n`, for tests and debugging: the kernel's
    /// register names reach only the ones the calling convention uses.
    pub fn sh_reg(&self, n: usize) -> u32 {
        self.sh.r[n]
    }

    pub fn set_sh_reg(&mut self, n: usize, value: u32) {
        self.sh.r[n] = value;
    }

    /// SuperH (SR, GBR, MACH, MACL).
    pub fn sh_system(&self) -> (u32, u32, u32, u32) {
        (self.sh.sr, self.sh.gbr, self.sh.mach, self.sh.macl)
    }

    pub fn set_sh_system(&mut self, sr: u32, gbr: u32, mach: u32, macl: u32) {
        self.sh.sr = sr;
        self.sh.gbr = gbr;
        self.sh.mach = mach;
        self.sh.macl = macl;
    }

    fn cpsr(&self) -> u32 {
        (u32::from(self.n) << 31)
            | (u32::from(self.z) << 30)
            | (u32::from(self.c) << 29)
            | (u32::from(self.v) << 28)
            | (self.cpsr_low & 0x0FFF_FFFF)
    }

    fn set_cpsr(&mut self, value: u32) {
        self.n = value >> 31 & 1 != 0;
        self.z = value >> 30 & 1 != 0;
        self.c = value >> 29 & 1 != 0;
        self.v = value >> 28 & 1 != 0;
        self.cpsr_low = value & 0x0FFF_FFFF;
    }

    #[inline(always)]
    fn is_hook(&self, pc: u32) -> bool {
        let page = (pc >> PAGE_SHIFT) as usize;
        if self.hook_pages[page / 64] >> (page % 64) & 1 == 0 {
            return false;
        }
        self.hook_ranges
            .iter()
            .any(|(lo, hi)| (*lo..=*hi).contains(&pc))
    }

    #[inline(always)]
    fn read8(&self, va: u32) -> Result<u8, CpuError> {
        match &self.pages[(va >> PAGE_SHIFT) as usize] {
            Some(page) => Ok(page[(va & PAGE_MASK) as usize]),
            None => Err(bad("READ unmapped", va)),
        }
    }

    #[inline(always)]
    fn write8(&mut self, va: u32, value: u8) -> Result<(), CpuError> {
        match &mut self.pages[(va >> PAGE_SHIFT) as usize] {
            Some(page) => {
                page[(va & PAGE_MASK) as usize] = value;
                Ok(())
            }
            None => Err(bad("WRITE unmapped", va)),
        }
    }

    #[inline(always)]
    fn read32(&self, va: u32) -> Result<u32, CpuError> {
        let offset = (va & PAGE_MASK) as usize;
        if offset <= PAGE_SIZE - 4 {
            if let Some(page) = &self.pages[(va >> PAGE_SHIFT) as usize] {
                return Ok(u32::from_le_bytes([
                    page[offset],
                    page[offset + 1],
                    page[offset + 2],
                    page[offset + 3],
                ]));
            }
            return Err(bad("READ unmapped", va));
        }
        Ok(u32::from_le_bytes([
            self.read8(va)?,
            self.read8(va.wrapping_add(1))?,
            self.read8(va.wrapping_add(2))?,
            self.read8(va.wrapping_add(3))?,
        ]))
    }

    #[inline(always)]
    fn write32(&mut self, va: u32, value: u32) -> Result<(), CpuError> {
        let offset = (va & PAGE_MASK) as usize;
        if offset <= PAGE_SIZE - 4 {
            if let Some(page) = &mut self.pages[(va >> PAGE_SHIFT) as usize] {
                page[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
                return Ok(());
            }
            return Err(bad("WRITE unmapped", va));
        }
        for (i, byte) in value.to_le_bytes().into_iter().enumerate() {
            self.write8(va.wrapping_add(i as u32), byte)?;
        }
        Ok(())
    }

    fn read16(&self, va: u32) -> Result<u16, CpuError> {
        Ok(u16::from_le_bytes([
            self.read8(va)?,
            self.read8(va.wrapping_add(1))?,
        ]))
    }

    /// Register as an instruction sees it: the PC reads ahead by 8.
    #[inline(always)]
    fn reg(&self, n: u32, pc: u32) -> u32 {
        if n == 15 {
            pc.wrapping_add(8)
        } else {
            self.r[n as usize]
        }
    }

    #[inline(always)]
    fn condition(&self, code: u32) -> bool {
        match code {
            0 => self.z,
            1 => !self.z,
            2 => self.c,
            3 => !self.c,
            4 => self.n,
            5 => !self.n,
            6 => self.v,
            7 => !self.v,
            8 => self.c && !self.z,
            9 => !self.c || self.z,
            10 => self.n == self.v,
            11 => self.n != self.v,
            12 => !self.z && self.n == self.v,
            13 => self.z || self.n != self.v,
            14 => true,
            _ => false,
        }
    }

    /// Barrel shifter: returns the operand and its carry out.
    #[inline(always)]
    fn shifted(&self, i: u32, pc: u32) -> (u32, bool) {
        let kind = i >> 5 & 3;
        let by_reg = i >> 4 & 1 != 0;
        let rm = i & 15;
        if by_reg {
            let value = if rm == 15 {
                pc.wrapping_add(12)
            } else {
                self.r[rm as usize]
            };
            let amount = self.reg(i >> 8 & 15, pc) & 0xFF;
            if amount == 0 {
                return (value, self.c);
            }
            return match kind {
                0 => match amount {
                    1..=31 => (value << amount, value >> (32 - amount) & 1 != 0),
                    32 => (0, value & 1 != 0),
                    _ => (0, false),
                },
                1 => match amount {
                    1..=31 => (value >> amount, value >> (amount - 1) & 1 != 0),
                    32 => (0, value >> 31 != 0),
                    _ => (0, false),
                },
                2 => {
                    if amount >= 32 {
                        let sign = (value as i32 >> 31) as u32;
                        (sign, sign & 1 != 0)
                    } else {
                        (
                            ((value as i32) >> amount) as u32,
                            value >> (amount - 1) & 1 != 0,
                        )
                    }
                }
                _ => {
                    let rot = amount & 31;
                    if rot == 0 {
                        (value, value >> 31 != 0)
                    } else {
                        (value.rotate_right(rot), value >> (rot - 1) & 1 != 0)
                    }
                }
            };
        }
        let value = self.reg(rm, pc);
        let amount = i >> 7 & 31;
        match kind {
            0 => {
                if amount == 0 {
                    (value, self.c)
                } else {
                    (value << amount, value >> (32 - amount) & 1 != 0)
                }
            }
            1 => {
                if amount == 0 {
                    (0, value >> 31 != 0)
                } else {
                    (value >> amount, value >> (amount - 1) & 1 != 0)
                }
            }
            2 => {
                if amount == 0 {
                    let sign = (value as i32 >> 31) as u32;
                    (sign, sign & 1 != 0)
                } else {
                    (
                        ((value as i32) >> amount) as u32,
                        value >> (amount - 1) & 1 != 0,
                    )
                }
            }
            _ => {
                if amount == 0 {
                    ((u32::from(self.c) << 31) | value >> 1, value & 1 != 0)
                } else {
                    (value.rotate_right(amount), value >> (amount - 1) & 1 != 0)
                }
            }
        }
    }

    #[inline(always)]
    fn data_processing(&mut self, i: u32, pc: u32, next: &mut u32) {
        let (operand, shift_carry) = if i >> 25 & 1 != 0 {
            let rot = (i >> 8 & 15) * 2;
            let value = (i & 0xFF).rotate_right(rot);
            (value, if rot == 0 { self.c } else { value >> 31 != 0 })
        } else {
            self.shifted(i, pc)
        };
        let rn_index = i >> 16 & 15;
        // A register-specified shift makes the PC read ahead by 12 everywhere.
        let rn = if rn_index == 15 && i >> 25 & 1 == 0 && i >> 4 & 1 != 0 {
            pc.wrapping_add(12)
        } else {
            self.reg(rn_index, pc)
        };
        let opcode = i >> 21 & 15;
        let (result, carry, overflow) = match opcode {
            0 | 8 => (rn & operand, shift_carry, self.v),
            1 | 9 => (rn ^ operand, shift_carry, self.v),
            2 | 10 => add_flags(rn, !operand, true),
            3 => add_flags(operand, !rn, true),
            4 | 11 => add_flags(rn, operand, false),
            5 => add_flags(rn, operand, self.c),
            6 => add_flags(rn, !operand, self.c),
            7 => add_flags(operand, !rn, self.c),
            12 => (rn | operand, shift_carry, self.v),
            13 => (operand, shift_carry, self.v),
            14 => (rn & !operand, shift_carry, self.v),
            _ => (!operand, shift_carry, self.v),
        };
        if i >> 20 & 1 != 0 {
            self.n = result >> 31 != 0;
            self.z = result == 0;
            self.c = carry;
            self.v = overflow;
        }
        if !(8..=11).contains(&opcode) {
            let rd = i >> 12 & 15;
            if rd == 15 {
                *next = result & !3;
            } else {
                self.r[rd as usize] = result;
            }
        }
    }

    fn multiply(&mut self, i: u32, pc: u32) {
        let (rm, rs) = (self.reg(i & 15, pc), self.reg(i >> 8 & 15, pc));
        let mut result = rm.wrapping_mul(rs);
        if i >> 21 & 1 != 0 {
            result = result.wrapping_add(self.reg(i >> 12 & 15, pc));
        }
        self.r[(i >> 16 & 15) as usize] = result;
        if i >> 20 & 1 != 0 {
            self.n = result >> 31 != 0;
            self.z = result == 0;
        }
    }

    fn multiply_long(&mut self, i: u32, pc: u32) {
        let (rm, rs) = (self.reg(i & 15, pc), self.reg(i >> 8 & 15, pc));
        let (lo_index, hi_index) = ((i >> 12 & 15) as usize, (i >> 16 & 15) as usize);
        let mut result = if i >> 22 & 1 != 0 {
            i64::from(rm as i32).wrapping_mul(i64::from(rs as i32)) as u64
        } else {
            u64::from(rm).wrapping_mul(u64::from(rs))
        };
        if i >> 21 & 1 != 0 {
            let accumulator = u64::from(self.r[hi_index]) << 32 | u64::from(self.r[lo_index]);
            result = result.wrapping_add(accumulator);
        }
        self.r[lo_index] = result as u32;
        self.r[hi_index] = (result >> 32) as u32;
        if i >> 20 & 1 != 0 {
            self.n = result >> 63 != 0;
            self.z = result == 0;
        }
    }

    fn halfword(&mut self, i: u32, pc: u32, next: &mut u32) -> Result<(), CpuError> {
        let offset = if i >> 22 & 1 != 0 {
            (i >> 4 & 0xF0) | (i & 15)
        } else {
            self.reg(i & 15, pc)
        };
        let rn_index = i >> 16 & 15;
        let base = self.reg(rn_index, pc);
        let up = i >> 23 & 1 != 0;
        let moved = if up {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };
        let pre = i >> 24 & 1 != 0;
        let address = if pre { moved } else { base };
        let load = i >> 20 & 1 != 0;
        let rd = i >> 12 & 15;
        let writeback = !pre || i >> 21 & 1 != 0;
        if load {
            let value = match i >> 5 & 3 {
                1 => u32::from(self.read16(address)?),
                2 => self.read8(address)? as i8 as i32 as u32,
                _ => self.read16(address)? as i16 as i32 as u32,
            };
            if writeback {
                self.r[rn_index as usize] = moved;
            }
            if rd == 15 {
                *next = value & !3;
            } else {
                self.r[rd as usize] = value;
            }
        } else {
            let value = self.reg(rd, pc).wrapping_add(if rd == 15 { 4 } else { 0 });
            self.write8(address, value as u8)?;
            self.write8(address.wrapping_add(1), (value >> 8) as u8)?;
            if writeback {
                self.r[rn_index as usize] = moved;
            }
        }
        Ok(())
    }

    fn single_transfer(&mut self, i: u32, pc: u32, next: &mut u32) -> Result<(), CpuError> {
        let offset = if i >> 25 & 1 != 0 {
            self.shifted(i & !0x10, pc).0
        } else {
            i & 0xFFF
        };
        let rn_index = i >> 16 & 15;
        let base = self.reg(rn_index, pc);
        let moved = if i >> 23 & 1 != 0 {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };
        let pre = i >> 24 & 1 != 0;
        let address = if pre { moved } else { base };
        let writeback = !pre || i >> 21 & 1 != 0;
        let byte = i >> 22 & 1 != 0;
        let rd = i >> 12 & 15;
        if i >> 20 & 1 != 0 {
            let value = if byte {
                u32::from(self.read8(address)?)
            } else {
                // ARMv4 rotates a misaligned word load.
                self.read32(address & !3)?.rotate_right((address & 3) * 8)
            };
            if writeback {
                self.r[rn_index as usize] = moved;
            }
            if rd == 15 {
                *next = value & !3;
            } else {
                self.r[rd as usize] = value;
            }
        } else {
            let value = self.reg(rd, pc).wrapping_add(if rd == 15 { 4 } else { 0 });
            if byte {
                self.write8(address, value as u8)?;
            } else {
                self.write32(address & !3, value)?;
            }
            if writeback {
                self.r[rn_index as usize] = moved;
            }
        }
        Ok(())
    }

    fn block_transfer(&mut self, i: u32, pc: u32, next: &mut u32) -> Result<(), CpuError> {
        let list = i & 0xFFFF;
        let count = list.count_ones();
        let rn_index = i >> 16 & 15;
        let base = self.reg(rn_index, pc);
        let up = i >> 23 & 1 != 0;
        let pre = i >> 24 & 1 != 0;
        let total = count * 4;
        let lowest = if up { base } else { base.wrapping_sub(total) };
        let mut address = if up == pre {
            lowest.wrapping_add(4)
        } else {
            lowest
        };
        let final_base = if up {
            base.wrapping_add(total)
        } else {
            base.wrapping_sub(total)
        };
        let writeback = i >> 21 & 1 != 0;
        if i >> 20 & 1 != 0 {
            if writeback {
                self.r[rn_index as usize] = final_base;
            }
            for reg in 0..16u32 {
                if list >> reg & 1 != 0 {
                    let value = self.read32(address & !3)?;
                    if reg == 15 {
                        *next = value & !3;
                    } else {
                        self.r[reg as usize] = value;
                    }
                    address = address.wrapping_add(4);
                }
            }
        } else {
            for reg in 0..16u32 {
                if list >> reg & 1 != 0 {
                    let value = if reg == 15 {
                        pc.wrapping_add(12)
                    } else {
                        self.r[reg as usize]
                    };
                    self.write32(address & !3, value)?;
                    address = address.wrapping_add(4);
                }
            }
            if writeback {
                self.r[rn_index as usize] = final_base;
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn execute(&mut self, i: u32, pc: u32, next: &mut u32) -> Result<(), CpuError> {
        match i >> 25 & 7 {
            0 => {
                if i & 0x0FFF_FFF0 == 0x012F_FF10 {
                    let target = self.reg(i & 15, pc);
                    if target & 1 != 0 {
                        return Err(CpuError::Unsupported("Thumb state"));
                    }
                    *next = target & !3;
                } else if i & 0x0FC0_00F0 == 0x0000_0090 {
                    self.multiply(i, pc);
                } else if i & 0x0F80_00F0 == 0x0080_0090 {
                    self.multiply_long(i, pc);
                } else if i & 0x0FB0_0FF0 == 0x0100_0090 {
                    let address = self.reg(i >> 16 & 15, pc);
                    let source = self.reg(i & 15, pc);
                    let rd = (i >> 12 & 15) as usize;
                    if i >> 22 & 1 != 0 {
                        let old = self.read8(address)?;
                        self.write8(address, source as u8)?;
                        self.r[rd] = u32::from(old);
                    } else {
                        let old = self.read32(address & !3)?;
                        self.write32(address & !3, source)?;
                        self.r[rd] = old.rotate_right((address & 3) * 8);
                    }
                } else if i & 0x0E00_0090 == 0x0000_0090 {
                    self.halfword(i, pc, next)?;
                } else if i & 0x0FBF_0FFF == 0x010F_0000 {
                    self.r[(i >> 12 & 15) as usize] = self.cpsr();
                } else if i & 0x0FB0_FFF0 == 0x0120_F000 {
                    self.write_status(i, self.reg(i & 15, pc));
                } else if i & 0x0FFF_0FF0 == 0x016F_0F10 {
                    self.r[(i >> 12 & 15) as usize] = self.reg(i & 15, pc).leading_zeros();
                } else {
                    self.data_processing(i, pc, next);
                }
            }
            1 => {
                if i & 0x0FB0_F000 == 0x0320_F000 {
                    let value = (i & 0xFF).rotate_right((i >> 8 & 15) * 2);
                    self.write_status(i, value);
                } else {
                    self.data_processing(i, pc, next);
                }
            }
            2 | 3 => {
                if i >> 25 & 7 == 3 && i >> 4 & 1 != 0 {
                    return Err(CpuError::Backend(format!(
                        "emu_start: undefined instruction 0x{i:08x} at 0x{pc:08x}"
                    )));
                }
                self.single_transfer(i, pc, next)?;
            }
            4 => self.block_transfer(i, pc, next)?,
            5 => {
                if i >> 24 & 1 != 0 {
                    self.r[14] = pc.wrapping_add(4);
                }
                let offset = ((i << 8) as i32 >> 6) as u32;
                *next = pc.wrapping_add(8).wrapping_add(offset);
            }
            _ => {
                return Err(CpuError::Backend(format!(
                    "emu_start: unsupported instruction 0x{i:08x} at 0x{pc:08x}"
                )));
            }
        }
        Ok(())
    }

    /// Run `intrinsic` and return where the guest continues, or `None` if this
    /// state has to go through the guest's own code.
    fn run_intrinsic(&mut self, pc: u32, intrinsic: Intrinsic) -> Result<Option<u32>, CpuError> {
        match intrinsic {
            Intrinsic::ColorKeyBlit16 => {
                self.color_key_blit(|src, _| src)?;
                Ok(Some(self.r[14] & !3))
            }
            Intrinsic::ColorKeyAlphaBlit565 => {
                let alpha = self.read32(self.r[13].wrapping_add(4))? >> 2;
                self.color_key_blit(|src, dst| blend565(src, dst, alpha))?;
                Ok(Some(self.r[14] & !3))
            }
            Intrinsic::FillWords => {
                // `subs r1, r1, #1; bpl`: a negative count has different flags.
                let count = self.r[1];
                if (count as i32) < 0 {
                    return Ok(None);
                }
                for _ in 0..count {
                    self.write32(self.r[3], self.r[2])?;
                    self.r[3] = self.r[3].wrapping_add(4);
                }
                self.r[1] = u32::MAX;
                self.n = true;
                self.z = false;
                self.c = false;
                self.v = false;
                Ok(Some(pc.wrapping_add(8)))
            }
            Intrinsic::RtSdiv => {
                let (divisor, dividend) = (self.r[0] as i32, self.r[1] as i32);
                let (q, r) = if divisor == 0 {
                    (0, 0)
                } else {
                    (
                        dividend.wrapping_div(divisor),
                        dividend.wrapping_rem(divisor),
                    )
                };
                self.r[0] = q as u32;
                self.r[1] = r as u32;
                Ok(Some(self.r[14] & !3))
            }
            Intrinsic::SoftFloat(op) => {
                self.soft_float(op);
                Ok(Some(self.r[14] & !3))
            }
            Intrinsic::RtUdiv => {
                let (divisor, dividend) = (self.r[0], self.r[1]);
                let (q, r) = (
                    dividend.checked_div(divisor).unwrap_or(0),
                    dividend.checked_rem(divisor).unwrap_or(0),
                );
                self.r[0] = q;
                self.r[1] = r;
                Ok(Some(self.r[14] & !3))
            }
        }
    }

    /// One soft-float helper on the argument registers, as the HLE handler
    /// computes it. Single results go to `r0` (leaving `r1`), doubles and
    /// nothing else to `r0:r1`.
    fn soft_float(&mut self, op: crate::SoftOp) {
        use crate::SoftOp::*;
        let s = |v: u32| f32::from_bits(v);
        let d = |lo: u32, hi: u32| f64::from_bits(u64::from(hi) << 32 | u64::from(lo));
        let (a, b) = (s(self.r[0]), s(self.r[1]));
        let (x, y) = (d(self.r[0], self.r[1]), d(self.r[2], self.r[3]));
        let unsigned_s = |v: f32| {
            if v < 0.0 || !v.is_finite() {
                0
            } else {
                v as u32
            }
        };
        let unsigned_d = |v: f64| {
            if v < 0.0 || !v.is_finite() {
                0
            } else {
                v as u32
            }
        };
        let order = |o: Option<std::cmp::Ordering>| match o {
            Some(std::cmp::Ordering::Less) => u32::MAX,
            Some(std::cmp::Ordering::Greater) => 1,
            _ => 0,
        };
        let single = |cpu: &mut Self, v: f32| cpu.r[0] = v.to_bits();
        let double = |cpu: &mut Self, v: f64| {
            let bits = v.to_bits();
            cpu.r[0] = bits as u32;
            cpu.r[1] = (bits >> 32) as u32;
        };
        match op {
            Adds => single(self, a + b),
            Subs => single(self, a - b),
            Muls => single(self, a * b),
            Divs => single(self, a / b),
            Negs => single(self, -a),
            Stoi => self.r[0] = a as i32 as u32,
            Stou => self.r[0] = unsigned_s(a),
            Itos => single(self, self.r[0] as i32 as f32),
            Utos => single(self, self.r[0] as f32),
            Addd => double(self, x + y),
            Subd => double(self, x - y),
            Muld => double(self, x * y),
            Divd => double(self, x / y),
            Negd => double(self, -x),
            Dtoi => self.r[0] = x as i32 as u32,
            Dtou => self.r[0] = unsigned_d(x),
            Itod => double(self, self.r[0] as i32 as f64),
            Utod => double(self, self.r[0] as f64),
            Stod => double(self, a as f64),
            Dtos => single(self, x as f32),
            // Comparisons answer 1 or 0 (`__cmp*`: -1, 0 or 1; unordered is 0).
            Cmps => self.r[0] = order(a.partial_cmp(&b)),
            Eqs => self.r[0] = u32::from(a == b),
            Nes => self.r[0] = u32::from(a != b),
            Lts => self.r[0] = u32::from(a < b),
            Les => self.r[0] = u32::from(a <= b),
            Gts => self.r[0] = u32::from(a > b),
            Ges => self.r[0] = u32::from(a >= b),
            Cmpd => self.r[0] = order(x.partial_cmp(&y)),
            Eqd => self.r[0] = u32::from(x == y),
            Ned => self.r[0] = u32::from(x != y),
            Ltd => self.r[0] = u32::from(x < y),
            Led => self.r[0] = u32::from(x <= y),
            Gtd => self.r[0] = u32::from(x > y),
            Ged => self.r[0] = u32::from(x >= y),
        }
    }

    /// The colour-keyed copy shared by GapiDraw's blits. Rows go through buffers
    /// so a row costs a page lookup instead of one per pixel.
    fn color_key_blit(&mut self, mix: impl Fn(u16, u16) -> u16) -> Result<(), CpuError> {
        let sp = self.r[13];
        // The guest compares a whole register, so a key above 16 bits never hits.
        let key = self.r[3];
        let arg = |cpu: &Self, index: u32| cpu.read32(sp.wrapping_add(index * 4));
        let mut dst = arg(self, 2)?;
        let mut src = arg(self, 3)?;
        let width = arg(self, 4)? as i32;
        let height = arg(self, 5)? as i32;
        let dst_step = arg(self, 6)?.wrapping_shl(1);
        let src_step = arg(self, 7)?.wrapping_shl(1);
        if width > 0 {
            let bytes = width as usize * 2;
            let mut src_row = vec![0u8; bytes];
            let mut dst_row = vec![0u8; bytes];
            for _ in 0..height.max(0) {
                let overlap =
                    src.wrapping_sub(dst) < bytes as u32 || dst.wrapping_sub(src) < bytes as u32;
                if overlap {
                    // The guest loop writes as it reads; keep that order.
                    for offset in (0..bytes as u32).step_by(2) {
                        let pixel = self.read16(src.wrapping_add(offset))?;
                        if u32::from(pixel) != key {
                            let at = dst.wrapping_add(offset);
                            let value = mix(pixel, self.read16(at)?);
                            self.write8(at, value as u8)?;
                            self.write8(at.wrapping_add(1), (value >> 8) as u8)?;
                        }
                    }
                } else {
                    self.read_mem_into(src, &mut src_row)?;
                    self.read_mem_into(dst, &mut dst_row)?;
                    for (out, pixel) in dst_row.chunks_exact_mut(2).zip(src_row.chunks_exact(2)) {
                        let pixel = u16::from_le_bytes([pixel[0], pixel[1]]);
                        if u32::from(pixel) != key {
                            let under = u16::from_le_bytes([out[0], out[1]]);
                            out.copy_from_slice(&mix(pixel, under).to_le_bytes());
                        }
                    }
                    self.write_mem(dst, &dst_row)?;
                }
                dst = dst.wrapping_add(dst_step);
                src = src.wrapping_add(src_step);
            }
        }
        self.r[0] = 0;
        Ok(())
    }

    /// `MSR CPSR`: only the condition flags matter to a user-mode guest.
    fn write_status(&mut self, i: u32, value: u32) {
        if i >> 22 & 1 == 0 && i >> 19 & 1 != 0 {
            self.n = value >> 31 & 1 != 0;
            self.z = value >> 30 & 1 != 0;
            self.c = value >> 29 & 1 != 0;
            self.v = value >> 28 & 1 != 0;
        }
    }
}

impl Cpu for InterpCpu {
    fn arch(&self) -> Arch {
        self.arch
    }

    fn supports_native_thunks(&self) -> bool {
        false
    }

    fn install_intrinsic(&mut self, va: u32, intrinsic: Intrinsic) {
        let page = (va >> PAGE_SHIFT) as usize;
        self.hook_pages[page / 64] |= 1 << (page % 64);
        // Kept sorted by address: a game-code page holding one intrinsic
        // (a GapiDraw blitter, say) looks it up on every instruction run on
        // that page, so the lookup is a binary search, not a scan of every
        // intrinsic installed. The first one installed at an address wins.
        if let Err(at) = self.intrinsics.binary_search_by_key(&va, |(va, _)| *va) {
            self.intrinsics.insert(at, (va, intrinsic));
        }
    }

    fn map_region(&mut self, va: u32, size: u32, _prot: Prot) -> Result<(), CpuError> {
        let first = va >> PAGE_SHIFT;
        let last = va.wrapping_add(size.max(1) - 1) >> PAGE_SHIFT;
        for page in first..=last {
            let slot = &mut self.pages[page as usize];
            if slot.is_none() {
                *slot = Some(Box::new([0; PAGE_SIZE]));
            }
        }
        Ok(())
    }

    fn write_mem(&mut self, va: u32, data: &[u8]) -> Result<(), CpuError> {
        let mut done = 0;
        while done < data.len() {
            let at = va.wrapping_add(done as u32);
            let offset = (at & PAGE_MASK) as usize;
            let count = (PAGE_SIZE - offset).min(data.len() - done);
            let Some(page) = &mut self.pages[(at >> PAGE_SHIFT) as usize] else {
                return Err(CpuError::BadMemory {
                    va,
                    size: data.len() as u32,
                });
            };
            page[offset..offset + count].copy_from_slice(&data[done..done + count]);
            done += count;
        }
        Ok(())
    }

    fn read_mem(&mut self, va: u32, len: u32) -> Result<Vec<u8>, CpuError> {
        let mut out = vec![0; len as usize];
        self.read_mem_into(va, &mut out)?;
        Ok(out)
    }

    fn read_mem_into(&mut self, va: u32, dst: &mut [u8]) -> Result<(), CpuError> {
        let mut done = 0;
        while done < dst.len() {
            let at = va.wrapping_add(done as u32);
            let offset = (at & PAGE_MASK) as usize;
            let count = (PAGE_SIZE - offset).min(dst.len() - done);
            let Some(page) = &self.pages[(at >> PAGE_SHIFT) as usize] else {
                return Err(CpuError::BadMemory {
                    va,
                    size: dst.len() as u32,
                });
            };
            dst[done..done + count].copy_from_slice(&page[offset..offset + count]);
            done += count;
        }
        Ok(())
    }

    fn read_reg(&mut self, reg: ArmReg) -> Result<u32, CpuError> {
        if self.arch == Arch::Sh {
            return Ok(match reg {
                ArmReg::Pc => self.sh.pc,
                ArmReg::Lr => self.sh.pr,
                ArmReg::Cpsr => self.sh.sr,
                other => self.sh.r[sh::index_of(other).unwrap_or(0)],
            });
        }
        if self.arch == Arch::Mips {
            return Ok(match reg {
                ArmReg::Pc => self.mips.pc,
                ArmReg::Cpsr => self.mips.status,
                other => self.mips.gpr[mips::index_of(other).unwrap_or(0)],
            });
        }
        Ok(if reg == ArmReg::Cpsr {
            self.cpsr()
        } else {
            self.r[reg as usize]
        })
    }

    fn write_reg(&mut self, reg: ArmReg, value: u32) -> Result<(), CpuError> {
        if self.arch == Arch::Sh {
            match reg {
                ArmReg::Pc => self.sh.pc = value,
                ArmReg::Lr => self.sh.pr = value,
                ArmReg::Cpsr => self.sh.sr = value,
                other => {
                    if let Some(index) = sh::index_of(other) {
                        self.sh.r[index] = value;
                    }
                }
            }
            return Ok(());
        }
        if self.arch == Arch::Mips {
            match reg {
                ArmReg::Pc => self.mips.pc = value,
                ArmReg::Cpsr => self.mips.status = value,
                other => {
                    if let Some(index) = mips::index_of(other) {
                        self.mips.gpr[index] = value;
                    }
                }
            }
            return Ok(());
        }
        if reg == ArmReg::Cpsr {
            self.set_cpsr(value);
        } else {
            self.r[reg as usize] = value;
        }
        Ok(())
    }

    fn read_return(&mut self) -> Result<u32, CpuError> {
        Ok(match self.arch {
            Arch::Mips => self.mips.gpr[2],
            Arch::Sh => self.sh.r[0],
            Arch::Arm => self.r[0],
        })
    }

    fn write_return(&mut self, value: u32) -> Result<(), CpuError> {
        match self.arch {
            Arch::Mips => self.mips.gpr[2] = value,
            Arch::Sh => self.sh.r[0] = value,
            Arch::Arm => self.r[0] = value,
        }
        Ok(())
    }

    fn write_return_pair(&mut self, first: u32, second: u32) -> Result<(), CpuError> {
        if self.arch == Arch::Sh {
            self.sh.r[0] = first;
            self.sh.r[1] = second;
        } else if self.arch == Arch::Mips {
            self.mips.gpr[2] = first;
            self.mips.gpr[3] = second;
        } else {
            self.r[0] = first;
            self.r[1] = second;
        }
        Ok(())
    }

    fn add_code_hook(&mut self, va: u32) -> Result<(), CpuError> {
        self.add_code_hook_range(va, va)
    }

    fn add_instruction_hook(&mut self, va: u32) -> Result<(), CpuError> {
        self.add_code_hook(va)
    }

    fn add_code_hook_range(&mut self, lo: u32, hi: u32) -> Result<(), CpuError> {
        for page in lo >> PAGE_SHIFT..=hi >> PAGE_SHIFT {
            self.hook_pages[page as usize / 64] |= 1 << (page % 64);
        }
        self.hook_ranges.push((lo, hi));
        Ok(())
    }

    fn run_until_hook(
        &mut self,
        start_va: u32,
        max_instructions: u64,
    ) -> Result<StopReason, CpuError> {
        if self.arch == Arch::Mips {
            return self.run_mips(start_va, max_instructions);
        }
        if self.arch == Arch::Sh {
            return self.run_sh(start_va, max_instructions);
        }
        if self.cpsr_low >> 5 & 1 != 0 || start_va & 1 != 0 {
            return Err(CpuError::Unsupported("Thumb state"));
        }
        self.stop = false;
        let budget = if max_instructions == 0 {
            DEFAULT_SLICE
        } else {
            max_instructions
        };
        let mut pc = start_va & !3;
        // Instruction fetch is the hottest path, so the current code page and
        // its hook flag are cached and refreshed only on crossing a page.
        let mut code_page = u32::MAX;
        let mut code: *const u8 = std::ptr::null();
        let mut page_has_hooks = false;
        let mut executed = 0u64;
        let result = 'run: loop {
            if executed == budget {
                break 'run Ok(StopReason::InstructionLimit);
            }
            executed += 1;
            #[cfg(guest_profile)]
            guest_profile::sample(pc);
            let page = pc >> PAGE_SHIFT;
            if page != code_page {
                let Some(bytes) = &self.pages[page as usize] else {
                    break 'run Err(bad("FETCH unmapped", pc));
                };
                code = bytes.as_ptr();
                code_page = page;
                page_has_hooks = self.hook_pages[page as usize / 64] >> (page % 64) & 1 != 0;
            }
            if page_has_hooks {
                let intrinsic = self
                    .intrinsics
                    .binary_search_by_key(&pc, |(va, _)| *va)
                    .ok()
                    .map(|at| self.intrinsics[at].1);
                if let Some(intrinsic) = intrinsic {
                    match self.run_intrinsic(pc, intrinsic) {
                        Ok(Some(next)) => {
                            pc = next;
                            continue;
                        }
                        Ok(None) => {}
                        Err(error) => break 'run Err(error),
                    }
                }
                if self.is_hook(pc) {
                    executed -= 1;
                    break 'run Ok(StopReason::Hook(pc));
                }
            }
            // SAFETY: `code` points at a live page of PAGE_SIZE bytes, pages are
            // never freed, and `pc & PAGE_MASK` is word-aligned below PAGE_SIZE.
            let instruction = unsafe {
                u32::from_le(std::ptr::read_unaligned(
                    code.add((pc & PAGE_MASK) as usize).cast::<u32>(),
                ))
            };
            let mut next = pc.wrapping_add(4);
            if instruction >> 28 == 14 || self.condition(instruction >> 28) {
                if let Err(error) = self.execute(instruction, pc, &mut next) {
                    break 'run Err(error);
                }
            }
            pc = next;
            if self.stop {
                break 'run Ok(StopReason::Requested);
            }
        };
        self.r[15] = pc;
        RETIRED.fetch_add(executed, Ordering::Relaxed);
        result
    }

    fn request_stop(&mut self) {
        self.stop = true;
    }
}

/// GapiDraw's blend, including the way it lets the channels' carries interact.
fn blend565(src: u16, dst: u16, alpha: u32) -> u16 {
    const RED_BLUE: u32 = 0xF81F;
    const GREEN: u32 = 0x07E0;
    let (src, dst) = (u32::from(src), u32::from(dst));
    let red_blue = dst & RED_BLUE;
    let red_blue =
        red_blue.wrapping_add(((src & RED_BLUE).wrapping_sub(red_blue)).wrapping_mul(alpha) >> 6);
    let green = dst & GREEN;
    let green = green.wrapping_add(((src & GREEN).wrapping_sub(green)).wrapping_mul(alpha) >> 6);
    (((green ^ red_blue) & GREEN) ^ (red_blue & 0xFFFF)) as u16
}

/// Optional histogram of guest code, enabled with `--cfg guest_profile`. Shows
/// which guest functions deserve a native replacement.
#[cfg(guest_profile)]
mod guest_profile {
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Instant;

    struct State {
        counts: HashMap<u32, u64>,
        tick: u64,
        start: Instant,
    }

    static STATE: Mutex<Option<State>> = Mutex::new(None);
    static mut COUNTER: u32 = 0;

    pub fn sample(pc: u32) {
        // SAFETY: the interpreter runs on one thread.
        unsafe {
            COUNTER = COUNTER.wrapping_add(1);
            if COUNTER & 0x3FF != 0 {
                return;
            }
        }
        let mut guard = STATE.lock().unwrap();
        let state = guard.get_or_insert_with(|| State {
            counts: HashMap::new(),
            tick: 0,
            start: Instant::now(),
        });
        *state.counts.entry(pc & !0xFF).or_default() += 1;
        state.tick += 1;
        if state.tick % 200_000 == 0 {
            let mut rows: Vec<_> = state.counts.iter().map(|(a, c)| (*c, *a)).collect();
            rows.sort_unstable_by(|a, b| b.cmp(a));
            let mut text = format!(
                "instructions={} seconds={:.1}\n",
                state.tick * 1024,
                state.start.elapsed().as_secs_f64()
            );
            for (count, address) in rows.iter().take(30) {
                text += &format!(
                    "{address:08x} {:.2}%\n",
                    *count as f64 * 100.0 / state.tick as f64
                );
            }
            let _ = std::fs::write("guest_profile.txt", text);
        }
    }
}
