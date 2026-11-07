"""Reuse the window probe's ELF-to-PE converter with the dialog probe's imports."""
import importlib.util
import json
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("window_elf2pe", here.parent / "wince-window-test" / "elf2pe.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
module.IMPORTS = json.loads((here / "imports.json").read_text())
module.main(sys.argv[1], sys.argv[2])
