"""Compare a window-test log from PocketPC3630 with the Device Emulator
reference, step by step.

Usage: python compare.py ours.txt [reference/device-emulator-ppc2003.txt]

Each log is split into steps at the lines that do not start with spaces
("create A", "show C", "state after ..."). For each step it prints the
lines only one side has, so a difference reads as "the device sends WM_SIZE
to B here, we do not".
"""
import difflib
import sys
from pathlib import Path

MESSAGES = {
    0x1: "WM_CREATE", 0x2: "WM_DESTROY", 0x3: "WM_MOVE", 0x5: "WM_SIZE",
    0x6: "WM_ACTIVATE", 0x7: "WM_SETFOCUS", 0x8: "WM_KILLFOCUS", 0xa: "WM_ENABLE",
    0xf: "WM_PAINT", 0x14: "WM_ERASEBKGND", 0x18: "WM_SHOWWINDOW",
    0x47: "WM_WINDOWPOSCHANGED", 0x86: "WM_NCACTIVATE", 0x281: "WM_IME_SETCONTEXT",
    0x282: "WM_IME_NOTIFY", 0x30f: "WM_QUERYNEWPALETTE",
}


def named(line):
    parts = line.split()
    if len(parts) >= 3 and parts[0] == "msg":
        try:
            parts[2] = MESSAGES.get(int(parts[2], 16), parts[2])
        except ValueError:
            pass
    return " ".join(parts)


def steps(path):
    out, title, body = [], "start", []
    for line in Path(path).read_text().splitlines():
        if not line.startswith(" "):
            out.append((title, body))
            # "state after X: focus=..." is the step "state after X" whose
            # first line is what it found.
            title, _, found = line.partition(": ")
            body = [found] if found else []
        else:
            body.append(named(line.strip()))
    out.append((title, body))
    return out


def main(ours, reference):
    a, b = steps(reference), steps(ours)
    differences = 0
    for (ta, la), (tb, lb) in zip(a, b):
        if ta != tb:
            print(f"!! steps out of step: device '{ta}' / ours '{tb}'")
            differences += 1
            break
        if la == lb:
            continue
        differences += 1
        print(f"== {ta}")
        for line in difflib.unified_diff(la, lb, "device", "ours", lineterm="", n=0):
            if not line.startswith(("---", "+++", "@@")):
                print("   " + line)
    print(f"{differences} step(s) differ")


if __name__ == "__main__":
    here = Path(__file__).parent
    main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else here / "reference" / "device-emulator-ppc2003.txt")
