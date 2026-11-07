// Smoke test of a supplied WASM build; optional pointer-only scenarios.
// node smoke.mjs <build-dir> <package> <output-dir> [seconds=20] [scenario.json]
import { createRequire } from 'node:module';
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve, join, basename } from 'node:path';
import { createHash } from 'node:crypto';
import { parseScenario } from './scenario.mjs';

const [buildArg, packageArg, outputArg, secondsArg = '20', scenarioArg, saveArg] = process.argv.slice(2);
const seconds = Number(secondsArg);
if (!buildArg || !packageArg || !outputArg || !Number.isFinite(seconds) || seconds < 1 || seconds > 120) {
  throw new Error('Usage: node smoke.mjs <build-dir> <package> <output-dir> [seconds: 1..120]');
}
const build = resolve(buildArg), input = resolve(packageArg), output = resolve(outputArg);
mkdirSync(output, { recursive: true });
const hash = (data) => createHash('sha256').update(data).digest('hex');
const actions = scenarioArg ? parseScenario(JSON.parse(readFileSync(scenarioArg, 'utf8')), seconds) : [];
const report = {
  date: new Date().toISOString(),
  scope: 'supplied WASM build; pointer-only input; registration/gameplay require separate verification; serial events are not unique frames',
  scenario: scenarioArg ? basename(scenarioArg) : null, actions: [],
  build: {}, package: { name: basename(input) },
  imported: false, booted: false, stopped: false, error: null,
  ticks: 0, presentationEvents: 0, screenshots: [], warnings: [],
};
let M;
function screenshot(name) {
  const w = M._pocket_width(), h = M._pocket_height();
  if (!w || !h || w * h > 4096 * 4096) throw new Error('Invalid framebuffer size');
  const ptr = M._pocket_frame_ptr();
  const rgba = M.HEAPU8.subarray(ptr, ptr + w * h * 4);
  if (rgba.length !== w * h * 4) throw new Error('Framebuffer outside WASM memory');
  const rgb = Buffer.alloc(w * h * 3);
  for (let i = 0; i < w * h; i++) rgb.set(rgba.subarray(i * 4, i * 4 + 3), i * 3);
  writeFileSync(join(output, name + '.ppm'), Buffer.concat([Buffer.from(`P6\n${w} ${h}\n255\n`), rgb]));
  report.screenshots.push({ file: name + '.ppm', width: w, height: h, rgbSha256: hash(rgb) });
}
try {
  for (const file of ['pockethle.js', 'pockethle.wasm']) report.build[file] = hash(readFileSync(join(build, file)));
  const bytes = readFileSync(input);
  report.package.sha256 = hash(bytes);
  const factory = createRequire(import.meta.url)(join(build, 'pockethle.js'));
  M = await factory({
    locateFile: (name) => join(build, name),
    printErr: (line) => { if (report.warnings.length < 200) report.warnings.push(String(line)); },
  });
  M.FS.mkdir('/library');
  const path = '/' + basename(input);
  M.FS.writeFile(path, bytes);
  const pointer = M.stringToNewUTF8(path);
  try {
    if (M._pocket_import(pointer) !== 0) throw new Error(M.UTF8ToString(M._pocket_last_error()));
  } finally { M._free(pointer); }
  report.imported = true;
  report.modules = [];
  let visited = 0;
  function inventory(path, depth = 0) {
    if (depth > 16) throw new Error('Module inventory depth limit');
    for (const name of M.FS.readdir(path)) {
      if (name === '.' || name === '..') continue;
      if (++visited > 20000) throw new Error('Module inventory file limit');
      const child = path + '/' + name, stat = M.FS.stat(child);
      if (M.FS.isDir(stat.mode)) inventory(child, depth + 1);
      else if (/\.(exe|dll)$/i.test(name)) {
        report.modules.push({ path: child, bytes: stat.size,
          sha256: stat.size <= 64 * 1024 * 1024 ? hash(M.FS.readFile(child)) : null });
      }
    }
  }
  inventory('/library');
  report.games = M._pocket_game_count();
  if (report.games < 1) throw new Error("Import produced no games");
  if (saveArg) {
    const save = readFileSync(saveArg);
    M.FS.writeFile('/progress.zip', save);
    const pointer = M.stringToNewUTF8('/progress.zip');
    try {
      const count = M._pocket_save_import(0, pointer);
      if (count < 0) throw new Error(M.UTF8ToString(M._pocket_last_error()) || 'Save import rejected');
      report.saveImport = { name: basename(saveArg), bytes: save.length, sha256: hash(save), files: count };
    } finally { M._free(pointer); }
  }
  if (M._pocket_boot(0) !== 0) throw new Error(M.UTF8ToString(M._pocket_last_error()));
  report.booted = true;
  const start = performance.now();
  let lastSerial = M._pocket_frame_serial(), nextShot = 5;
  while (performance.now() - start < seconds * 1000) {
    const now = (performance.now() - start) / 1000;
    while (actions.length && actions[0].at <= now) {
      const action = actions.shift();
      if (action.pointer) {
        const [, x, y] = action.pointer;
        if (x >= M._pocket_width() || y >= M._pocket_height()) throw new Error('Tap outside screen');
        M._pocket_pointer(...action.pointer);
      } else screenshot(action.shot);
      report.actions.push({ ...action, actualSeconds: now });
    }
    const delay = M._pocket_tick();
    report.ticks++;
    if (delay < 0) {
      report.stopped = true;
      report.error = M.UTF8ToString(M._pocket_last_error()) || null;
      report.stopReason = report.error ? 'error' : 'clean-exit';
      break;
    }
    const serial = M._pocket_frame_serial();
    if (serial !== lastSerial) { report.presentationEvents++; lastSerial = serial; }
    const elapsed = (performance.now() - start) / 1000;
    if (elapsed >= nextShot) { screenshot('at-' + nextShot + 's'); nextShot += 5; }
    await new Promise((done) => setTimeout(done, Math.min(Math.max(delay, 0), 100)));
  }
  report.wallSeconds = (performance.now() - start) / 1000;
  screenshot('final');
} catch (error) {
  report.error = String(error.stack || error);
} finally {
  report.distinctCapturedFrames = new Set(report.screenshots.map((shot) => shot.rgbSha256)).size;
  writeFileSync(join(output, 'report.json'), JSON.stringify(report, null, 2) + '\n');
}
console.log(JSON.stringify({ imported: report.imported, booted: report.booted, ticks: report.ticks,
  presentationEvents: report.presentationEvents, error: report.error, report: join(output, 'report.json') }));
process.exit(report.error ? 1 : 0);
