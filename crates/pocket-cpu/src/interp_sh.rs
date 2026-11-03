//! SuperH (SH-3) for the portable interpreter.
//!
//! Pocket PC 2000 software also shipped as SH3 builds for machines such as the
//! HP Jornada 54x. Unicorn has no SuperH, so every frontend runs these here.
//! Windows CE runs user code in little-endian mode with 16-bit instructions,
//! delay slots after most branches, and no floating-point unit on the SH-3.
//! `tools/sh_vectors.py` records programs run by QEMU's SH-4, whose integer
//! instructions are the SH-3's, and `tests/sh_vectors.rs` replays them here.

use super::{
    bad, InterpCpu, DEFAULT_SLICE, IMPLICIT_CALL_HIGH, IMPLICIT_CALL_LOW, PAGE_MASK, PAGE_SHIFT,
    RETIRED,
};
use crate::{regs::ArmReg, CpuError, StopReason};
use std::sync::atomic::Ordering;

const T: u32 = 1;
const S: u32 = 1 << 1;
const Q: u32 = 1 << 8;
const M: u32 = 1 << 9;
/// Status bits user code can change; the rest of SR is privileged.
const USER_SR: u32 = T | S | Q | M;

/// SH registers the kernel names with ARM register names: the four argument
/// registers r4-r7 first, then the callee-saved r8-r14, then two scratch ones.
pub(super) fn index_of(reg: ArmReg) -> Option<usize> {
    use ArmReg::*;
    Some(match reg {
        R0 => 4,
        R1 => 5,
        R2 => 6,
        R3 => 7,
        R4 => 8,
        R5 => 9,
        R6 => 10,
        R7 => 11,
        R8 => 12,
        R9 => 13,
        R10 => 14,
        R11 => 2,
        R12 => 3,
        Sp => 15,
        Lr | Pc | Cpsr => return None,
    })
}

#[derive(Default)]
pub(super) struct ShState {
    pub r: [u32; 16],
    pub sr: u32,
    pub gbr: u32,
    pub mach: u32,
    pub macl: u32,
    pub pr: u32,
    pub pc: u32,
}

enum Flow {
    Next,
    /// A branch without a delay slot.
    Jump(u32),
    /// A delayed branch: the next instruction runs, then the target.
    Delayed(u32),
}

fn trap(kind: &str, pc: u32) -> CpuError {
    CpuError::Backend(format!(
        "emu_start: EXCEPTION ({kind}) at guest address 0x{pc:08x}"
    ))
}

fn sext8(x: u32) -> u32 {
    x as u8 as i8 as i32 as u32
}

fn sext12(x: u32) -> u32 {
    (((x & 0xFFF) << 20) as i32 >> 20) as u32
}

impl InterpCpu {
    #[inline(always)]
    fn t(&self) -> bool {
        self.sh.sr & T != 0
    }

    #[inline(always)]
    fn set_t(&mut self, value: bool) {
        self.sh.sr = (self.sh.sr & !T) | u32::from(value);
    }

    fn sh_read16(&self, va: u32) -> Result<u32, CpuError> {
        if va & 1 != 0 {
            return Err(trap("READ unaligned", va));
        }
        Ok(u32::from(self.read16(va)?))
    }

    fn sh_read32(&self, va: u32) -> Result<u32, CpuError> {
        if va & 3 != 0 {
            return Err(trap("READ unaligned", va));
        }
        self.read32(va)
    }

    fn sh_write16(&mut self, va: u32, value: u32) -> Result<(), CpuError> {
        if va & 1 != 0 {
            return Err(trap("WRITE unaligned", va));
        }
        self.write8(va, value as u8)?;
        self.write8(va.wrapping_add(1), (value >> 8) as u8)
    }

    fn sh_write32(&mut self, va: u32, value: u32) -> Result<(), CpuError> {
        if va & 3 != 0 {
            return Err(trap("WRITE unaligned", va));
        }
        self.write32(va, value)
    }

    fn sh_fetch(&self, pc: u32) -> Result<u32, CpuError> {
        if pc & 1 != 0 {
            return Err(trap("FETCH unaligned", pc));
        }
        match &self.pages[(pc >> PAGE_SHIFT) as usize] {
            Some(page) => {
                let at = (pc & PAGE_MASK) as usize;
                Ok(u32::from(u16::from_le_bytes([page[at], page[at + 1]])))
            }
            None => Err(bad("FETCH unmapped", pc)),
        }
    }

    /// One step of the non-restoring division DIV0S / DIV0U set up, as the
    /// SH-3 manual defines it: subtract when the old Q equals M, else add; the
    /// new Q is the bit shifted out, the carry or borrow and M combined.
    fn div1(&mut self, m: usize, n: usize) {
        let old_q = self.sh.sr & Q != 0;
        let shifted_out = self.sh.r[n] >> 31 != 0;
        let mflag = self.sh.sr & M != 0;
        let divisor = self.sh.r[m];
        let before = (self.sh.r[n] << 1) | u32::from(self.t());
        let (after, carry) = if old_q == mflag {
            let after = before.wrapping_sub(divisor);
            (after, after > before)
        } else {
            let after = before.wrapping_add(divisor);
            (after, after < before)
        };
        self.sh.r[n] = after;
        let q = shifted_out ^ carry ^ mflag;
        self.sh.sr = (self.sh.sr & !Q) | if q { Q } else { 0 };
        self.set_t(q == mflag);
    }

    fn mac_w(&mut self, product: i64) {
        let s = self.sh.sr & S != 0;
        if s {
            let acc = i64::from(self.sh.macl as i32) + product;
            let saturated = acc.clamp(i64::from(i32::MIN), i64::from(i32::MAX));
            if saturated != acc {
                self.sh.mach |= 1;
            }
            self.sh.macl = saturated as u32;
        } else {
            let acc = ((u64::from(self.sh.mach) << 32) | u64::from(self.sh.macl)) as i64 + product;
            self.sh.mach = (acc >> 32) as u32;
            self.sh.macl = acc as u32;
        }
    }

    fn mac_l(&mut self, product: i64) {
        let acc = ((u64::from(self.sh.mach) << 32) | u64::from(self.sh.macl)) as i64;
        let mut sum = acc.wrapping_add(product);
        if self.sh.sr & S != 0 {
            // Saturates to 48 bits.
            const MAX: i64 = (1 << 47) - 1;
            sum = sum.clamp(-MAX - 1, MAX);
        }
        self.sh.mach = (sum >> 32) as u32;
        self.sh.macl = sum as u32;
    }

    fn sh_step(&mut self, i: u32, pc: u32) -> Result<Flow, CpuError> {
        let n = ((i >> 8) & 15) as usize;
        let m = ((i >> 4) & 15) as usize;
        let d4 = i & 15;
        let d8 = i & 0xFF;
        let r = |cpu: &Self, x: usize| cpu.sh.r[x];
        let next = Flow::Next;
        Ok(match i >> 12 {
            0x0 => match i & 0xF {
                0x2 => {
                    let value = match m {
                        0 => self.sh.sr,
                        1 => self.sh.gbr,
                        _ => return Err(trap("privileged instruction", pc)),
                    };
                    self.sh.r[n] = value;
                    next
                }
                0x3 => match m {
                    0 => {
                        self.sh.pr = pc.wrapping_add(4);
                        Flow::Delayed(pc.wrapping_add(4).wrapping_add(r(self, n)))
                    }
                    2 => Flow::Delayed(pc.wrapping_add(4).wrapping_add(r(self, n))),
                    8 => next, // PREF
                    _ => return Err(trap("reserved instruction", pc)),
                },
                0x4 => {
                    self.write8(r(self, n).wrapping_add(r(self, 0)), r(self, m) as u8)?;
                    next
                }
                0x5 => {
                    self.sh_write16(r(self, n).wrapping_add(r(self, 0)), r(self, m))?;
                    next
                }
                0x6 => {
                    self.sh_write32(r(self, n).wrapping_add(r(self, 0)), r(self, m))?;
                    next
                }
                0x7 => {
                    self.sh.macl = r(self, n).wrapping_mul(r(self, m));
                    next
                }
                0x8 => {
                    match i {
                        0x0008 => self.set_t(false),
                        0x0018 => self.set_t(true),
                        0x0028 => {
                            self.sh.mach = 0;
                            self.sh.macl = 0;
                        }
                        0x0048 => self.sh.sr &= !S,
                        0x0058 => self.sh.sr |= S,
                        _ => return Err(trap("reserved instruction", pc)),
                    }
                    next
                }
                0x9 => match i & 0xFF {
                    0x09 => next,
                    0x19 => {
                        self.sh.sr &= !(M | Q | T);
                        next
                    }
                    0x29 => {
                        self.sh.r[n] = u32::from(self.t());
                        next
                    }
                    _ => return Err(trap("reserved instruction", pc)),
                },
                0xA => {
                    self.sh.r[n] = match m {
                        0 => self.sh.mach,
                        1 => self.sh.macl,
                        2 => self.sh.pr,
                        _ => return Err(trap("reserved instruction", pc)),
                    };
                    next
                }
                0xB => match i {
                    0x000B => Flow::Delayed(self.sh.pr),
                    0x001B => next, // SLEEP: nothing to wait for here.
                    _ => return Err(trap("privileged instruction", pc)),
                },
                0xC => {
                    self.sh.r[n] =
                        sext8(u32::from(self.read8(r(self, m).wrapping_add(r(self, 0)))?));
                    next
                }
                0xD => {
                    let half = self.sh_read16(r(self, m).wrapping_add(r(self, 0)))?;
                    self.sh.r[n] = half as u16 as i16 as i32 as u32;
                    next
                }
                0xE => {
                    self.sh.r[n] = self.sh_read32(r(self, m).wrapping_add(r(self, 0)))?;
                    next
                }
                0xF => {
                    let b = self.sh_read32(r(self, n))? as i32;
                    self.sh.r[n] = r(self, n).wrapping_add(4);
                    let a = self.sh_read32(r(self, m))? as i32;
                    self.sh.r[m] = r(self, m).wrapping_add(4);
                    self.mac_l(i64::from(a) * i64::from(b));
                    next
                }
                _ => return Err(trap("reserved instruction", pc)),
            },
            0x1 => {
                self.sh_write32(r(self, n).wrapping_add(d4 * 4), r(self, m))?;
                next
            }
            0x2 => {
                let (rn, rm) = (r(self, n), r(self, m));
                match d4 {
                    0x0 => self.write8(rn, rm as u8)?,
                    0x1 => self.sh_write16(rn, rm)?,
                    0x2 => self.sh_write32(rn, rm)?,
                    0x4 => {
                        let at = rn.wrapping_sub(1);
                        self.write8(at, rm as u8)?;
                        self.sh.r[n] = at;
                    }
                    0x5 => {
                        let at = rn.wrapping_sub(2);
                        self.sh_write16(at, rm)?;
                        self.sh.r[n] = at;
                    }
                    0x6 => {
                        let at = rn.wrapping_sub(4);
                        self.sh_write32(at, rm)?;
                        self.sh.r[n] = at;
                    }
                    0x7 => {
                        let (qn, mm) = (rn >> 31 != 0, rm >> 31 != 0);
                        self.sh.sr = (self.sh.sr & !(Q | M))
                            | if qn { Q } else { 0 }
                            | if mm { M } else { 0 };
                        self.set_t(qn != mm);
                    }
                    0x8 => self.set_t(rn & rm == 0),
                    0x9 => self.sh.r[n] = rn & rm,
                    0xA => self.sh.r[n] = rn ^ rm,
                    0xB => self.sh.r[n] = rn | rm,
                    0xC => {
                        let x = rn ^ rm;
                        self.set_t(x.to_le_bytes().contains(&0));
                    }
                    0xD => self.sh.r[n] = (rn >> 16) | (rm << 16),
                    0xE => self.sh.macl = (rn & 0xFFFF).wrapping_mul(rm & 0xFFFF),
                    0xF => {
                        self.sh.macl = ((rn as u16 as i16 as i32)
                            .wrapping_mul(rm as u16 as i16 as i32))
                            as u32;
                    }
                    _ => return Err(trap("reserved instruction", pc)),
                }
                next
            }
            0x3 => {
                let (rn, rm) = (r(self, n), r(self, m));
                match d4 {
                    0x0 => self.set_t(rn == rm),
                    0x2 => self.set_t(rn >= rm),
                    0x3 => self.set_t(rn as i32 >= rm as i32),
                    0x4 => self.div1(m, n),
                    0x5 => {
                        let product = u64::from(rn) * u64::from(rm);
                        self.sh.mach = (product >> 32) as u32;
                        self.sh.macl = product as u32;
                    }
                    0x6 => self.set_t(rn > rm),
                    0x7 => self.set_t(rn as i32 > rm as i32),
                    0x8 => self.sh.r[n] = rn.wrapping_sub(rm),
                    0xA => {
                        let (a, borrow1) = rn.overflowing_sub(rm);
                        let (b, borrow2) = a.overflowing_sub(u32::from(self.t()));
                        self.sh.r[n] = b;
                        self.set_t(borrow1 || borrow2);
                    }
                    0xB => {
                        let (value, overflow) = (rn as i32).overflowing_sub(rm as i32);
                        self.sh.r[n] = value as u32;
                        self.set_t(overflow);
                    }
                    0xC => self.sh.r[n] = rn.wrapping_add(rm),
                    0xD => {
                        let product = i64::from(rn as i32) * i64::from(rm as i32);
                        self.sh.mach = (product >> 32) as u32;
                        self.sh.macl = product as u32;
                    }
                    0xE => {
                        let (a, carry1) = rn.overflowing_add(rm);
                        let (b, carry2) = a.overflowing_add(u32::from(self.t()));
                        self.sh.r[n] = b;
                        self.set_t(carry1 || carry2);
                    }
                    0xF => {
                        let (value, overflow) = (rn as i32).overflowing_add(rm as i32);
                        self.sh.r[n] = value as u32;
                        self.set_t(overflow);
                    }
                    _ => return Err(trap("reserved instruction", pc)),
                }
                next
            }
            0x4 => {
                let rn = r(self, n);
                match i & 0xFF {
                    0x00 | 0x20 => {
                        self.set_t(rn >> 31 != 0);
                        self.sh.r[n] = rn << 1;
                    }
                    0x01 => {
                        self.set_t(rn & 1 != 0);
                        self.sh.r[n] = rn >> 1;
                    }
                    0x21 => {
                        self.set_t(rn & 1 != 0);
                        self.sh.r[n] = ((rn as i32) >> 1) as u32;
                    }
                    0x04 => {
                        self.set_t(rn >> 31 != 0);
                        self.sh.r[n] = rn.rotate_left(1);
                    }
                    0x05 => {
                        self.set_t(rn & 1 != 0);
                        self.sh.r[n] = rn.rotate_right(1);
                    }
                    0x24 => {
                        let carry = self.t();
                        self.set_t(rn >> 31 != 0);
                        self.sh.r[n] = (rn << 1) | u32::from(carry);
                    }
                    0x25 => {
                        let carry = self.t();
                        self.set_t(rn & 1 != 0);
                        self.sh.r[n] = (rn >> 1) | (u32::from(carry) << 31);
                    }
                    0x08 => self.sh.r[n] = rn << 2,
                    0x09 => self.sh.r[n] = rn >> 2,
                    0x18 => self.sh.r[n] = rn << 8,
                    0x19 => self.sh.r[n] = rn >> 8,
                    0x28 => self.sh.r[n] = rn << 16,
                    0x29 => self.sh.r[n] = rn >> 16,
                    0x10 => {
                        self.sh.r[n] = rn.wrapping_sub(1);
                        self.set_t(self.sh.r[n] == 0);
                    }
                    0x11 => self.set_t(rn as i32 >= 0),
                    0x15 => self.set_t(rn as i32 > 0),
                    0x02 | 0x12 | 0x22 | 0x13 => {
                        let value = match i & 0xFF {
                            0x02 => self.sh.mach,
                            0x12 => self.sh.macl,
                            0x22 => self.sh.pr,
                            _ => self.sh.gbr,
                        };
                        let at = rn.wrapping_sub(4);
                        self.sh_write32(at, value)?;
                        self.sh.r[n] = at;
                    }
                    0x03 => {
                        let at = rn.wrapping_sub(4);
                        self.sh_write32(at, self.sh.sr)?;
                        self.sh.r[n] = at;
                    }
                    0x06 | 0x16 | 0x26 | 0x17 | 0x07 => {
                        let value = self.sh_read32(rn)?;
                        self.sh.r[n] = rn.wrapping_add(4);
                        match i & 0xFF {
                            0x06 => self.sh.mach = value,
                            0x16 => self.sh.macl = value,
                            0x26 => self.sh.pr = value,
                            0x17 => self.sh.gbr = value,
                            _ => self.sh.sr = (self.sh.sr & !USER_SR) | (value & USER_SR),
                        }
                    }
                    0x0A => self.sh.mach = rn,
                    0x1A => self.sh.macl = rn,
                    0x2A => self.sh.pr = rn,
                    0x1E => self.sh.gbr = rn,
                    0x0E => self.sh.sr = (self.sh.sr & !USER_SR) | (rn & USER_SR),
                    0x0B => {
                        self.sh.pr = pc.wrapping_add(4);
                        return Ok(Flow::Delayed(rn));
                    }
                    0x2B => return Ok(Flow::Delayed(rn)),
                    0x1B => {
                        let byte = self.read8(rn)?;
                        self.set_t(byte == 0);
                        self.write8(rn, byte | 0x80)?;
                    }
                    _ => match d4 {
                        0xC | 0xD => {
                            let rm = r(self, m);
                            let amount = rm & 31;
                            self.sh.r[n] = if rm as i32 >= 0 {
                                rn << amount
                            } else if amount == 0 {
                                if d4 == 0xC && rn as i32 >= 0 || d4 == 0xD {
                                    0
                                } else {
                                    u32::MAX
                                }
                            } else if d4 == 0xC {
                                ((rn as i32) >> (32 - amount)) as u32
                            } else {
                                rn >> (32 - amount)
                            };
                        }
                        0xF => {
                            let b = self.sh_read16(rn)? as u16 as i16;
                            self.sh.r[n] = rn.wrapping_add(2);
                            let rm = r(self, m);
                            let a = self.sh_read16(rm)? as u16 as i16;
                            self.sh.r[m] = rm.wrapping_add(2);
                            self.mac_w(i64::from(a) * i64::from(b));
                        }
                        _ => return Err(trap("reserved instruction", pc)),
                    },
                }
                next
            }
            0x5 => {
                self.sh.r[n] = self.sh_read32(r(self, m).wrapping_add(d4 * 4))?;
                next
            }
            0x6 => {
                let rm = r(self, m);
                self.sh.r[n] = match d4 {
                    0x0 => sext8(u32::from(self.read8(rm)?)),
                    0x1 => self.sh_read16(rm)? as u16 as i16 as i32 as u32,
                    0x2 => self.sh_read32(rm)?,
                    0x3 => rm,
                    0x4 | 0x5 | 0x6 => {
                        let (value, size) = match d4 {
                            0x4 => (sext8(u32::from(self.read8(rm)?)), 1),
                            0x5 => (self.sh_read16(rm)? as u16 as i16 as i32 as u32, 2),
                            _ => (self.sh_read32(rm)?, 4),
                        };
                        if n != m {
                            self.sh.r[m] = rm.wrapping_add(size);
                        }
                        value
                    }
                    0x7 => !rm,
                    0x8 => (rm & 0xFFFF_0000) | ((rm & 0xFF) << 8) | ((rm >> 8) & 0xFF),
                    0x9 => rm.rotate_left(16),
                    0xA => {
                        let t = u32::from(self.t());
                        let value = 0u32.wrapping_sub(rm).wrapping_sub(t);
                        self.set_t(rm != 0 || t != 0);
                        value
                    }
                    0xB => 0u32.wrapping_sub(rm),
                    0xC => rm & 0xFF,
                    0xD => rm & 0xFFFF,
                    0xE => sext8(rm),
                    _ => rm as u16 as i16 as i32 as u32,
                };
                next
            }
            0x7 => {
                self.sh.r[n] = r(self, n).wrapping_add(sext8(d8));
                next
            }
            0x8 => {
                let rm = r(self, m);
                match n {
                    0x0 => self.write8(rm.wrapping_add(d4), r(self, 0) as u8)?,
                    0x1 => self.sh_write16(rm.wrapping_add(d4 * 2), r(self, 0))?,
                    0x4 => self.sh.r[0] = sext8(u32::from(self.read8(rm.wrapping_add(d4))?)),
                    0x5 => {
                        self.sh.r[0] =
                            self.sh_read16(rm.wrapping_add(d4 * 2))? as u16 as i16 as i32 as u32
                    }
                    0x8 => self.set_t(r(self, 0) == sext8(d8)),
                    0x9 | 0xB | 0xD | 0xF => {
                        let target = pc.wrapping_add(4).wrapping_add(sext8(d8) << 1);
                        let taken = if n == 0x9 || n == 0xD {
                            self.t()
                        } else {
                            !self.t()
                        };
                        return Ok(match (taken, n >= 0xD) {
                            (true, true) => Flow::Delayed(target),
                            (true, false) => Flow::Jump(target),
                            (false, _) => next,
                        });
                    }
                    _ => return Err(trap("reserved instruction", pc)),
                }
                next
            }
            0x9 => {
                let at = pc.wrapping_add(4).wrapping_add(d8 * 2);
                self.sh.r[n] = self.sh_read16(at)? as u16 as i16 as i32 as u32;
                next
            }
            0xA => Flow::Delayed(pc.wrapping_add(4).wrapping_add(sext12(i) << 1)),
            0xB => {
                self.sh.pr = pc.wrapping_add(4);
                Flow::Delayed(pc.wrapping_add(4).wrapping_add(sext12(i) << 1))
            }
            0xC => {
                let gbr = self.sh.gbr;
                match n {
                    0x0 => self.write8(gbr.wrapping_add(d8), r(self, 0) as u8)?,
                    0x1 => self.sh_write16(gbr.wrapping_add(d8 * 2), r(self, 0))?,
                    0x2 => self.sh_write32(gbr.wrapping_add(d8 * 4), r(self, 0))?,
                    0x3 => return Err(trap("trapa", pc)),
                    0x4 => self.sh.r[0] = sext8(u32::from(self.read8(gbr.wrapping_add(d8))?)),
                    0x5 => {
                        self.sh.r[0] =
                            self.sh_read16(gbr.wrapping_add(d8 * 2))? as u16 as i16 as i32 as u32
                    }
                    0x6 => self.sh.r[0] = self.sh_read32(gbr.wrapping_add(d8 * 4))?,
                    0x7 => self.sh.r[0] = (pc & !3).wrapping_add(4).wrapping_add(d8 * 4),
                    0x8 => self.set_t(r(self, 0) & d8 == 0),
                    0x9 => self.sh.r[0] &= d8,
                    0xA => self.sh.r[0] ^= d8,
                    0xB => self.sh.r[0] |= d8,
                    _ => {
                        let at = gbr.wrapping_add(r(self, 0));
                        let byte = u32::from(self.read8(at)?);
                        match n {
                            0xC => self.set_t(byte & d8 == 0),
                            0xD => self.write8(at, (byte & d8) as u8)?,
                            0xE => self.write8(at, (byte ^ d8) as u8)?,
                            _ => self.write8(at, (byte | d8) as u8)?,
                        }
                    }
                }
                next
            }
            0xD => {
                let at = (pc & !3).wrapping_add(4).wrapping_add(d8 * 4);
                self.sh.r[n] = self.sh_read32(at)?;
                next
            }
            0xE => {
                self.sh.r[n] = sext8(d8);
                next
            }
            _ => return Err(CpuError::Unsupported("SH floating-point instruction")),
        })
    }

    pub(super) fn run_sh(
        &mut self,
        start_va: u32,
        max_instructions: u64,
    ) -> Result<StopReason, CpuError> {
        self.stop = false;
        let budget = if max_instructions == 0 {
            DEFAULT_SLICE
        } else {
            max_instructions
        };
        let mut pc = start_va;
        let mut executed = 0u64;
        let result = 'run: loop {
            if executed >= budget {
                break 'run Ok(StopReason::InstructionLimit);
            }
            if self.is_hook(pc) {
                break 'run Ok(StopReason::Hook(pc));
            }
            // An implicit kernel call (see `interp_mips.rs`): return to
            // the caller with the first argument left as the result.
            if (IMPLICIT_CALL_LOW..=IMPLICIT_CALL_HIGH).contains(&pc) {
                self.sh.r[0] = self.sh.r[4];
                pc = self.sh.pr;
                continue;
            }
            let instruction = match self.sh_fetch(pc) {
                Ok(i) => i,
                Err(error) => break 'run Err(error),
            };
            executed += 1;
            match self.sh_step(instruction, pc) {
                Ok(Flow::Next) => pc = pc.wrapping_add(2),
                Ok(Flow::Jump(target)) => pc = target,
                Ok(Flow::Delayed(target)) => {
                    let slot_pc = pc.wrapping_add(2);
                    let slot = match self.sh_fetch(slot_pc) {
                        Ok(i) => i,
                        Err(error) => break 'run Err(error),
                    };
                    executed += 1;
                    match self.sh_step(slot, slot_pc) {
                        Ok(Flow::Next) => pc = target,
                        Ok(_) => break 'run Err(trap("slot illegal instruction", slot_pc)),
                        Err(error) => break 'run Err(error),
                    }
                }
                Err(error) => break 'run Err(error),
            }
            if self.stop {
                break 'run Ok(StopReason::Requested);
            }
        };
        self.sh.pc = pc;
        RETIRED.fetch_add(executed, Ordering::Relaxed);
        result
    }
}
