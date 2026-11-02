//! Replays MIPS programs recorded from Unicorn's MIPS by
//! `tools/mips_vectors.py` and checks the interpreter leaves the same
//! registers and memory. Recorded rather than run side by side, so the check
//! needs no native Unicorn and runs wherever the interpreter is built.

use pocket_cpu::{interp::InterpCpu, regs::ArmReg, Arch, Cpu, Prot, StopReason};

const CODE: u32 = 0x10000;
const DATA: u32 = 0x40000;
const DATA_SIZE: u32 = 0x400;
/// a0-a3 and s0-s7, as the kernel names them, then ra.
const NAMES: [ArmReg; 12] = [
    ArmReg::R0, ArmReg::R1, ArmReg::R2, ArmReg::R3, ArmReg::R4, ArmReg::R5,
    ArmReg::R6, ArmReg::R7, ArmReg::R8, ArmReg::R9, ArmReg::R10, ArmReg::R11,
];

fn words(field: &str) -> Vec<u32> {
    field.split_whitespace().map(|w| u32::from_str_radix(w, 16).unwrap()).collect()
}

fn bytes(field: &str) -> Vec<u8> {
    (0..field.len()).step_by(2).map(|i| u8::from_str_radix(&field[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn the_interpreter_matches_recorded_unicorn_runs() {
    let text = include_str!("data/mips_vectors.txt");
    let mut cases = 0;
    for (number, line) in text.lines().enumerate() {
        let fields: Vec<&str> = line.split('|').collect();
        let (code, seed, data, expected_regs, expected_mem) =
            (words(fields[0]), words(fields[1]), bytes(fields[2]), words(fields[3]), bytes(fields[4]));
        let mut cpu = InterpCpu::new_for(Arch::Mips);
        cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
        cpu.map_region(DATA, 0x10000, Prot::READ | Prot::WRITE).unwrap();
        let program: Vec<u8> = code.iter().flat_map(|w| w.to_le_bytes()).collect();
        cpu.write_mem(CODE, &program).unwrap();
        cpu.write_mem(DATA, &data).unwrap();
        for (name, value) in NAMES.iter().zip(&seed) {
            cpu.write_reg(*name, *value).unwrap();
        }
        cpu.write_reg(ArmReg::R12, DATA + DATA_SIZE / 2).unwrap();
        cpu.write_reg(ArmReg::Lr, 0).unwrap();
        let end = CODE + program.len() as u32;
        cpu.add_code_hook(end).unwrap();
        assert_eq!(cpu.run_until_hook(CODE, 0).unwrap(), StopReason::Hook(end), "case {number}");
        let mut regs: Vec<u32> = NAMES.iter().map(|n| cpu.read_reg(*n).unwrap()).collect();
        regs.push(cpu.read_reg(ArmReg::Lr).unwrap());
        let mem = cpu.read_mem(DATA, DATA_SIZE).unwrap();
        assert!(
            regs == expected_regs && mem == expected_mem,
            "case {number}:\n got      {regs:08x?}\n expected {expected_regs:08x?}\n program {code:08x?}"
        );
        cases += 1;
    }
    assert_eq!(cases, 300);
}
