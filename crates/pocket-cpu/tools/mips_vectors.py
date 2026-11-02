"""Write tests/data/mips_vectors.txt: random MIPS programs and what Unicorn's
MIPS (the same engine the desktop build uses) leaves after running them.
tests/mips_vectors.rs replays them on the interpreter.

    python tools/mips_vectors.py   (needs `pip install unicorn`)
"""
import os, random
from unicorn import Uc, UC_ARCH_MIPS, UC_MODE_MIPS32, UC_MODE_LITTLE_ENDIAN
from unicorn.mips_const import *

CODE, DATA, DATA_SIZE = 0x10000, 0x40000, 0x400
REGS = [4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23]
GP = 28

def special(rs, rt, rd, sa, funct):
    return rs << 21 | rt << 16 | rd << 11 | sa << 6 | funct

def immediate(op, rs, rt, imm):
    return op << 26 | rs << 21 | rt << 16 | (imm & 0xFFFF)

def straight(r):
    d, s, t = r.choice(REGS), r.choice(REGS), r.choice(REGS)
    k = r.randrange(11)
    if k == 0:
        # The unused field must be zero: MIPS32 reads a 1 there as a rotate.
        funct = r.choice([0, 2, 3, 4, 6, 7])
        if funct < 4:
            return special(0, t, d, r.randrange(32), funct)
        return special(s, t, d, 0, funct)
    if k in (1, 2):
        return special(s, t, d, 0, r.choice([0x21, 0x23, 0x24, 0x25, 0x26, 0x27, 0x2A, 0x2B]))
    if k == 3:
        op = r.choice([9, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F])
        return immediate(op, 0 if op == 0x0F else s, d, r.getrandbits(16))
    if k == 4:
        return special(s, t, 0, 0, r.choice([0x18, 0x19]))
    if k == 5:
        return special(0, 0, d, 0, r.choice([0x10, 0x12]))
    if k == 6:
        return special(s, 0, 0, 0, r.choice([0x11, 0x13]))
    if k == 7:
        op, align = r.choice([(0x20, 1), (0x24, 1), (0x21, 2), (0x25, 2), (0x23, 4), (0x28, 1), (0x29, 2), (0x2B, 4)])
        return immediate(op, GP, t, r.randrange(128) * align)
    if k == 8:
        # DIV / DIVU by gp, which is never zero.
        return special(s, GP, 0, 0, r.choice([0x1A, 0x1B]))
    return immediate(r.choice([0x22, 0x26, 0x2A, 0x2E]), GP, t, r.randrange(DATA_SIZE // 2))

def branch(r):
    s, t = r.choice(REGS), r.choice(REGS)
    k = r.randrange(4)
    if k == 0:
        first = immediate(r.choice([4, 5, 0x14, 0x15]), s, t, 2)
    elif k == 1:
        first = immediate(r.choice([6, 7, 0x16, 0x17]), s, 0, 2)
    elif k == 2:
        first = immediate(1, s, r.choice([0, 1, 2, 3]), 2)
    else:
        first = immediate(1, s, r.choice([0x10, 0x11]), 2)
    return [first, straight(r), straight(r)]

def program(r):
    words = []
    while len(words) < 60:
        words += branch(r) if r.randrange(5) == 0 else [straight(r)]
    return words

def run(code, seed, data):
    uc = Uc(UC_ARCH_MIPS, UC_MODE_MIPS32 | UC_MODE_LITTLE_ENDIAN)
    uc.mem_map(CODE, 0x10000)
    uc.mem_map(DATA, 0x10000)
    blob = b''.join(w.to_bytes(4, 'little') for w in code)
    uc.mem_write(CODE, blob)
    uc.mem_write(DATA, data)
    reg = lambda n: UC_MIPS_REG_0 + n
    for n, v in zip(REGS, seed):
        uc.reg_write(reg(n), v)
    uc.reg_write(reg(GP), DATA + DATA_SIZE // 2)
    uc.reg_write(reg(31), 0)
    uc.emu_start(CODE, CODE + len(blob))
    regs = [uc.reg_read(reg(n)) & 0xFFFFFFFF for n in REGS + [31]]
    return regs, bytes(uc.mem_read(DATA, DATA_SIZE))

def main():
    r = random.Random(0x5EED)
    out = []
    for _ in range(300):
        code = program(r)
        seed = [r.choice([r.randrange(8), 0x80000000 | r.getrandbits(31), r.getrandbits(32), r.getrandbits(32)]) for _ in REGS]
        data = bytes(r.getrandbits(8) for _ in range(DATA_SIZE))
        regs, mem = run(code, seed, data)
        out.append('|'.join([
            ' '.join('%08x' % w for w in code),
            ' '.join('%08x' % v for v in seed),
            data.hex(),
            ' '.join('%08x' % v for v in regs),
            mem.hex(),
        ]))
    here = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(here, '..', 'tests', 'data', 'mips_vectors.txt'), 'w') as f:
        f.write('\n'.join(out) + '\n')
    print('wrote', len(out))

if __name__ == '__main__':
    main()
