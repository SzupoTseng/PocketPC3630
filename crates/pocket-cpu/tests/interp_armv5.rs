//! ARMv5 instructions that programs built for XScale use, checked on the
//! interpreter (the backend of the browser and Android builds) without
//! Unicorn. `interp_diff` compares the loads and stores with Unicorn too.

use pocket_cpu::{interp::InterpCpu, regs::ArmReg::*, Cpu, Prot, StopReason};

const CODE: u32 = 0x10000;
const DATA: u32 = 0x20000;

fn cpu_with(words: &[u32]) -> InterpCpu {
    let mut cpu = InterpCpu::new();
    cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
    cpu.map_region(DATA, 0x1000, Prot::ALL).unwrap();
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    cpu.write_mem(CODE, &bytes).unwrap();
    cpu
}

/// `blx r4` calls the function r4 points at and comes back to the next
/// instruction, with LR set as a call sets it.
#[test]
fn blx_register_calls_and_returns() {
    // 0: blx r4        4: (hook)
    // 8: add r0, r0, #1   c: bx lr
    let mut cpu = cpu_with(&[0xE12F_FF34, 0xE1A0_0000, 0xE280_0001, 0xE12F_FF1E]);
    cpu.write_reg(R0, 41).unwrap();
    cpu.write_reg(R4, CODE + 8).unwrap();
    cpu.add_code_hook(CODE + 4).unwrap();
    assert_eq!(
        cpu.run_until_hook(CODE, 0).unwrap(),
        StopReason::Hook(CODE + 4)
    );
    assert_eq!(cpu.read_reg(R0).unwrap(), 42);
    assert_eq!(cpu.read_reg(Lr).unwrap(), CODE + 4);
}

/// `ldrd` loads a register pair, `strd` stores one, with writeback.
#[test]
fn ldrd_and_strd_move_register_pairs() {
    // 0: ldrd r2, [r1, #8]     (r2, r3 <- words at DATA+8, DATA+12)
    // 4: strd r2, [r1, #16]!   (words at DATA+16.. <- r2, r3; r1 += 16)
    // 8: (hook)
    let mut cpu = cpu_with(&[0xE1C1_20D8, 0xE1E1_21F0, 0xE1A0_0000]);
    let mut data = vec![0u8; 32];
    data[8..12].copy_from_slice(&0x1111_2222u32.to_le_bytes());
    data[12..16].copy_from_slice(&0x3333_4444u32.to_le_bytes());
    cpu.write_mem(DATA, &data).unwrap();
    cpu.write_reg(R1, DATA).unwrap();
    cpu.add_code_hook(CODE + 8).unwrap();
    assert_eq!(
        cpu.run_until_hook(CODE, 0).unwrap(),
        StopReason::Hook(CODE + 8)
    );
    assert_eq!(cpu.read_reg(R2).unwrap(), 0x1111_2222);
    assert_eq!(cpu.read_reg(R3).unwrap(), 0x3333_4444);
    assert_eq!(cpu.read_reg(R1).unwrap(), DATA + 16);
    let out = cpu.read_mem(DATA + 16, 8).unwrap();
    assert_eq!(
        u32::from_le_bytes(out[..4].try_into().unwrap()),
        0x1111_2222
    );
    assert_eq!(
        u32::from_le_bytes(out[4..].try_into().unwrap()),
        0x3333_4444
    );
}
