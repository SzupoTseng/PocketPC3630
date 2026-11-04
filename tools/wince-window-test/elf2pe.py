"""Turn the linked ARM ELF program into a Windows CE PE executable.

The ELF is linked at the PE's own addresses (link.ld), so its sections are
copied as they are. The import table is built here: the program calls
coredll through the `IAT` array, whose slots follow the order of IMPORTS.
Usage: python elf2pe.py window-test.elf window-test.exe
"""
import struct
import sys

IMPORTS = [
    "RegisterClassW", "CreateWindowExW", "DestroyWindow", "DefWindowProcW",
    "GetParent", "GetWindow", "GetActiveWindow", "GetFocus", "SetFocus",
    "GetForegroundWindow", "IsWindow", "PostThreadMessageW", "PeekMessageW",
    "GetWindowThreadProcessId", "CreateFileW", "WriteFile", "CloseHandle",
    "ShowWindow", "SetActiveWindow", "PostQuitMessage", "DispatchMessageW",
    "PostMessageW", "SendMessageW", "SetForegroundWindow", "EnableWindow",
]
BASE = 0x10000
ALIGN = 0x1000
FILE_ALIGN = 0x200


def up(n, a):
    return (n + a - 1) // a * a


def read_elf(path):
    b = open(path, "rb").read()
    assert b[:4] == b"\x7fELF" and b[4] == 1, "32-bit ELF expected"
    entry = struct.unpack_from("<I", b, 24)[0]
    shoff, = struct.unpack_from("<I", b, 32)
    shentsize, shnum, shstrndx = struct.unpack_from("<HHH", b, 46)
    heads = [struct.unpack_from("<IIIIIIIIII", b, shoff + i * shentsize) for i in range(shnum)]
    names = heads[shstrndx]
    def name_of(off):
        start = names[4] + off
        return b[start:b.index(b"\0", start)].decode()
    sections, symbols = {}, {}
    for h in heads:
        sections[name_of(h[0])] = h
    for h in heads:
        if h[1] == 2:  # SYMTAB
            strtab = heads[h[6]]
            for i in range(h[5] // 16):
                st_name, value, size, info, other, shndx = struct.unpack_from("<IIIBBH", b, h[4] + i * 16)
                start = strtab[4] + st_name
                symbols[b[start:b.index(b"\0", start)].decode()] = value
    def content(name):
        h = sections.get(name)
        if not h:
            return None, 0, 0
        addr, off, size, typ = h[3], h[4], h[5], h[1]
        data = b"" if typ == 8 else b[off:off + size]  # NOBITS
        return data, addr, size
    return entry, content, symbols


def main(src, dst):
    entry, content, symbols = read_elf(src)
    iat = symbols["IAT"]
    out = []
    for name, flags in ((".text", 0x60000020), (".rdata", 0x40000040), (".data", 0xC0000040)):
        data, addr, size = content(name)
        if data is None:
            continue
        out.append([name, addr, size, bytearray(data), flags])
    end = max(addr + size for _, addr, size, _, _ in out)
    idata_rva = up(end, ALIGN) - BASE

    # Import directory: one descriptor and a null one, the lookup table,
    # hint/name entries, the DLL name. The IAT itself lives in .data.
    idata = bytearray(40)
    ilt_at = len(idata)
    idata += bytes(4 * (len(IMPORTS) + 1))
    hints = []
    for name in IMPORTS:
        if len(idata) % 2:
            idata += b"\0"
        hints.append(idata_rva + len(idata))
        idata += struct.pack("<H", 0) + name.encode() + b"\0"
    dll_at = idata_rva + len(idata)
    idata += b"COREDLL.dll\0"
    for i, rva in enumerate(hints):
        struct.pack_into("<I", idata, ilt_at + 4 * i, rva)
    struct.pack_into("<IIIII", idata, 0, idata_rva + ilt_at, 0, 0, dll_at, iat - BASE)
    out.append([".idata", BASE + idata_rva, len(idata), idata, 0xC0000040])

    # The IAT starts as a copy of the lookup table, as linkers write it.
    for sec in out:
        if sec[0] == ".data":
            off = iat - sec[1]
            for i, rva in enumerate(hints):
                if off + 4 * i + 4 > len(sec[3]):
                    sec[3].extend(bytes(off + 4 * i + 4 - len(sec[3])))
                struct.pack_into("<I", sec[3], off + 4 * i, rva)

    headers = 0x400
    pe = bytearray(headers)
    pe[0:2] = b"MZ"
    struct.pack_into("<I", pe, 0x3C, 0x80)
    pe[0x80:0x84] = b"PE\0\0"
    struct.pack_into("<HHIIIHH", pe, 0x84, 0x01C0, len(out), 0, 0, 0, 0xE0, 0x0103)
    raw = headers
    table = 0x80 + 24 + 0xE0
    image_end = 0
    body = bytearray()
    for i, (name, addr, size, data, flags) in enumerate(out):
        rva = addr - BASE
        raw_size = up(len(data), FILE_ALIGN)
        vsize = max(size, len(data))
        struct.pack_into("<8sIIIIIIHHI", pe, table + 40 * i, name.encode(), vsize, rva,
                         raw_size, raw if raw_size else 0, 0, 0, 0, 0, flags)
        body += bytes(data) + bytes(raw_size - len(data))
        raw += raw_size
        image_end = max(image_end, up(rva + vsize, ALIGN))
    code = next(s for s in out if s[0] == ".text")
    o = 0x98
    struct.pack_into("<HBBIIIIII", pe, o, 0x10B, 6, 0, up(len(code[3]), FILE_ALIGN), 0, 0,
                     entry - BASE, code[1] - BASE, 0)
    struct.pack_into("<IIIHHHHHHIIIIHHIIIIII", pe, o + 28, BASE, ALIGN, FILE_ALIGN,
                     4, 0, 0, 0, 4, 20, 0, image_end, headers, 0, 9, 0,
                     0x10000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = o + 96
    struct.pack_into("<II", pe, dirs + 8, idata_rva, 40)
    struct.pack_into("<II", pe, dirs + 12 * 8, iat - BASE, 4 * (len(IMPORTS) + 1))
    open(dst, "wb").write(bytes(pe) + bytes(body))
    print(f"{dst}: {len(pe) + len(body)} bytes, entry 0x{entry:x}, {len(IMPORTS)} imports")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
