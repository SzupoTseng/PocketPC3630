// Runs the real WASM build in Node through the page's own call sequence:
// import a package, boot, pump pocket_tick with taps, and report the result.
// Usage: node verify.mjs <dir with pockethle.js/.wasm> <package.zip> <out.ppm> [seconds] [taps as JSON: [[second,x,y] or [second,"key",vk],..]]
// Without taps it plays through EverQuest's character creation.
import { createRequire } from 'node:module';
import { readFileSync, writeFileSync } from 'node:fs';
import { basename, resolve, join } from 'node:path';

const [buildDir, packagePath, outPath, secondsArg, tapsArg] = process.argv.slice(2);
const seconds = Number(secondsArg ?? 40);
const require = createRequire(import.meta.url);
const factory = require(resolve(buildDir, 'pockethle.js'));
const M = await factory({ locateFile: (name) => join(resolve(buildDir), name) });

M.FS.mkdir('/library');
// The library names a game after its file and picks the importer by extension.
const guestName = '/' + basename(packagePath);
M.FS.writeFile(guestName, readFileSync(packagePath));
const path = M.stringToNewUTF8(guestName);
const lastError = () => M.UTF8ToString(M._pocket_last_error());
if (M._pocket_import(path) !== 0) throw new Error(lastError());
console.log('games', M._pocket_game_count());
if (M._pocket_boot(0) !== 0) throw new Error(lastError());
const [w, h] = [M._pocket_width(), M._pocket_height()];
console.log('screen', w, h);

const tap = (x, y) => {
  M._pocket_pointer(0, x, y);
  M._pocket_pointer(1, x, y);
  M._pocket_pointer(2, x, y);
};
// Time-scheduled script: choose slot 1, make a character, skip the tutorial.
const script = [
  [9, () => tap(138, 45)],
  [12, () => tap(53, 25)],
  [13, () => tap(60, 206)],
  [14, () => tap(140, 208)],
  [15, () => tap(121, 307)],
];
for (let i = 0; i < 13; i++) script.push([24 + i, () => tap(120, 100)]);
script.push([38, () => tap(41, 303)]);

if (tapsArg) {
  script.length = 0;
  for (const [second, x, y] of JSON.parse(tapsArg)) {
    // [second, "key", vk] presses and releases half a second later.
    if (x === 'key') {
      script.push([second, () => M._pocket_key(1, y)]);
      script.push([second + 0.5, () => M._pocket_key(0, y)]);
    } else script.push([second, () => tap(x, y)]);
  }
}

const start = performance.now();
let busy = 0, ticks = 0, waited = 0, serial = 0, frames = 0;
const saveFrame = (file) => {
  const p = M._pocket_frame_ptr();
  const rgba = M.HEAPU8.subarray(p, p + w * h * 4);
  const rgb = Buffer.alloc(w * h * 3);
  for (let i = 0; i < w * h; i++) {
    rgb[i * 3] = rgba[i * 4]; rgb[i * 3 + 1] = rgba[i * 4 + 1]; rgb[i * 3 + 2] = rgba[i * 4 + 2];
  }
  writeFileSync(file, Buffer.concat([Buffer.from(`P6\n${w} ${h}\n255\n`), rgb]));
};
let shot = 0;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
for (;;) {
  const now = (performance.now() - start) / 1000;
  if (now > seconds) break;
  while (script.length && script[0][0] <= now) script.shift()[1]();
  if (now >= (shot + 1) * 5) { shot++; saveFrame(`${outPath}.${shot}.ppm`); }
  const t = performance.now();
  const delay = M._pocket_tick();
  busy += performance.now() - t;
  ticks++;
  const s = M._pocket_frame_serial();
  if (s !== serial) { serial = s; frames++; }
  if (delay < 0) { console.log('stopped:', lastError()); break; }
  if (delay > 0) { waited += delay; await sleep(delay); }
  else await sleep(0);
}
const wall = performance.now() - start;
saveFrame(outPath);
console.log(JSON.stringify({
  wallSeconds: +(wall / 1000).toFixed(1), ticks, frames, fps: +(frames / (wall / 1000)).toFixed(1),
  mips: +(M._pocket_million_instructions() / (busy / 1000)).toFixed(1), guestBusyPercent: +(100 * busy / wall).toFixed(1), requestedWaitPercent: +(100 * waited / wall).toFixed(1),
}));
process.exit(0);
