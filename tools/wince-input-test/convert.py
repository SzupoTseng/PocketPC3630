"""Use the existing clean-room ARM PE converter with this probe's imports."""
import importlib.util
import json
from pathlib import Path
import sys
here = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('window_pe', here.parent / 'wince-window-test/elf2pe.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
module.IMPORTS = json.loads((here / 'imports.json').read_text())
module.main(sys.argv[1], sys.argv[2])
