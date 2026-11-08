import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import vm from 'node:vm';
import '../../frontends/pocket-wasm/web/library-ui.js';
const UI = globalThis.PocketLibraryUI;
UI.setGuides(JSON.parse(readFileSync('crates/pocket-library/data/control-guides.json','utf8')));

test('shared profiles migrate legacy data and bound custom guest keys', () => {
  assert.equal(UI.profiles.length, 7);
  assert.equal(UI.profiles.find(p => p.id === 'device').available, true);
  assert.equal(UI.normalize({version: 1, profile: 'device'}).profile, 'device');
  assert.equal(UI.normalize({version: 2, profile: 'stylus'}).profile, 'full');
  assert.deepEqual(UI.fromProfile('stylus').keys, []);
  assert.deepEqual(UI.normalize({version: 1, profile: 'custom', keys: [38, 38, -1, '13', 13, 999], keyboard: true}),
    {...UI.defaultConfig(), version: 1, profile: 'custom', keys: [38, 13], keyboard: true});
  assert.equal(UI.fromProfile('text').keyboard, true);
  assert.equal(UI.info({id: 'wm-uno-free-fall'}).recommended, null);
  assert.equal(UI.info({id: 'everquest-e2'}).recommended, 'stylus');
});

test('queued persistence waits for the correct callback and recovers after errors', async () => {
  const calls = [], done = [];
  const sync = UI.createSyncQueue((populate, callback) => { calls.push(populate); done.push(callback); });
  const first = sync(false);
  let secondFinished = false;
  const second = sync(false).then(() => {secondFinished = true;});
  await new Promise(r => setImmediate(r));
  assert.equal(calls.length, 1); assert.equal(secondFinished, false);
  done[0](); await first; await new Promise(r => setImmediate(r));
  assert.equal(calls.length, 2); assert.equal(secondFinished, false);
  done[1](); await second; assert.equal(secondFinished, true);
  const failure = sync(false); const recovered = sync(true);
  await new Promise(r => setImmediate(r));
  const rejected = assert.rejects(failure, /disk full/);
  done[2](new Error('disk full')); await rejected;
  await new Promise(r => setImmediate(r));
  assert.equal(calls[3], true); done[3](); await recovered;
});

test('overlapping keyboard and touch holds emit one key pair; cleanup cancels deferred releases', () => {
  const html = readFileSync('frontends/pocket-wasm/web/index.html', 'utf8');
  const source = html.slice(html.indexOf('const pressedAt ='), html.indexOf('// Landscape games'));
  const events = [], timers = new Map(); let id = 0;
  const context = {running: true, M: {_pocket_key: (...args) => events.push(args)},
    performance: {now: () => 0}, frameMs: 33, MAX_PRESS_MS: 1000, FRAMES_PER_PRESS: 3,
    setTimeout: action => {timers.set(++id, action); return id;}, clearTimeout: n => timers.delete(n),
    document: {querySelectorAll: () => []}};
  vm.createContext(context); vm.runInContext(source, context);
  vm.runInContext("press(38); press(38, 'pad:1'); release(38);", context);
  assert.deepEqual(events, [[1, 38]]); assert.equal(timers.size, 0);
  vm.runInContext("release(38, 'pad:1'); press(38);", context);
  assert.equal(timers.size, 0); assert.deepEqual(events, [[1, 38]]);
  vm.runInContext('release(38); releaseAllInput();', context);
  assert.equal(timers.size, 0); assert.deepEqual(events, [[1, 38], [0, 38]]);
});

test('both staged and published pages have valid JavaScript', () => {
  for (const path of ['frontends/pocket-wasm/web/index.html','docs/index.html']) {
    const page=readFileSync(path,'utf8');
    for(const match of page.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/g)) new vm.Script(match[1],{filename:path});
  }
});

test('publishing rejects a stale core before changing any published file', () => {
  const paths=['docs/index.html','docs/library-ui.js','docs/pockethle.js','docs/pockethle.wasm'];
  const hashes=()=>paths.map(path=>createHash('sha256').update(readFileSync(path)).digest('hex'));
  const before=hashes();
  const result=spawnSync(process.execPath,['frontends/pocket-wasm/web/prepare-pages.mjs','/tmp/pockethle-missing-core'],{encoding:'utf8'});
  assert.notEqual(result.status,0);
  assert.deepEqual(hashes(),before);
});

test('bindings and both orientation layouts survive round trips and profile normalization', () => {
  const value={...UI.fromProfile('custom'),bindings:[{input:'PadA',vk:134},{input:'Up',vk:38}],
    portrait:[{vk:134,x:.2,y:.8,size:.16}],landscape:[{vk:134,x:.8,y:.7,size:.2}],volume:.35,opacity:.6,zoom:2,auto_hide_gamepad:false};
  assert.deepEqual(UI.normalize(JSON.parse(JSON.stringify(value))),value);
  assert.equal(UI.normalize({...value,zoom:Infinity}).zoom,1);
  assert.deepEqual(UI.normalize({...value,portrait:[{vk:13,x:-1,y:0,size:.1}]}).portrait,[]);
});

test('host key aliases agree across arrow, modifier and Escape names',()=>{
  for(const [a,b] of [['ArrowUp','Up'],['Esc','Escape'],['Control','Ctrl'],['ShiftLeft','Shift']])assert.equal(UI.canonicalName(a),UI.canonicalName(b));
});

test('shared catalog guidance uses exact IDs and keeps evidence separate from suggestions',()=>{
  const entries=JSON.parse(readFileSync('docs/library/index.json','utf8'));
  const guides=JSON.parse(readFileSync('crates/pocket-library/data/control-guides.json','utf8'));
  assert.equal(entries.length,1203);
  for(const [id,guide] of Object.entries(guides)) {
    assert.ok(entries.some(e=>e.id===id),id);
    assert.ok(UI.profiles.some(p=>p.id===guide.recommended));
    assert.ok(guide.steps.zh.length && guide.steps.en.length);
    assert.equal('verified' in guide,false);
  }
  assert.equal(UI.info({id:'everquest-e2'}).recommended,'stylus');
  assert.equal(UI.info({id:'other-everquest-e2-version'}).recommended,null);
});
