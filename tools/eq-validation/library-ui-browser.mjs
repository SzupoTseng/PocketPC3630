// Real Chrome / WASM acceptance for the catalog and per-program Web controls.
// Owns its browser profile and loopback server; never downloads game material.
import {createServer} from 'node:http';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {resolve, join, extname, sep} from 'node:path';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
const [siteArg, chrome, profileArg, outArg, ep1, ep2, progress] = process.argv.slice(2);
if (!progress) throw Error('site chrome isolated-profile output ep1-package ep2-package ep2-progress required');
const site = resolve(siteArg), out = resolve(outArg); mkdirSync(out, {recursive: true});
const report = {scope: 'Real Chrome with real WASM; dedicated library/profile; no native frontend or physical touch acceptance', steps: [], errors: [], screenshots: [], artifacts: {}};
for (const file of ['index.html', 'library-ui.js', 'pockethle.js', 'pockethle.wasm', 'library/control-guides.json', 'library/index.json']) report.artifacts[file] = createHash('sha256').update(readFileSync(join(site, file))).digest('hex');
const server = createServer((req, res) => {
  try {
    const file = resolve(site, '.' + ((req.url ?? '/').split('?')[0] === '/' ? '/index.html' : decodeURIComponent(req.url.split('?')[0])));
    if (!file.startsWith(site + sep)) throw Error('outside site');
    const bytes=readFileSync(file);
    res.writeHead(200, {'Content-Type': ({'.html': 'text/html', '.js': 'text/javascript', '.json': 'application/json', '.wasm': 'application/wasm', '.png': 'image/png', '.webp': 'image/webp'})[extname(file)] ?? 'application/octet-stream'});
    res.end(bytes);
  } catch {if(!res.headersSent)res.writeHead(404); res.end();}
});
await new Promise(r => server.listen(8183, '127.0.0.1', r));
const browser = spawn(chrome, ['--headless=new', '--remote-debugging-port=9341', `--user-data-dir=${resolve(profileArg)}-${Date.now()}`, '--no-first-run', '--no-default-browser-check', '--disable-background-networking', '--autoplay-policy=no-user-gesture-required', 'about:blank'], {stdio: 'ignore'});
const sleep = ms => new Promise(r => setTimeout(r, ms));
let ws, sequence = 0; const pending = new Map();
function cmd(method, params = {}) {
  return new Promise((done, reject) => {
    const id = ++sequence;
    const timeout = setTimeout(() => {pending.delete(id); reject(Error('CDP timeout: ' + method));}, 45000);
    pending.set(id, value => {clearTimeout(timeout); value.error ? reject(Error(JSON.stringify(value.error))) : done(value.result);});
    ws.send(JSON.stringify({id, method, params}));
  });
}
async function evaluate(expression) {
  const r = await cmd('Runtime.evaluate', {expression, awaitPromise: true, returnByValue: true, userGesture: true});
  if (r.exceptionDetails) throw Error(JSON.stringify(r.exceptionDetails));
  return r.result?.value;
}
async function until(expression, timeout = 120000) {
  const end = Date.now() + timeout;
  while (Date.now() < end) {if (await evaluate(expression)) return; await sleep(300);}
  throw Error('Timed out: ' + expression);
}
async function check(expression, name) {
  if (!(await evaluate(expression))) throw Error(name);
  report.steps.push(name); console.log(name);
}
async function file(selector, path) {
  const {root} = await cmd('DOM.getDocument');
  const {nodeId} = await cmd('DOM.querySelector', {nodeId: root.nodeId, selector});
  await cmd('DOM.setFileInputFiles', {nodeId, files: [resolve(path)]});
}
async function shot(name) {
  const value = await cmd('Page.captureScreenshot', {format: 'png'});
  writeFileSync(join(out, name + '.png'), Buffer.from(value.data, 'base64')); report.screenshots.push(name + '.png');
}
async function load() {
  await cmd('Page.navigate', {url: 'http://127.0.0.1:8183/'});
  await until('window.pocketReady === true && LIB.items.length > 100'); await evaluate("setLang('zh-TW')");
  await check("typeof M._pocket_controls_json==='function' && typeof M._pocket_set_paused==='function'",'Current shared-control WASM exports available');
}
async function openLocal(index) {await evaluate(`[...document.querySelectorAll('#games .game')].find(row=>row.dataset.gameId===gameId(${index})).querySelector('.menu').lastElementChild.click()`); await until('document.querySelector(".software-dialog[open]")');}
async function select(profile) {await evaluate(`(()=>{const s=document.getElementById('software-profile');s.value=${JSON.stringify(profile)};s.dispatchEvent(new Event('change'));})()`);}
async function apply() {
  await evaluate("document.querySelector('.software-actions .primary').click()");
  await until("document.querySelector('.software-actions').nextElementSibling.textContent.includes('已保存')");
}
async function close() {await evaluate("document.querySelector('.software-dialog .software-header button').click()");}
try {
  let target;
  for (let n = 0; n < 80 && !target; n++) {await sleep(500); try {target = (await (await fetch('http://127.0.0.1:9341/json')).json()).find(t => t.type === 'page');} catch {}}
  if (!target) throw Error('Chrome not ready');
  ws = new WebSocket(target.webSocketDebuggerUrl); await new Promise((done, reject) => {ws.onopen = done; ws.onerror = reject;});
  ws.onmessage = event => {const data = JSON.parse(event.data); if (data.id) pending.get(data.id)?.(data), pending.delete(data.id); else if (data.method === 'Runtime.exceptionThrown') report.errors.push(data.params.exceptionDetails);};
  await cmd('Page.enable'); await cmd('Runtime.enable'); await cmd('DOM.enable');
  await cmd('Page.setInterceptFileChooserDialog', {enabled: true});
  await cmd('Emulation.setDeviceMetricsOverride', {width: 1280, height: 960, deviceScaleFactor: 1, mobile: false});
  await load();
  await evaluate("(()=>{const q=document.getElementById('lib-q');q.value='everquest';q.dispatchEvent(new Event('input'));document.getElementById('free').scrollIntoView();})()"); await sleep(500);
  await check("document.querySelectorAll('.lib-card').length >= 3", 'LIB-01 search lists the EQ episodes');
  await shot('desktop-library');
  await evaluate("document.querySelector('.lib-controls').focus();document.querySelector('.lib-controls').click()");
  await check("document.querySelector('.software-dialog').textContent.includes('保存與退出') && document.querySelector('.software-dialog').textContent.includes('Freeport')", 'LIB-02 version-specific introduction and saving guidance');
  await shot('desktop-introduction');
  await evaluate('window.__previewKeys=[];window.__originalKey=M._pocket_key;M._pocket_key=(...args)=>{__previewKeys.push(args);return __originalKey(...args)}');
  for (const profile of ['stylus', 'direction', 'full', 'text', 'desktop', 'custom', 'device']) {
    await select(profile);
    await evaluate("document.querySelector('.control-preview button')?.click()");
    await check("document.querySelector('.software-actions .primary').disabled", 'LIB-03 catalog preview cannot apply ' + profile);
  }
  await check('__previewKeys.length === 0', 'LIB-03 previews send zero guest keys');
  await evaluate("document.querySelector('.control-preview').scrollIntoView({block:'center'})");
  await shot('device-panel-preview'); await close();
  await check("document.activeElement.classList.contains('lib-controls') && document.getElementById('lib-q').value === 'everquest'", 'LIB-01 close restores focus and search');
  await evaluate("(()=>{const s=document.getElementById('lib-input');s.value='text';s.dispatchEvent(new Event('change'));})()");
  await check("document.querySelectorAll('.lib-card').length>0 && [...document.querySelectorAll('.lib-card h3')].every(n=>n.textContent.includes('III'))", 'LIB-01 input needs filter stays independent');
  await evaluate("LIB.input='all';LIB.q='';document.getElementById('lib-q').value='';buildLibraryFilters();renderLibrary(true)");
  await file('#file', ep1); await until('M._pocket_game_count() === 1 && document.querySelectorAll("#games .menu").length === 1');
  await file('#file', ep2); await until('M._pocket_game_count() === 2 && document.querySelectorAll("#games .menu").length === 2');
  await evaluate("document.querySelectorAll('#games .menu')[1].children[1].click()");
  await file('#save-file', progress);
  await until("M.FS.analyzePath('/library/games/'+gameId(1)+'/saves/pockethle-registry.dat').exists");
  const snapshot = `async()=>{const files=[];const walk=p=>{if(!M.FS.analyzePath(p).exists)return;for(const n of M.FS.readdir(p)){if(n==='.'||n==='..')continue;const f=p+'/'+n;if(M.FS.isDir(M.FS.stat(f).mode))walk(f);else files.push(f);}};for(let i=0;i<2;i++){files.push('/library/games/'+gameId(i)+'/game.json');walk('/library/games/'+gameId(i)+'/saves');}const result={};for(const f of files.sort())result[f]=Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',M.FS.readFile(f))),x=>x.toString(16).padStart(2,'0')).join('');return result;}`;
  report.before = await evaluate(`(${snapshot})()`);
  await openLocal(0); await select('stylus'); await apply(); await close();
  await openLocal(1); await select('text'); await apply(); await close();
  await check("readControls(gameId(0)).profile==='stylus' && readControls(gameId(1)).profile==='text'", 'LIB-04 independent per-program persisted profiles');
  await openLocal(1); await select('device');
  await check("!document.querySelector('.software-actions .primary').disabled && document.querySelector('.software-dialog').textContent.includes('通用 CE')", 'LIB-07 generic CE hardware keys remain distinct from unverified OEM skins');
  await select('custom');
  await evaluate("(()=>{for(const b of document.querySelectorAll('.control-custom input[value]'))b.checked=b.value==='13';document.querySelector('.control-custom').dispatchEvent(new Event('change'));})()");
  await apply(); await close();
  await check("JSON.stringify(readControls(gameId(1)).keys)==='[13]'", 'LIB-04 custom visible keys persist');
  await openLocal(1); await evaluate("document.querySelectorAll('.software-actions button')[1].click()");
  await until("document.querySelector('.software-actions').nextElementSibling.textContent.includes('已恢復')"); await close();
  await check("!hasControls(gameId(1)) && readControls(gameId(0)).profile==='stylus'", 'LIB-04 reset changes only one program');
  await openLocal(1); await select('direction'); await apply(); await close();
  await openLocal(1); await select('text');
  await evaluate("window.__sync=M.FS.syncfs;M.FS.syncfs=(_p,cb)=>cb(new Error('acceptance: write failure'));document.querySelector('.software-actions .primary').click()");
  await until("document.querySelector('.software-actions').nextElementSibling.textContent.includes('無法保存')");
  await check("readControls(gameId(1)).profile==='direction'", 'LIB-09 failed persistence restores previous controls');
  await evaluate('M.FS.syncfs=__sync'); await close();
  await evaluate("M.FS.writeFile(controlsPath(gameId(1)),JSON.stringify({version:2}));listGames()");
  await check("document.querySelectorAll('#games .game').length===2 && document.querySelector('#games').textContent.includes('操作設定無法讀取')", 'LIB-09 unreadable controls do not break other program cards');
  await openLocal(1); await select('stylus');
  await check("document.querySelector('.software-actions .primary').disabled && JSON.parse(M.FS.readFile(controlsPath(gameId(1)),{encoding:'utf8'})).version===2", 'LIB-09 future config retained until explicit reset');
  await evaluate("document.querySelectorAll('.software-actions button')[1].click()");
  await until("document.querySelector('.software-actions').nextElementSibling.textContent.includes('已恢復')");
  await select('direction'); await apply(); await close();
  report.after = await evaluate(`(${snapshot})()`);
  if (JSON.stringify(report.before) !== JSON.stringify(report.after)) throw Error('Control settings changed progress or manifests');
  report.steps.push('LIB-10 progress and game manifest hashes unchanged');
  await cmd('Page.reload'); await until('window.pocketReady === true && LIB.items.length > 100'); await evaluate("setLang('zh-TW')");
  await check("readControls(gameId(0)).profile==='stylus' && readControls(gameId(1)).profile==='direction'", 'LIB-04 profiles survive page reload');
  await evaluate('start(1)'); await until('running && document.body.classList.contains("playing")');
  await check("!document.querySelector('.pad').hidden && [...document.querySelectorAll('.pad .key:not([hidden])')].every(k=>[38,40,37,39,13].includes(Number(k.dataset.vk)))", 'LIB-04 running game uses saved direction layout');
  await evaluate('openGameMenu();window.__pausedTime=M._pocket_guest_time()');
  await sleep(250);
  await check('M._pocket_guest_time()===__pausedTime', 'CT-06 menu freezes shared guest clock');
  await evaluate('gameMenu.close()'); await sleep(100);
  await check('M._pocket_guest_time()>__pausedTime', 'CT-06 closing final pause reason resumes shared guest clock');
  await evaluate('finishWebSession()');
  // Emscripten's first export call replaces its lazy wrapper. Warm it before
  // installing the observer, otherwise that first call removes the observer.
  await evaluate('M._pocket_key(0,38);window.__events=[];window.__host=[];window.addEventListener("keydown",e=>__host.push({key:e.keyCode,target:e.target.tagName}));window.__sendKey=M._pocket_key;M._pocket_key=(...args)=>{__events.push(args);return __sendKey(...args)}');
  report.restarts = [];
  for (let i = 0; i < 10; i++) {
    await evaluate('start(1).then(()=>document.getElementById("screen").focus())');
    await cmd('Input.dispatchKeyEvent', {type: 'keyDown', key: 'ArrowUp', code: 'ArrowUp', windowsVirtualKeyCode: 38});
    await cmd('Input.dispatchKeyEvent', {type: 'keyUp', key: 'ArrowUp', code: 'ArrowUp', windowsVirtualKeyCode: 38});
    await sleep(150);
    report.restarts.push(await evaluate('({focus:document.activeElement.tagName,focused:document.hasFocus(),events:[...__events],host:[...__host]})'));
    await evaluate('finishWebSession()');
  }
  await check('__events.filter(e=>e[0]===1&&e[1]===38).length===10 && __events.filter(e=>e[0]===0&&e[1]===38).length===10', 'CT-10 ten real session restarts produce ten key pairs');
  await check('saveTimer===null && pressedAt.size===0 && pendingUp.size===0', 'CT-10 session cleanup releases keys and save timer');
  await cmd('Emulation.setDeviceMetricsOverride', {width: 390, height: 844, deviceScaleFactor: 1, mobile: true});
  await cmd('Emulation.setTouchEmulationEnabled', {enabled: true});
  await cmd('Page.reload'); await until('window.pocketReady === true && LIB.items.length > 100'); await evaluate("setLang('zh-TW')");
  await openLocal(1); await select('custom'); await evaluate("document.querySelector('.control-preview').scrollIntoView({block:'center'})"); await shot('mobile-controls');
  await check("document.querySelector('.software-dialog').getBoundingClientRect().width<=390 && document.querySelector('.software-dialog article').scrollWidth<=document.querySelector('.software-dialog article').clientWidth+1", 'LIB-08 mobile dialog has no horizontal overflow');
  await cmd('Emulation.setDeviceMetricsOverride', {width: 844, height: 390, deviceScaleFactor: 1, mobile: true}); await shot('landscape-controls');
  await cmd('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27});
  await cmd('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27});
  await check("!document.querySelector('.software-dialog').open", 'LIB-08 native Escape closes the dialog');
  await evaluate('start(1)'); await until('running'); await evaluate('openGameMenu()');
  await check("gameMenu.scrollHeight>gameMenu.clientHeight && getComputedStyle(gameMenu).overflowY==='auto'", 'CT-07 landscape game menu scrolls within viewport');
  await evaluate("gameMenu.scrollTop=gameMenu.scrollHeight");
  await check("(()=>{const b=[...gameMenu.querySelectorAll('button')].find(b=>b.textContent.includes('返回遊戲庫'));const r=b.getBoundingClientRect();const d=gameMenu.getBoundingClientRect();return r.top>=d.top&&r.bottom<=d.bottom;})()", 'CT-07 return action reachable at bottom of landscape menu');
  await shot('landscape-game-menu');
  await evaluate("window.__menuSync=M.FS.syncfs;M.FS.syncfs=(_p,cb)=>cb(new Error('acceptance: leave failure'));document.getElementById('leave').click()");
  await until("document.getElementById('game-menu-error').textContent.includes('進度寫入失敗')");
  await check("running && gameMenu.open && pauseReasons.has('menu') && !pauseReasons.has('leaving')", 'CT-07 failed leave retains paused session and visible modal error');
  await evaluate('M.FS.syncfs=__menuSync;gameMenu.close();finishWebSession()');
  await evaluate("setLang('en')"); await openLocal(1); await select('direction');
  await check("document.querySelector('.software-dialog').textContent.includes('Controls and preview') && document.querySelector('.software-dialog').textContent.includes('Save inside')", 'LIB-02 English guidance and controls');
  await close();
  await evaluate("window.__fetch=window.fetch;window.fetch=(url,...rest)=>url==='library/index.json'?Promise.reject(new Error('acceptance: catalog unavailable')):__fetch(url,...rest);loadLibrary()");
  await until("document.querySelector('#lib-cards button')?.textContent.includes('retry')");
  await check("M._pocket_game_count()===2", 'LIB-09 catalog failure keeps local programs available');
  await evaluate("window.fetch=__fetch;document.querySelector('#lib-cards button').click()");
  await until("document.querySelectorAll('.lib-card').length>0");
  report.steps.push('LIB-09 catalog retry recovers');
  report.ok = report.errors.length === 0;
} catch (error) {report.ok = false; report.error = String(error.stack ?? error); console.error(report.error);}
finally {
  writeFileSync(join(out, 'report.json'), JSON.stringify(report, null, 2) + '\n');
  if (ws?.readyState === 1) {try {await cmd('Browser.close');} catch {} ws.close();}
  browser.kill(); await new Promise(r => server.close(r));
}
console.log(JSON.stringify({ok: report.ok, output: out})); process.exit(report.ok ? 0 : 1);
