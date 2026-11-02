//! Replays SuperH blocks recorded from QEMU's SH-4 by `tools/sh_vectors.py`
//! and checks the interpreter leaves the same registers, T bit, MACH, MACL and
//! memory. The SH-4's integer instructions are the SH-3's.

use pocket_cpu::{interp::InterpCpu, Arch, Cpu, Prot, StopReason};

const AREA: u32 = 0x0050_0400;
const AREA_SIZE: u32 = 0x400;
const MID: u32 = AREA + AREA_SIZE / 2;

fn hex(field: &str) -> Vec<u8> {
    (0..field.len()).step_by(2).map(|i| u8::from_str_radix(&field[i..i + 2], 16).unwrap()).collect()
}

fn words(field: &str) -> Vec<u32> {
    field.split_whitespace().map(|w| u32::from_str_radix(w, 16).unwrap()).collect()
}

#[test]
fn the_interpreter_matches_recorded_qemu_runs() {
    let text = include_str!("data/sh_vectors.txt");
    let mut cases = 0;
    for (number, line) in text.lines().enumerate() {
        let f: Vec<&str> = line.split('|').collect();
        let place = words(f[0]);
        let (base, start, length) = (place[0], place[1], place[2]);
        let (code, init, area, expected, expected_area) = (hex(f[1]), words(f[2]), hex(f[3]), words(f[4]), hex(f[5]));
        let mut cpu = InterpCpu::new_for(Arch::Sh);
        cpu.map_region(base, 0x10000, Prot::ALL).unwrap();
        cpu.map_region(0x0050_0000, 0x1000, Prot::READ | Prot::WRITE).unwrap();
        cpu.write_mem(base, &code).unwrap();
        cpu.write_mem(AREA, &area).unwrap();
        for (n, value) in init.iter().take(14).enumerate() {
            cpu.set_sh_reg(n, *value);
        }
        cpu.set_sh_reg(14, MID);
        cpu.set_sh_system(init[16] & 1, MID, init[14], init[15]);
        let end = base + start + length;
        cpu.add_code_hook(end).unwrap();
        assert_eq!(cpu.run_until_hook(base + start, 0).unwrap(), StopReason::Hook(end), "case {number}");
        let mut got: Vec<u32> = (0..14).map(|n| cpu.sh_reg(n)).collect();
        let (sr, gbr, mach, macl) = cpu.sh_system();
        got.extend([sr & 1, mach, macl, gbr]);
        let area_now = cpu.read_mem(AREA, AREA_SIZE).unwrap();
        assert!(
            got == expected && area_now == expected_area,
            "case {number}:\n got      {got:08x?}\n expected {expected:08x?}\n block {:04x?}",
            code[start as usize..(start + length) as usize]
                .chunks(2)
                .map(|w| u16::from_le_bytes([w[0], w[1]]))
                .collect::<Vec<_>>()
        );
        cases += 1;
    }
    assert_eq!(cases, 300);
}

/// ADDV and SUBV set T on signed overflow, as the SH-3 manual defines it.
#[test]
fn addv_and_subv_flag_signed_overflow() {
    let cases = [
        // (instruction, Rn, Rm, result, T)
        (0x312F, 0x7FFF_FFFF, 1, 0x8000_0000, 1), // ADDV r2,r1: overflow
        (0x312F, 0x7FFF_FFFE, 1, 0x7FFF_FFFF, 0),
        (0x312F, 0x8000_0000, 0xFFFF_FFFF, 0x7FFF_FFFF, 1),
        (0x312B, 0x8000_0000, 1, 0x7FFF_FFFF, 1), // SUBV r2,r1: overflow
        (0x312B, 0x7FFF_FFFF, 0xFFFF_FFFF, 0x8000_0000, 1),
        (0x312B, 5, 3, 2, 0),
        (0x312B, 0xFFFF_FFFF, 0x7FFF_FFFF, 0x8000_0000, 0),
    ];
    for (instruction, rn, rm, result, t) in cases {
        let mut cpu = InterpCpu::new_for(Arch::Sh);
        cpu.map_region(0x10000, 0x1000, Prot::ALL).unwrap();
        cpu.write_mem(0x10000, &u16::to_le_bytes(instruction)).unwrap();
        cpu.set_sh_reg(1, rn);
        cpu.set_sh_reg(2, rm);
        cpu.add_code_hook(0x10002).unwrap();
        cpu.run_until_hook(0x10000, 0).unwrap();
        assert_eq!((cpu.sh_reg(1), cpu.sh_system().0 & 1), (result, t), "{instruction:04x} {rn:08x} {rm:08x}");
    }
}

/// ROTL and ROTR copy the bit rotated out into T, as the SH-3 manual defines it.
#[test]
fn rotations_set_t_from_the_bit_rotated_out() {
    let cases = [
        (0x4104u16, 0xD750_7759u32, 0xAEA0_EEB3u32, 1), // rotl r1
        (0x4104, 0x4000_0001, 0x8000_0002, 0),
        (0x4105, 0xD750_7759, 0xEBA8_3BAC, 1), // rotr r1
        (0x4105, 0x8000_0002, 0x4000_0001, 0),
    ];
    for (instruction, value, rotated, t) in cases {
        let mut cpu = InterpCpu::new_for(Arch::Sh);
        cpu.map_region(0x10000, 0x1000, Prot::ALL).unwrap();
        cpu.write_mem(0x10000, &instruction.to_le_bytes()).unwrap();
        cpu.set_sh_reg(1, value);
        cpu.add_code_hook(0x10002).unwrap();
        cpu.run_until_hook(0x10000, 0).unwrap();
        assert_eq!((cpu.sh_reg(1), cpu.sh_system().0 & 1), (rotated, t));
    }
}

/// Prints the interpreter's state before each instruction of one recorded case,
/// to line up with a `qemu-sh4 -one-insn-per-tb -d cpu` log. Run by hand.
#[test]
#[ignore]
fn trace_one_case() {
    let index: usize = std::env::var("SH_CASE").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    let line = include_str!("data/sh_vectors.txt").lines().nth(index).unwrap();
    let f: Vec<&str> = line.split('|').collect();
    let place = words(f[0]);
    let (base, start, length) = (place[0], place[1], place[2]);
    let (code, init, area) = (hex(f[1]), words(f[2]), hex(f[3]));
    let mut cpu = InterpCpu::new_for(Arch::Sh);
    cpu.map_region(base, 0x10000, Prot::ALL).unwrap();
    cpu.map_region(0x0050_0000, 0x1000, Prot::READ | Prot::WRITE).unwrap();
    cpu.write_mem(base, &code).unwrap();
    cpu.write_mem(AREA, &area).unwrap();
    for (n, value) in init.iter().take(14).enumerate() {
        cpu.set_sh_reg(n, *value);
    }
    cpu.set_sh_reg(14, MID);
    cpu.set_sh_system(init[16] & 1, MID, init[14], init[15]);
    let end = base + start + length;
    cpu.add_code_hook(end).unwrap();
    let mut pc = base + start;
    while pc != end {
        let regs: Vec<String> = (0..15).map(|n| format!("{:08x}", cpu.sh_reg(n))).collect();
        println!("{pc:08x} {} sr={:03x}", regs.join(" "), cpu.sh_system().0 & 0x303);
        let _ = cpu.run_until_hook(pc, 1);
        pc = cpu.read_reg(pocket_cpu::regs::ArmReg::Pc).unwrap();
    }
}
