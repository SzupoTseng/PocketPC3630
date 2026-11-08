"""Packaging safety checks in an isolated fake project; no SDK/Gradle required."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

PACKAGER = Path(__file__).with_name('package-ui-android.py')
UNICORN = b'Unicorn (ARM, MIPS) / interpreter (SH3)'
INTERPRETER = b'interpreter (ARM, MIPS, SH3)'


class PackageSafety(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='ui-package-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        script = self.root / 'tools/eq-validation/package-ui-android.py'
        script.parent.mkdir(parents=True)
        shutil.copyfile(PACKAGER, script)
        self.script = script
        (self.root / 'target').mkdir()
        self.native = self.root / 'frontends/pocket-android/app/src/main/jniLibs'
        self.interpreter = self.root / 'interpreter'
        for abi in ['arm64-v8a', 'x86_64']:
            for folder, content in [(self.native, UNICORN), (self.interpreter, INTERPRETER)]:
                path = folder / abi / 'libpockethle_jni.so'
                path.parent.mkdir(parents=True)
                path.write_bytes(content + abi.encode())
        self.apk = self.root / 'frontends/pocket-android/app/build/outputs/apk/debug/app-debug.apk'
        self.apk.parent.mkdir(parents=True)
        self.apk.write_bytes(b'original default APK')
        fake = self.root / 'fake-gradle.py'
        fake.write_text('''#!/usr/bin/env python3
import os,sys,zipfile
from pathlib import Path
root=Path.cwd()
apk=root/'frontends/pocket-android/app/build/outputs/apk/debug/app-debug.apk'
source=(root/'frontends/pocket-android/app/src/main/jniLibs/arm64-v8a/libpockethle_jni.so').read_bytes()
backend='unicorn' if b'Unicorn' in source else 'interpreter'
apk.unlink(missing_ok=True)
if os.environ.get('UI_TEST_FAILURE')==backend:sys.exit(3)
apk.parent.mkdir(parents=True,exist_ok=True)
markers=b'Program ended. Return to the library to start again.\\0Could not start program. Return to the library to retry.\\0apply_run_limits\\0Lcom/pockethle/app/ControlsActivity;'
if os.environ.get('UI_TEST_FAILURE')=='stale-kotlin':markers=b'old managed UI'
with zipfile.ZipFile(apk,'w') as archive:
 archive.writestr('classes.dex',markers)
 archive.writestr('backend.txt',backend)
''')
        if os.name == 'nt':
            launcher = self.root / 'fake-gradle.cmd'
            launcher.write_text(f'@echo off\n"{sys.executable}" "{fake}" %*\n')
            self.gradle = launcher
        else:
            fake.chmod(0o755)
            self.gradle = fake

    def run_packager(self, failure=''):
        env = os.environ.copy()
        env['UI_TEST_FAILURE'] = failure
        return subprocess.run([sys.executable, str(self.script), '--interpreter-jni',
                               str(self.interpreter), '--gradle', str(self.gradle)],
                              env=env, capture_output=True, text=True)

    def assert_native_restored(self):
        for abi in ['arm64-v8a', 'x86_64']:
            self.assertEqual((self.native / abi / 'libpockethle_jni.so').read_bytes(),
                             UNICORN + abi.encode())

    def test_success_retains_two_distinct_apks_and_default_unicorn(self):
        result = self.run_packager()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assert_native_restored()
        output = self.root / 'target/ui-validation'
        records = json.loads((output / 'android-builds.json').read_text())
        self.assertEqual(set(records), {'interpreter', 'unicorn'})
        self.assertNotEqual(records['interpreter']['sha256'], records['unicorn']['sha256'])
        for backend, record in records.items():
            self.assertEqual(record['sha256'], hashlib.sha256((output / record['apk']).read_bytes()).hexdigest())
        self.assertEqual(self.apk.read_bytes(), (output / records['unicorn']['apk']).read_bytes())

    def test_interpreter_failure_restores_default_inputs_and_apk(self):
        result = self.run_packager('interpreter')
        self.assertNotEqual(result.returncode, 0)
        self.assert_native_restored()
        self.assertEqual(self.apk.read_bytes(), b'original default APK')

    def test_final_unicorn_failure_restores_default_apk_after_clean(self):
        result = self.run_packager('unicorn')
        self.assertNotEqual(result.returncode, 0)
        self.assert_native_restored()
        self.assertEqual(self.apk.read_bytes(), b'original default APK')

    def test_current_jni_with_stale_managed_ui_is_rejected(self):
        result = self.run_packager('stale-kotlin')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('stale managed UI', result.stderr)
        self.assert_native_restored()
        self.assertEqual(self.apk.read_bytes(), b'original default APK')


if __name__ == '__main__':
    unittest.main()
