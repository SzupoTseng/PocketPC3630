"""Write tests/data/sh_vectors.txt: random SuperH programs and what QEMU's SH-4
leaves after running them. The SH-4's integer instructions are the SH-3's.
tests/sh_vectors.rs replays the same block of code on the interpreter.

Each program is a tiny static Linux ELF: a prologue loads the registers from a
table, the random block runs, an epilogue stores them and writes them and the
data area to stdout. Nothing but the block is replayed.

    python tools/sh_vectors.py <path to qemu-sh4>
"""
import os, random, struct, subprocess, sys, tempfile

BASE = 0x00400000          # code
DATA = 0x00500000          # data segment: table, test area, output
TABLE = DATA               # initial r0..r13, mach, macl, T, gbr
AREA = DATA + 0x400        # bytes the block reads and writes
AREA_SIZE = 0x400
MID = AREA + AREA_SIZE // 2  # r14 and GBR point here
OUT = DATA + 0x100
REGS = list(range(14))      # r0..r13 are random; r14 is the base, r15 the stack

def u16(x): return struct.pack('<H', x & 0xFFFF)

def straight(r):
    """One instruction that does not branch and only touches the test area."""
    n, m = r.choice(REGS), r.choice(REGS)
    k = r.randrange(24)
    if k == 0: return 0x3000 | n << 8 | m << 4 | r.choice([0x0, 0x2, 0x3, 0x6, 0x7])   # CMP/xx
    # ADDV and SUBV are left out: QEMU 8.2 computes their overflow bit wrongly
    # (fixed upstream in 2024), so tests/sh_vectors.rs checks them by hand.
    if k == 1: return 0x3000 | n << 8 | m << 4 | r.choice([0x8, 0xA, 0xC, 0xE])  # SUB/SUBC/ADD/ADDC
    if k == 2: return 0x2000 | n << 8 | m << 4 | r.choice([0x8, 0x9, 0xA, 0xB, 0xC, 0xD])  # TST AND XOR OR CMP/STR XTRCT
    if k == 3: return 0x6000 | n << 8 | m << 4 | r.choice([0x3, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF])
    # ROTL and ROTR are left out: QEMU 8.2 always clears T for them, where the
    # manual copies the bit rotated out; tests/sh_vectors.rs checks them by hand.
    if k == 4: return 0x4000 | n << 8 | r.choice([0x00, 0x01, 0x08, 0x09, 0x10, 0x11, 0x15, 0x18, 0x19, 0x20, 0x21, 0x24, 0x25, 0x28, 0x29])
    if k == 5: return 0x400C | n << 8 | m << 4 | r.choice([0, 1])                        # SHAD SHLD
    if k == 6: return 0x7000 | n << 8 | r.getrandbits(8)                                  # ADD #imm
    if k == 7: return 0xE000 | n << 8 | r.getrandbits(8)                                  # MOV #imm
    if k == 8: return r.choice([0x8800, 0xC800, 0xC900, 0xCA00, 0xCB00]) | r.getrandbits(8)  # CMP/EQ TST AND XOR OR #imm,R0
    if k == 9: return 0x0007 | n << 8 | m << 4                                            # MUL.L
    if k == 10: return 0x2000 | n << 8 | m << 4 | r.choice([0xE, 0xF])                     # MULU.W MULS.W
    if k == 11: return 0x3000 | n << 8 | m << 4 | r.choice([0x5, 0xD])                     # DMULU DMULS
    if k == 12: return r.choice([0x000A, 0x001A, 0x400A, 0x401A]) | n << 8                 # STS / LDS MACH MACL
    if k == 13: return r.choice([0x0029 | n << 8, 0x0008, 0x0018, 0x0019, 0x2007 | n << 8 | m << 4])  # MOVT CLRT SETT DIV0U DIV0S
    if k == 14:
        # DIV1 comes as a sequence (see branch()), never alone: QEMU 8.2 takes a
        # zero divisor as a borrow, which the manual does not, so the divisor
        # is always loaded with a non-zero value first.
        return 0x0009
    if k == 15: return 0x1000 | 14 << 8 | m << 4 | r.randrange(16)                        # MOV.L Rm,@(disp,r14)
    if k == 16: return 0x5000 | n << 8 | 14 << 4 | r.randrange(16)                        # MOV.L @(disp,r14),Rn
    if k == 17: return r.choice([0x8000, 0x8100, 0x8400, 0x8500]) | 14 << 4 | r.randrange(16)  # MOV.B/W R0 <-> @(disp,r14)
    if k == 18: return r.choice([0xC000, 0xC100, 0xC200, 0xC400, 0xC500, 0xC600]) | r.randrange(64)  # GBR forms
    if k == 19: return 0x2000 | 14 << 8 | m << 4 | r.choice([0x0, 0x1, 0x2])              # MOV.x Rm,@r14
    if k == 20: return 0x6000 | n << 8 | 14 << 4 | r.choice([0x0, 0x1, 0x2])              # MOV.x @r14,Rn
    if k == 21: return 0x4010 | n << 8                                                    # DT
    if k == 22: return 0xC700 | r.randrange(16)                                           # MOVA
    return 0xD000 | n << 8 | r.randrange(16)                                              # MOV.L @(disp,PC)

def branch(r):
    k = r.randrange(5)
    if k == 0:  # BT / BF over one instruction
        return [r.choice([0x8900, 0x8B00]) | 0, straight(r)]
    if k == 1:  # BT/S / BF/S: delay slot, then skip one
        return [r.choice([0x8D00, 0x8F00]) | 1, straight(r), straight(r)]
    if k == 2:  # BRA over one instruction, with its delay slot
        return [0xA001, straight(r), straight(r)]
    if k == 3:  # a DIV1 sequence as compilers emit it
        n = r.choice([x for x in REGS if x != 13])
        divisor = r.choice([x for x in range(-128, 128) if x != 0]) & 0xFF
        return ([0xED00 | divisor, r.choice([0x2007 | n << 8 | 13 << 4, 0x0019])]
                + [0x3004 | n << 8 | 13 << 4] * r.randrange(1, 9))
    return [0x0029 | r.choice(REGS) << 8]

def block(r):
    words = []
    while len(words) < 50:
        words += branch(r) if r.randrange(4) == 0 else [straight(r)]
    return words

def build(r):
    """Return (elf bytes, block offset, block length, init words, area)."""
    code = []
    pool = []        # (index of the instruction, value)
    def literal(rn, value):
        pool.append((len(code), value))
        code.append(0xD000 | rn << 8)          # MOV.L @(disp,PC),Rn, fixed below
    literal(14, TABLE)
    code.append(0x50E0 | 0 << 8 | 14)           # mov.l @(56,r14),r0 -> mach
    code.append(0x400A)                         # lds r0,mach
    code.append(0x50EF & 0xF0FF | 0 << 8)       # mov.l @(60,r14),r0 -> macl
    code[-1] = 0x5000 | 0 << 8 | 14 << 4 | 15
    code.append(0x401A)                         # lds r0,macl
    code.append(0x0019)                         # div0u: Q = M = 0, as the replay starts
    code.append(0x60E3)                         # mov r14,r0
    code.append(0x7040)                         # add #64,r0
    code.append(0x6002)                         # mov.l @r0,r0 (T)
    code.append(0x4001)                         # shlr r0: T = bit 0
    literal(14, MID)
    code.append(0x4E1E)                         # ldc r14,gbr
    literal(13, TABLE)
    for n in range(1, 13):
        code.append(0x5000 | n << 8 | 13 << 4 | n)   # mov.l @(n*4,r13),rn
    code.append(0x5000 | 0 << 8 | 13 << 4 | 0)      # r0
    code.append(0x5000 | 13 << 8 | 13 << 4 | 13)    # r13 last
    start = len(code)
    test = block(r)
    code += test
    end = len(code)
    literal(14, OUT)
    for n in range(14):
        code.append(0x1000 | 14 << 8 | n << 4 | n)   # mov.l rn,@(n*4,r14)
    code.append(0x0029)                              # movt r0
    code.append(0x1000 | 14 << 8 | 0 << 4 | 14)
    code.append(0x000A)                              # sts mach,r0
    code.append(0x1000 | 14 << 8 | 0 << 4 | 15)
    code.append(0x7E40)                              # add #64,r14
    code.append(0x001A)                              # sts macl,r0
    code.append(0x2E02)                              # mov.l r0,@r14
    code.append(0x0012)                              # stc gbr,r0
    code.append(0x1E01)                              # mov.l r0,@(4,r14)
    code += [0xE304, 0xE401]                         # write(1, OUT, 72)
    literal(5, OUT)
    code += [0xE648, 0xC317]
    code += [0xE304, 0xE401]                         # write(1, AREA, AREA_SIZE)
    literal(5, AREA)
    literal(6, AREA_SIZE)
    code += [0xC317, 0xE301, 0xE400, 0xC317, 0x0009]
    # Literal pool after the code, word aligned; fix each MOV.L @(disp,PC).
    if len(code) % 2:
        code.append(0x0009)
    pool_at = len(code) * 2
    words = list(code)
    blob = b''.join(u16(w) for w in words)
    lits = b''
    for k, (index, value) in enumerate(pool):
        address = BASE + pool_at + 4 * k
        pc = BASE + index * 2
        disp = (address - ((pc & ~3) + 4)) // 4
        assert 0 <= disp < 256
        words[index] |= disp
        lits += struct.pack('<I', value)
    blob = b''.join(u16(w) for w in words) + lits + b'\0' * 64
    init = [r.choice([r.randrange(8), 0x80000000 | r.getrandbits(31), r.getrandbits(32), r.getrandbits(32)]) for _ in REGS]
    init += [r.getrandbits(32), r.getrandbits(32), r.getrandbits(1)]   # mach macl T
    area = bytes(r.getrandbits(8) for _ in range(AREA_SIZE))
    data = struct.pack('<16I', *(init[:14] + init[14:16])) + struct.pack('<I', init[16])
    data = data.ljust(AREA - DATA, b'\0') + area
    return elf(blob, data), start * 2, (end - start) * 2, init, area, blob

def elf(code, data):
    ehsize, phsize = 52, 32
    off_code = 0x1000
    off_data = off_code + ((len(code) + 0xFFF) & ~0xFFF)
    eh = b'\x7fELF' + bytes([1, 1, 1, 0]) + b'\0' * 8 + struct.pack(
        '<HHIIIIIHHHHHH', 2, 42, 1, BASE, ehsize, 0, 0, ehsize, phsize, 2, 40, 0, 0)
    ph = struct.pack('<IIIIIIII', 1, off_code, BASE, BASE, len(code), len(code), 5, 0x1000)
    ph += struct.pack('<IIIIIIII', 1, off_data, DATA, DATA, len(data), len(data), 6, 0x1000)
    out = bytearray(eh + ph)
    out += b'\0' * (off_code - len(out)) + code
    out += b'\0' * (off_data - len(out)) + data
    return bytes(out)

def main():
    qemu = sys.argv[1]
    r = random.Random(0x5E3)
    lines = []
    with tempfile.TemporaryDirectory() as tmp:
        path = os.path.join(tmp, 't.elf')
        while len(lines) < 300:
            program, start, length, init, area, code = build(r)
            open(path, 'wb').write(program)
            os.chmod(path, 0o755)
            run = subprocess.run([qemu, path], capture_output=True, timeout=20)
            if run.returncode != 0 or len(run.stdout) != 72 + AREA_SIZE:
                continue   # an exception in the block; covered elsewhere
            out = run.stdout
            regs = struct.unpack('<18I', out[:72])
            lines.append('|'.join([
                '%x %x %x' % (BASE, start, length),
                code.hex(),
                ' '.join('%08x' % v for v in init),
                area.hex(),
                ' '.join('%08x' % v for v in regs),
                out[72:].hex(),
            ]))
    here = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(here, '..', 'tests', 'data', 'sh_vectors.txt'), 'w') as f:
        f.write('\n'.join(lines) + '\n')
    print('wrote', len(lines))

if __name__ == '__main__':
    main()
