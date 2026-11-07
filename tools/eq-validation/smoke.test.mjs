import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, writeFileSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

function fixture(importResult, requireSave = false) {
  const root = mkdtempSync(join(tmpdir(), 'eq-validation-'));
  writeFileSync(join(root, 'package.zip'), 'fixture');
  writeFileSync(join(root, 'pockethle.wasm'), 'fixture');
  // This mocks only the runner's contract; it is not a guest emulation test.
  writeFileSync(join(root, 'pockethle.js'), `let restored = false; module.exports = async () => ({
    FS: { mkdir() {}, writeFile() {}, readdir() { return []; } }, HEAPU8: Buffer.from([255,0,0,255]),
    stringToNewUTF8: () => 1, UTF8ToString: () => ${importResult ? "'import rejected'" : "''"},
    _free() {}, _pocket_import: () => ${importResult}, _pocket_game_count: () => 1,
    _pocket_save_import: () => { restored = true; return 3; },
    _pocket_boot: () => ${requireSave ? "restored ? 0 : -1" : "0"}, _pocket_tick: () => -1, _pocket_last_error: () => 0,
    _pocket_width: () => 1, _pocket_height: () => 1,
    _pocket_frame_ptr: () => 0, _pocket_frame_serial: () => 0,
  });`);
  return root;
}

for (const rejected of [false, true]) {
  test(rejected ? 'import rejection leaves a failure report' : 'clean exit captures a valid image without reporting a crash', () => {
    const root = fixture(rejected ? -1 : 0);
    try {
      const output = join(root, 'output');
      const result = spawnSync(process.execPath, [resolve('tools/eq-validation/smoke.mjs'), root,
        join(root, 'package.zip'), output, '1'], { encoding: 'utf8', timeout: 10000 });
      assert.equal(result.status, rejected ? 1 : 0, result.stderr);
      const report = JSON.parse(readFileSync(join(output, 'report.json')));
      assert.equal(report.imported, !rejected);
      if (rejected) assert.match(report.error, /import rejected/);
      else {
        assert.equal(report.error, null);
        assert.equal(report.stopReason, 'clean-exit');
        assert.equal(report.distinctCapturedFrames, 1);
        assert.deepEqual(readFileSync(join(output, 'final.ppm')), Buffer.concat([
          Buffer.from('P6\n1 1\n255\n'), Buffer.from([255, 0, 0]),
        ]));
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

test('supplied progress is restored before the guest boots', () => {
  const root = fixture(0, true);
  try {
    writeFileSync(join(root, 'scenario.json'), '[]');
    writeFileSync(join(root, 'progress.zip'), 'mock progress');
    const output = join(root, 'output');
    const result = spawnSync(process.execPath, [resolve('tools/eq-validation/smoke.mjs'), root,
      join(root, 'package.zip'), output, '1', join(root, 'scenario.json'), join(root, 'progress.zip')],
      { encoding: 'utf8', timeout: 10000 });
    assert.equal(result.status, 0, result.stderr);
    const report = JSON.parse(readFileSync(join(output, 'report.json')));
    assert.equal(report.booted, true);
    assert.equal(report.saveImport.files, 3);
    assert.equal(report.saveImport.bytes, 13);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
