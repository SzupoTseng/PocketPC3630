//! Interpreter intrinsics that need no Unicorn to check: the CRT helpers the
//! interpreter answers in place of a dispatcher round trip.

use pocket_cpu::{interp::InterpCpu, regs::ArmReg::*, Cpu, Prot};

const CODE: u32 = 0x10000;

/// `bl` to a helper slot carrying the divide intrinsic: the call returns to
/// the next instruction with the quotient in r0 and the remainder in r1, and
/// a zero divisor gives 0 and 0 as the HLE handler does.
#[test]
fn the_divide_intrinsics_answer_like_the_helpers() {
    use pocket_cpu::Intrinsic;
    // bl CODE+0x100 ; (hook here)          helper slot: bx lr
    let mut code = vec![0u8; 0x104];
    code[..4].copy_from_slice(&0xEB00_003Eu32.to_le_bytes());
    code[0x100..0x104].copy_from_slice(&0xE12F_FF1Eu32.to_le_bytes());
    let run = |intrinsic: Intrinsic, divisor: u32, dividend: u32| {
        let mut cpu = InterpCpu::new();
        cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
        cpu.write_mem(CODE, &code).unwrap();
        cpu.write_reg(R0, divisor).unwrap();
        cpu.write_reg(R1, dividend).unwrap();
        cpu.install_intrinsic(CODE + 0x100, intrinsic);
        cpu.add_code_hook(CODE + 4).unwrap();
        assert_eq!(
            cpu.run_until_hook(CODE, 0).unwrap(),
            pocket_cpu::StopReason::Hook(CODE + 4)
        );
        (cpu.read_reg(R0).unwrap(), cpu.read_reg(R1).unwrap())
    };
    let s = |d: i32, n: i32| run(Intrinsic::RtSdiv, d as u32, n as u32);
    assert_eq!(s(7, 100), (14, 2));
    assert_eq!(s(-7, 100), ((-14i32) as u32, 2));
    assert_eq!(s(7, -100), ((-14i32) as u32, (-2i32) as u32));
    assert_eq!(s(0, 5), (0, 0));
    assert_eq!(s(-1, i32::MIN), (i32::MIN as u32, 0));
    let u = |d: u32, n: u32| run(Intrinsic::RtUdiv, d, n);
    assert_eq!(u(7, 100), (14, 2));
    assert_eq!(u(3, u32::MAX), (u32::MAX / 3, u32::MAX % 3));
    assert_eq!(u(0, 9), (0, 0));
}

/// Run one helper call through its intrinsic and return r0..r3 afterwards.
fn call(intrinsic: pocket_cpu::Intrinsic, regs: [u32; 4]) -> [u32; 4] {
    let mut code = vec![0u8; 0x104];
    code[..4].copy_from_slice(&0xEB00_003Eu32.to_le_bytes());
    code[0x100..0x104].copy_from_slice(&0xE12F_FF1Eu32.to_le_bytes());
    let mut cpu = InterpCpu::new();
    cpu.map_region(CODE, 0x10000, Prot::ALL).unwrap();
    cpu.write_mem(CODE, &code).unwrap();
    for (reg, value) in [R0, R1, R2, R3].iter().zip(regs) {
        cpu.write_reg(*reg, value).unwrap();
    }
    cpu.install_intrinsic(CODE + 0x100, intrinsic);
    cpu.add_code_hook(CODE + 4).unwrap();
    assert_eq!(
        cpu.run_until_hook(CODE, 0).unwrap(),
        pocket_cpu::StopReason::Hook(CODE + 4)
    );
    [R0, R1, R2, R3].map(|r| cpu.read_reg(r).unwrap())
}

/// The soft-float helpers give the same bits the HLE handlers give: IEEE
/// arithmetic, saturating float-to-int with NaN as 0, doubles in register
/// pairs low word first, and r1 left alone by single-result helpers.
#[test]
fn the_soft_float_intrinsics_answer_like_the_helpers() {
    use pocket_cpu::helper_intrinsic;
    let op = |name: &str| helper_intrinsic("COREDLL.dll", name).unwrap();
    let f = |v: f32| v.to_bits();
    let split = |v: f64| [v.to_bits() as u32, (v.to_bits() >> 32) as u32];
    let join = |r: [u32; 4]| f64::from_bits(u64::from(r[1]) << 32 | u64::from(r[0]));
    assert_eq!(
        call(op("__muls"), [f(1.5), f(-2.25), 7, 7])[..2],
        [f(-3.375), f(-2.25)]
    );
    assert_eq!(call(op("__adds"), [f(0.1), f(0.2), 0, 0])[0], f(0.1 + 0.2));
    assert_eq!(
        call(op("__divs"), [f(1.0), f(0.0), 0, 0])[0],
        f(f32::INFINITY)
    );
    assert_eq!(call(op("__negs"), [f(2.0), 9, 0, 0])[..2], [f(-2.0), 9]);
    assert_eq!(call(op("__stoi"), [f(-7.9), 0, 0, 0])[0], (-7i32) as u32);
    assert_eq!(call(op("__stoi"), [f(f32::NAN), 0, 0, 0])[0], 0);
    assert_eq!(call(op("__stoi"), [f(3e10), 0, 0, 0])[0], i32::MAX as u32);
    assert_eq!(call(op("__stou"), [f(-1.0), 0, 0, 0])[0], 0);
    assert_eq!(call(op("__itos"), [(-3i32) as u32, 0, 0, 0])[0], f(-3.0));
    let [a0, a1] = split(1.25);
    let [b0, b1] = split(-0.5);
    assert_eq!(join(call(op("__muld"), [a0, a1, b0, b1])), -0.625);
    assert_eq!(join(call(op("__subd"), [a0, a1, b0, b1])), 1.75);
    assert_eq!(call(op("__dtoi"), [a0, a1, 0, 0])[0], 1);
    assert_eq!(call(op("__dtos"), [a0, a1, 0, 0])[0], f(1.25));
    assert_eq!(join(call(op("__stod"), [f(0.75), 0, 0, 0])), 0.75);
    assert_eq!(join(call(op("__itod"), [(-9i32) as u32, 0, 0, 0])), -9.0);
    assert_eq!(
        call(op("__lts"), [f(1.0), f(2.0), 5, 0])[..3],
        [1, f(2.0), 5]
    );
    assert_eq!(call(op("__ges"), [f(1.0), f(2.0), 0, 0])[0], 0);
    assert_eq!(call(op("__cmps"), [f(1.0), f(2.0), 0, 0])[0], u32::MAX);
    assert_eq!(call(op("__cmps"), [f(f32::NAN), f(2.0), 0, 0])[0], 0);
    assert_eq!(call(op("__eqd"), [a0, a1, a0, a1])[0], 1);
    assert_eq!(call(op("__ltd"), [b0, b1, a0, a1])[0], 1);
    assert_eq!(call(op("__cmpd"), [a0, a1, b0, b1])[0], 1);
    assert!(helper_intrinsic("COREDLL.dll", "malloc").is_none());
    assert!(helper_intrinsic("gx.dll", "__muls").is_none());
}
