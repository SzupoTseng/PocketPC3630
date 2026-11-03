//! Differential test: random ARM instructions on Unicorn and on the
//! interpreter must leave identical registers, flags and memory.
#![cfg(feature = "unicorn")]

use pocket_cpu::{interp::InterpCpu, regs::ArmReg::*, unicorn::UnicornCpu, Cpu, Prot};

const CODE: u32 = 0x10000;
const DATA: u32 = 0x40000;
const REGS: [pocket_cpu::regs::ArmReg; 13] =
    [R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    fn below(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}

/// An instruction that stays inside the data page and never branches. r11 is
/// the base register; it is left alone so every load and store stays mapped.
fn random_instruction(rng: &mut Rng) -> u32 {
    let cond = if rng.below(4) == 0 { rng.below(15) } else { 14 };
    let rd = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12][rng.below(12) as usize];
    let rn = rng.below(11);
    let rm = rng.below(11);
    let rs = rng.below(11);
    // The compare forms require Rd = 0, and MOV / MVN require Rn = 0.
    let dp_regs = |op: u32| -> (u32, u32) {
        (
            if op == 13 || op == 15 { 0 } else { rn },
            if (8..=11).contains(&op) { 0 } else { rd },
        )
    };
    let body = match rng.below(9) {
        0 => {
            // Data processing, immediate.
            let op = rng.below(16);
            let s = if (8..=11).contains(&op) {
                1
            } else {
                rng.below(2)
            };
            let (n, d) = dp_regs(op);
            (1 << 25) | op << 21 | s << 20 | n << 16 | d << 12 | rng.below(16) << 8 | rng.below(256)
        }
        1 => {
            // Data processing, immediate shift.
            let op = rng.below(16);
            let s = if (8..=11).contains(&op) {
                1
            } else {
                rng.below(2)
            };
            let (n, d) = dp_regs(op);
            op << 21 | s << 20 | n << 16 | d << 12 | rng.below(32) << 7 | rng.below(4) << 5 | rm
        }
        2 => {
            // Data processing, register shift.
            let op = rng.below(16);
            let s = if (8..=11).contains(&op) {
                1
            } else {
                rng.below(2)
            };
            let (n, d) = dp_regs(op);
            op << 21 | s << 20 | n << 16 | d << 12 | rs << 8 | rng.below(4) << 5 | 1 << 4 | rm
        }
        3 => {
            // MUL / MLA with distinct registers.
            let d = 12;
            let accumulate = rng.below(2);
            accumulate << 21
                | rng.below(2) << 20
                | d << 16
                | (rn * accumulate) << 12
                | rs << 8
                | 0x90
                | rm
        }
        4 => {
            // UMULL / UMLAL / SMULL / SMLAL.
            let (hi, lo) = (12, 10);
            0x0080_0090
                | rng.below(4) << 21
                | rng.below(2) << 20
                | hi << 16
                | lo << 12
                | rs << 8
                | rng.below(10)
        }
        5 => {
            // LDR / STR / LDRB / STRB, aligned word offset from r11.
            let load = rng.below(2);
            let byte = rng.below(2);
            let offset = rng.below(128) * 4;
            (1 << 26) | 1 << 24 | 1 << 23 | byte << 22 | load << 20 | 11 << 16 | rd << 12 | offset
        }
        6 => {
            // LDRH / STRH / LDRSB / LDRSH.
            let offset = rng.below(32) * 2;
            let (load, sh) = match rng.below(4) {
                0 => (0, 1),
                1 => (1, 1),
                2 => (1, 2),
                _ => (1, 3),
            };
            1 << 24
                | 1 << 23
                | 1 << 22
                | load << 20
                | 11 << 16
                | rd << 12
                | (offset >> 4) << 8
                | 0x90
                | sh << 5
                | (offset & 15)
        }
        7 => {
            // STMIA / LDMIA from r11, no writeback, r11 not in the list.
            let list = rng.next() & 0x07FF | 1;
            let load = rng.below(2);
            (4 << 25) | 1 << 23 | load << 20 | 11 << 16 | list
        }
        _ => {
            // MRS, then a flag-setting compare so flags keep moving.
            0x010F_0000 | rd << 12
        }
    };
    cond << 28 | body
}

fn setup(cpu: &mut dyn Cpu, code: &[u8], data: &[u8], regs: &[u32]) {
    cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
    cpu.map_region(DATA, 0x10000, Prot::READ | Prot::WRITE)
        .unwrap();
    cpu.write_mem(CODE, code).unwrap();
    cpu.write_mem(DATA, data).unwrap();
    for (reg, value) in REGS.iter().zip(regs) {
        cpu.write_reg(*reg, *value).unwrap();
    }
    cpu.write_reg(R11, DATA).unwrap();
    // Unicorn resets with Z set; start both from the same flags.
    let cpsr = cpu.read_reg(Cpsr).unwrap() & 0x0FFF_FFFF;
    cpu.write_reg(Cpsr, cpsr).unwrap();
    cpu.add_code_hook(CODE + code.len() as u32).unwrap();
}

#[test]
fn random_programs_match_unicorn() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for program in 0..300 {
        let words: Vec<u32> = (0..64).map(|_| random_instruction(&mut rng)).collect();
        let mut code: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        code.extend_from_slice(&0xE1A0_0000u32.to_le_bytes());
        let data: Vec<u8> = (0..1024).map(|_| rng.next() as u8).collect();
        let regs: Vec<u32> = (0..13)
            .map(|_| match rng.below(3) {
                0 => rng.next(),
                1 => rng.below(40),
                _ => 0x8000_0000u32.wrapping_add(rng.below(5)).wrapping_sub(2),
            })
            .collect();
        if let Some(detail) = first_difference(&words, &data, &regs) {
            panic!("program {program}: {detail}");
        }
    }
}

fn final_state(cpu: &mut dyn Cpu) -> (Vec<u32>, u32, Vec<u8>) {
    let regs = REGS.iter().map(|r| cpu.read_reg(*r).unwrap()).collect();
    let flags = cpu.read_reg(Cpsr).unwrap() & 0xF000_0000;
    (regs, flags, cpu.read_mem(DATA, 1024).unwrap())
}

fn run_both(words: &[u32], data: &[u8], regs: &[u32]) -> Result<bool, String> {
    let code: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let mut reference = UnicornCpu::new().unwrap();
    let mut subject = InterpCpu::new();
    setup(&mut reference, &code, data, regs);
    setup(&mut subject, &code, data, regs);
    reference.run_until_hook(CODE, 0).map_err(|e| {
        let pc = reference.read_reg(Pc).unwrap();
        format!("unicorn {e} at {pc:#x}")
    })?;
    subject
        .run_until_hook(CODE, 0)
        .map_err(|e| format!("interp {e}"))?;
    Ok(final_state(&mut reference) == final_state(&mut subject))
}

/// Shrink a mismatch to the first instruction whose prefix diverges.
fn first_difference(words: &[u32], data: &[u8], regs: &[u32]) -> Option<String> {
    match run_both(words, data, regs) {
        Ok(true) => return None,
        Err(error) => return Some(error),
        Ok(false) => {}
    }
    for length in 1..=words.len() {
        if run_both(&words[..length], data, regs) != Ok(true) {
            let code: Vec<u8> = words[..length]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect();
            let mut a = UnicornCpu::new().unwrap();
            let mut b = InterpCpu::new();
            setup(&mut a, &code, data, regs);
            setup(&mut b, &code, data, regs);
            a.run_until_hook(CODE, 0).unwrap();
            b.run_until_hook(CODE, 0).unwrap();
            let (ra, fa, _) = final_state(&mut a);
            let (rb, fb, _) = final_state(&mut b);
            return Some(format!(
                "instruction {:08x} (index {})\nunicorn r={ra:08x?} flags={fa:08x}\ninterp  r={rb:08x?} flags={fb:08x}",
                words[length - 1],
                length - 1
            ));
        }
    }
    Some("no shrink".into())
}

/// A native routine must leave exactly what GapiDraw's own ARM code leaves.
#[test]
fn intrinsics_match_the_guest_code() {
    use pocket_cpu::GD104_ROUTINES;
    const STACK: u32 = 0x60000;
    let return_to = CODE + 0x8000;
    let mut rng = Rng(0xABCD_EF01_2345_6789);
    for routine in GD104_ROUTINES.iter().filter(|r| r.code_rva != 0x841C) {
        let code: Vec<u8> = routine.code.iter().flat_map(|w| w.to_le_bytes()).collect();
        let mut hits = 0;
        for case in 0..200 {
            let key = if case % 5 == 0 { 0x1_07E0u32 } else { 0x07E0 };
            let mut pixels = Vec::with_capacity(0x8000);
            for _ in 0..0x4000 {
                let pixel = if rng.below(4) == 0 {
                    0x07E0
                } else {
                    rng.next() as u16
                };
                pixels.extend_from_slice(&pixel.to_le_bytes());
            }
            hits += pixels
                .chunks_exact(2)
                .filter(|p| p == &[0xE0, 0x07])
                .count();
            let width = rng.below(40) as i32 - 2;
            let height = rng.below(20) as i32 - 2;
            let src = DATA + rng.below(0x800) * 2;
            // Some cases overlap, which forces the pixel-by-pixel path.
            let dst = if case % 4 == 0 {
                src + rng.below(8) * 2
            } else {
                DATA + 0x4000 + rng.below(0x800) * 2
            };
            let dst_pitch = rng.below(50) + 40;
            let src_pitch = rng.below(50) + 40;
            let alpha = rng.below(257);
            let run = |native: bool| {
                let mut cpu = InterpCpu::new();
                cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
                cpu.map_region(DATA, 0x10000, Prot::READ | Prot::WRITE)
                    .unwrap();
                cpu.map_region(STACK, 0x10000, Prot::READ | Prot::WRITE)
                    .unwrap();
                cpu.write_mem(CODE, &code).unwrap();
                cpu.write_mem(DATA, &pixels).unwrap();
                let sp = STACK + 0x8000;
                let args = [
                    0,
                    alpha,
                    dst,
                    src,
                    width as u32,
                    height as u32,
                    dst_pitch,
                    src_pitch,
                ];
                for (i, value) in args.iter().enumerate() {
                    cpu.write_mem(sp + i as u32 * 4, &value.to_le_bytes())
                        .unwrap();
                }
                cpu.write_reg(Sp, sp).unwrap();
                cpu.write_reg(R3, key).unwrap();
                cpu.write_reg(Lr, return_to).unwrap();
                cpu.add_code_hook(return_to).unwrap();
                if native {
                    cpu.install_intrinsic(CODE, routine.intrinsic);
                }
                assert_eq!(
                    cpu.run_until_hook(CODE, 0).unwrap(),
                    pocket_cpu::StopReason::Hook(return_to)
                );
                (
                    cpu.read_reg(R0).unwrap(),
                    cpu.read_reg(Sp).unwrap(),
                    cpu.read_mem(DATA, 0x8000).unwrap(),
                )
            };
            let (guest, native) = (run(false), run(true));
            assert!(
                guest == native,
                "{:?} case {case}: {width}x{height} {src:#x}->{dst:#x} key {key:#x} alpha {alpha}",
                routine.intrinsic
            );
        }
        assert!(hits > 0, "the key has to occur or the test proves nothing");
    }
}

/// The fill loop ends in a jump to the function's tail, so it is entered and left
/// inside the routine; the flags survive into that tail.
#[test]
fn the_fill_intrinsic_matches_the_guest_loop() {
    use pocket_cpu::{Intrinsic, GD104_ROUTINES};
    let routine = GD104_ROUTINES
        .iter()
        .find(|r| r.intrinsic == Intrinsic::FillWords)
        .unwrap();
    let code: Vec<u8> = routine.code.iter().flat_map(|w| w.to_le_bytes()).collect();
    // The closing branch goes to RVA 0x8524; 0x841C is where the code starts.
    let tail = CODE + (0x8524 - 0x841C);
    let entry = CODE + (routine.entry_rva - routine.code_rva);
    for count in [0u32, 1, 2, 17, 500] {
        let run = |native: bool| {
            let mut cpu = InterpCpu::new();
            cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
            cpu.map_region(DATA, 0x10000, Prot::READ | Prot::WRITE)
                .unwrap();
            cpu.write_mem(CODE, &code).unwrap();
            cpu.write_reg(R1, count).unwrap();
            cpu.write_reg(R2, 0x1234_ABCD).unwrap();
            cpu.write_reg(R3, DATA + 8).unwrap();
            cpu.add_code_hook(tail).unwrap();
            if native {
                cpu.install_intrinsic(entry, routine.intrinsic);
            }
            assert_eq!(
                cpu.run_until_hook(entry, 0).unwrap(),
                pocket_cpu::StopReason::Hook(tail)
            );
            (
                [
                    cpu.read_reg(R1).unwrap(),
                    cpu.read_reg(R2).unwrap(),
                    cpu.read_reg(R3).unwrap(),
                ],
                cpu.read_reg(Cpsr).unwrap(),
                cpu.read_mem(DATA, 0x1000).unwrap(),
            )
        };
        assert!(run(false) == run(true), "count {count}");
    }
}

#[test]
fn a_branch_and_link_loop_reaches_its_hook() {
    // mov r0,#0 / loop: add r0,r0,#1 / cmp r0,#10 / bne loop / <hook>
    let words = [0xE3A0_0000u32, 0xE280_0001, 0xE350_000A, 0x1AFF_FFFC];
    let code: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let mut cpu = InterpCpu::new();
    setup(&mut cpu, &code, &[0; 4], &[0; 13]);
    let stop = cpu.run_until_hook(CODE, 0).unwrap();
    assert_eq!(stop, pocket_cpu::StopReason::Hook(CODE + 16));
    assert_eq!(cpu.read_reg(R0).unwrap(), 10);
}
