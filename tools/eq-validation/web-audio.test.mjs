import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';

test('restarting browser audio leaves exactly one mixer consumer and frees old buffers', () => {
  const html=readFileSync('frontends/pocket-wasm/web/index.html','utf8');
  const source=html.slice(html.indexOf('let stopAudio ='),html.indexOf('// A press shorter'));
  const contexts=[],freed=[];let allocation=0;
  class Context {
    sampleRate=44100;
    destination={};
    constructor(){contexts.push(this);}
    createScriptProcessor(){return this.node={connect(){},disconnect(){this.disconnected=true;}};}
    resume(){return Promise.resolve();}
    close(){this.closed=true;return Promise.resolve();}
  }
  const env={window:{AudioContext:Context},console,AUDIO_FRAMES:4,
    M:{_malloc:()=>++allocation*32,_free:p=>freed.push(p),_pocket_audio(){},HEAPF32:new Float32Array(64)}};
  vm.createContext(env);vm.runInContext(source,env);
  vm.runInContext('startAudio(); startAudio();',env);
  assert.equal(contexts.length,2);
  assert.equal(contexts[0].closed,true);
  assert.equal(contexts[0].node.disconnected,true);
  assert.equal(contexts[0].node.onaudioprocess,null);
  assert.equal(typeof contexts[1].node.onaudioprocess,'function');
  assert.deepEqual(freed,[32]);
  vm.runInContext('stopAudio(); stopAudio();',env);
  assert.deepEqual(freed,[32,64]);
  assert.equal(contexts[1].closed,true);
  assert.equal(readFileSync('docs/index.html','utf8'),html);
});
