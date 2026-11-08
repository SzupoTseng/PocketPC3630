#!/usr/bin/env node
// End-to-end check of the published web page in a real headless Chrome.
//
// Serves <site> (normally docs/ after prepare-pages) on a loopback port, opens it,
// and fails on any uncaught page error. Then, in order:
//   1. the WASM module starts;
//   2. the software library shows cards and a card's story opens in three paragraphs;
//   3. a test program (our own tools/wince-window-test, never a game) is imported,
//      started and draws frames;
//   4. pausing stops the guest clock and resuming starts it again;
//   5. switching the interface theme changes the player colours;
//   6. leaving returns to the library.
//
// Usage: node tools/web-e2e/e2e.mjs <site dir> <program .zip> [chrome binary]
// The chrome binary defaults to $CHROME or google-chrome. Exit code 1 on failure.
import { createServer } from 'node:http';
import { readFileSync, existsSync, statSync, mkdtempSync, rmSync } from 'node:fs';
import { join, extname, resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { spawn } from 'node:child_process';

const [siteArg, pkgArg, chromeArg] = process.argv.slice(2);
if (!siteArg || !pkgArg) {
  console.error('usage: e2e.mjs <site dir> <program .zip> [chrome]');
  process.exit(2);
}
const site = resolve(siteArg), pkg = resolve(pkgArg);
const chrome = chromeArg || process.env.CHROME || 'google-chrome';
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const TYPES = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript', '.json': 'application/json',
  '.wasm': 'application/wasm', '.png': 'image/png', '.jpg': 'image/jpeg', '.svg': 'image/svg+xml', '.css': 'text/css' };

const server = createServer((req, res) => {
  const path = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  const file = path === '/e2e-package.zip' ? pkg : join(site, path === '/' ? 'index.html' : path);
  if (!file.startsWith(site) && file !== pkg) { res.writeHead(403); return res.end(); }
  if (!existsSync(file) || statSync(file).isDirectory()) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'content-type': TYPES[extname(file)] || 'application/octet-stream' });
  res.end(readFileSync(file));
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const url = `http://127.0.0.1:${server.address().port}/`;

const profile = mkdtempSync(join(tmpdir(), 'pocket-e2e-'));
const port = 9400 + Math.floor(Math.random() * 400);
// --do-not-de-elevate: on some Windows set-ups headless Chrome started from an elevated
// shell exits at once without it.
const browser = spawn(chrome, ['--headless=new', `--remote-debugging-port=${port}`, `--user-data-dir=${profile}`,
  '--no-first-run', '--no-default-browser-check', '--disable-gpu', '--lang=en-US',
  ...(process.platform === 'win32' ? ['--do-not-de-elevate'] : []), 'about:blank'],
  { stdio: 'ignore' });
browser.on('error', (error) => {
  console.log(`FAIL start Chrome — ${error.message} (set CHROME to the browser binary)`);
  process.exit(1);
});

let failures = 0;
const check = (ok, what, detail = '') => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}${detail ? ' — ' + detail : ''}`);
  if (!ok) failures++;
};

async function finish(code) {
  try { browser.kill(); } catch {}
  server.close();
  await sleep(300);
  try { rmSync(profile, { recursive: true, force: true }); } catch {}
  process.exit(code);
}

try {
  let target;
  for (let i = 0; i < 60 && !target; i++) {
    try { target = await (await fetch(`http://127.0.0.1:${port}/json/new?about:blank`, { method: 'PUT' })).json(); }
    catch { await sleep(250); }
  }
  if (!target) throw new Error('Chrome did not start (is the chrome binary right?)');
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((r, e) => { ws.onopen = r; ws.onerror = e; });
  let id = 0; const pending = new Map(); const pageErrors = [];
  ws.onmessage = (m) => {
    const d = JSON.parse(m.data);
    if (d.id && pending.has(d.id)) { pending.get(d.id)(d); pending.delete(d.id); }
    else if (d.method === 'Runtime.exceptionThrown') {
      const x = d.params.exceptionDetails;
      pageErrors.push(x.exception?.description || x.text);
    }
  };
  const send = (method, params = {}) => new Promise((r) => { const i = ++id; pending.set(i, r); ws.send(JSON.stringify({ id: i, method, params })); });
  const evaluate = async (expression, timeout = 30000) => {
    const r = await send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true, timeout });
    if (r.result?.exceptionDetails) throw new Error(r.result.exceptionDetails.exception?.description || 'evaluate failed');
    return r.result?.result?.value;
  };
  const until = async (expression, ms) => {
    const end = Date.now() + ms;
    while (Date.now() < end) { try { if (await evaluate(expression)) return true; } catch {} await sleep(250); }
    return false;
  };

  await send('Runtime.enable');
  await send('Page.enable');
  await send('Emulation.setDeviceMetricsOverride', { width: 1100, height: 800, deviceScaleFactor: 1, mobile: false });
  await send('Page.navigate', { url });

  check(await until("typeof M !== 'undefined' && !!M && typeof M._pocket_game_count === 'function'", 60000),
    '1. the WASM module starts');
  check(await until("document.querySelectorAll('.lib-card').length >= 20", 30000), '2. the library shows cards',
    `${await evaluate("document.querySelectorAll('.lib-card').length")} cards`);
  const story = await evaluate(`(async () => {
    const info = [...document.querySelectorAll('.lib-info')].find((d) => d.querySelector('.story') || /story/i.test(d.querySelector('summary')?.textContent || ''));
    if (!info) return 0;
    info.open = true;
    for (let i = 0; i < 40 && !info.querySelector('.story p'); i++) await new Promise((r) => setTimeout(r, 250));
    return info.querySelectorAll('.story p').length;
  })()`);
  check(story === 3, '2. a story opens in three paragraphs', `${story} paragraphs`);

  const started = await evaluate(`(async () => {
    const blob = await (await fetch('/e2e-package.zip')).blob();
    await importFile(new File([blob], 'window-test.zip'));
    start(0);
    for (let i = 0; i < 60 && !(running && M._pocket_frame_serial() > 0); i++) await new Promise((r) => setTimeout(r, 250));
    return { running, frames: M._pocket_frame_serial(), games: M._pocket_game_count(), status: document.getElementById('status')?.textContent.slice(0, 160), error: typeof lastError === 'function' ? lastError() : '' };
  })()`, 60000);
  check(started?.running === true && started?.frames > 0, '3. the test program runs and draws', JSON.stringify(started));

  const clock = await evaluate(`(async () => {
    const wait = (ms) => new Promise((r) => setTimeout(r, ms));
    setPaused('e2e', true); await wait(300);
    const a = M._pocket_guest_time(); await wait(1000); const b = M._pocket_guest_time();
    setPaused('e2e', false); await wait(800); const c = M._pocket_guest_time();
    return { pausedFor: b - a, afterResume: c - b, overlay: document.getElementById('paused').hidden };
  })()`, 20000);
  check(clock && clock.pausedFor <= 20 && clock.afterResume > 300 && clock.overlay === true,
    '4. pause stops the guest clock, resume restarts it', JSON.stringify(clock));

  const theme = await evaluate(`(() => {
    applyEmTheme('blue');
    const blue = getComputedStyle(document.documentElement).getPropertyValue('--em-flat').trim();
    applyEmTheme('red');
    const red = getComputedStyle(document.documentElement).getPropertyValue('--em-flat').trim();
    return { blue, red };
  })()`);
  check(theme?.blue === '#2f5fb8' && theme?.red === '#c8372d', '5. switching the theme changes the player colours',
    JSON.stringify(theme));

  await evaluate("setTimeout(() => document.getElementById('leave').click(), 0); true");
  check(await until("!document.body.classList.contains('playing') && typeof M !== 'undefined'", 30000),
    '6. leaving returns to the library');

  check(pageErrors.length === 0, 'no uncaught page errors', pageErrors.slice(0, 3).join(' | '));
} catch (error) {
  check(false, 'run', String(error));
}
console.log(failures ? `${failures} check(s) failed` : 'all checks passed');
await finish(failures ? 1 : 0);
