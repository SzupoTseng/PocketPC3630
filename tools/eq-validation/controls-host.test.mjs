import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import '../../frontends/pocket-wasm/web/library-ui.js';
const UI=globalThis.PocketLibraryUI;
const html=readFileSync('frontends/pocket-wasm/web/index.html','utf8');
function fixture() {
  const files=new Map(), events=[];
  let fail=false;
  const ids=['eq.e2','other'];
  const context={PocketLibraryUI:UI, running:true,runningIndex:0,releaseAllInput:()=>events.push('release'),
    gameId:i=>ids[i],withPath:(text,action)=>action(text),lastError:()=> 'invalid',listGames:()=>events.push('list'),applyControls:c=>events.push(c),
    syncFs:async()=>{if(fail)throw new Error('disk full');},
    M:{UTF8ToString:x=>x,_pocket_game_count:()=>ids.length,_pocket_set_paused:x=>events.push(['pause',x]),
      FS:{analyzePath:p=>({exists:files.has(p)}),readFile:p=>files.get(p),writeFile:(p,b)=>files.set(p,b),unlink:p=>files.delete(p)},
      _pocket_controls_json:index=>JSON.stringify({ok:true,value:index<0?UI.defaultConfig():JSON.parse(files.get(`/library/games/${ids[index]}/controls.json`)||files.get('/library/controls-default.json')||JSON.stringify(UI.defaultConfig())),overridden:index>=0&&files.has(`/library/games/${ids[index]}/controls.json`)}),
      _pocket_controls_write:(index,text)=>{const path=index<0?'/library/controls-default.json':`/library/games/${ids[index]}/controls.json`;if(text==='null')files.delete(path);else files.set(path,text);return 0;},
    }};
  vm.createContext(context);
  vm.runInContext(html.slice(html.indexOf('const pauseReasons ='),html.indexOf('const softwareUI =')),context);
  return {context,files,events,setFailure:v=>{fail=v;},run:code=>vm.runInContext(code,context)};
}
test('nested menu, focus and settings reasons resume only after the last closes',()=>{
  const f=fixture();
  f.run("setPaused('menu',true);setPaused('controls',true);setPaused('menu',false);setPaused('controls',false)");
  assert.deepEqual(f.events.filter(x=>Array.isArray(x)),[['pause',1],['pause',1],['pause',1],['pause',0]]);
});
test('shared persistence preserves dotted installed IDs, overrides and progress on failure',async()=>{
  const f=fixture();f.files.set('/library/games/eq.e2/saves/player.sav','progress');
  f.context.config=UI.fromProfile('stylus');
  await f.run("writeControls('eq.e2',config)");
  assert.equal(f.run("readControls('eq.e2').profile"),'stylus');
  assert.equal(f.run("readControls('other').profile"),'full');
  f.setFailure(true); f.context.config=UI.fromProfile('direction');
  await assert.rejects(f.run("writeControls('eq.e2',config)"),/disk full/);
  assert.equal(f.run("readControls('eq.e2').profile"),'stylus');
  assert.equal(f.files.get('/library/games/eq.e2/saves/player.sav'),'progress');
  assert.throws(()=>f.run("controlsPath('../escape')"),/Unknown/);
});
test('reset resumes inherited global controls without touching another program',async()=>{
  const f=fixture();f.context.config=UI.fromProfile('text');await f.run("writeControls('',config)");
  f.context.config=UI.fromProfile('stylus');await f.run("writeControls('eq.e2',config)");
  await f.run("writeControls('eq.e2',null)");
  assert.equal(f.run("readControls('eq.e2').profile"),'text');
  assert.equal(f.run("hasControls('eq.e2')"),false);
  assert.equal(f.run("readControls('other').profile"),'text');
});

class Element {
  constructor(tag){this.tagName=tag;this.children=[];this.listeners={};this.style={};this.dataset={};this.classList={toggle(){},add(){},remove(){}};this.isConnected=true;this.open=false;}
  append(...children){this.children.push(...children);}
  replaceChildren(...children){this.children=children;}
  get lastChild(){return this.children.at(-1);}
  setAttribute(name,value){this[name]=value;}
  addEventListener(name,callback){this.listeners[name]=callback;}
  focus(){globalThis.document.activeElement=this;}
  showModal(){this.open=true;}
  close(){this.open=false;this.listeners.close?.();}
  walk(){return [this,...this.children.filter(c=>c instanceof Element).flatMap(c=>c.walk())];}
}
test('preview and cancel never apply; saving preserves remaps and layouts; nested open stays paused',async()=>{
  const old=globalThis.document;
  const body=new Element('body');globalThis.document={body,activeElement:null,createElement:tag=>new Element(tag),createTextNode:text=>({textContent:text}),querySelectorAll:()=>[]};
  try {
    const saved=[],states=[];
    const api=UI.mount({lang:()=> 'en',t:key=>key,describe:()=>'',apply:async(id,value)=>saved.push([id,value]),reset:async()=>UI.fromProfile('text'),onOpen:()=>states.push('open'),onClose:()=>states.push('close')});
    const config={...UI.fromProfile('custom'),bindings:[{input:'PadA',vk:134}],portrait:[{vk:134,x:.2,y:.8,size:.16}],volume:.35};
    api.open({id:'local',name:'Program'},{id:'eq.e2',config});
    const dialog=body.children[0];
    const find=text=>dialog.walk().find(node=>node.textContent===text);
    assert.equal(dialog.open,true);
    find('Close').onclick();assert.equal(saved.length,0);assert.deepEqual(states,['open','close']);
    api.open({id:'local',name:'Program'},{id:'eq.e2',config});
    await find('Apply to this program').onclick();
    assert.equal(saved.length,1);assert.deepEqual(saved[0][1].bindings,config.bindings);assert.deepEqual(saved[0][1].portrait,config.portrait);assert.equal(saved[0][1].volume,.35);
    await find('Restore inherited controls').onclick();
    assert.equal(dialog.walk().find(n=>n.id==='software-profile').value,'text');
    api.open({id:'another',name:'Another'});assert.equal(dialog.open,true);assert.equal(states.at(-1),'open');
  } finally {globalThis.document=old;}
});

test('overlapping preference transactions recover before the next update mutates files',async()=>{
  const f=fixture();let syncs=0;
  f.context.syncFs=async()=>{if(++syncs===1)throw new Error('first write failed');};
  f.context.first=UI.fromProfile('stylus');f.context.second=UI.fromProfile('direction');
  const first=f.run("writeControls('eq.e2',first)");
  const second=f.run("writeControls('eq.e2',second)");
  await assert.rejects(first,/first write failed/);await second;
  assert.equal(f.run("readControls('eq.e2').profile"),'direction');
  assert.equal(syncs,2);
});

test('zoom focus and rotated stylus mapping use the same displayed crop',()=>{
  const fit=html.slice(html.indexOf('function fitCanvas()'),html.indexOf("window.addEventListener('resize'",html.indexOf('function fitCanvas()')));
  const at=html.slice(html.indexOf('  const at = (event)'),html.indexOf('  let stylus =',html.indexOf('function attachInput(')));
  for(const rotation of [0,90,270])for(const zoom of [1,2,4]) {
    const style={};const canvas={width:240,height:320,style,parentElement:{getBoundingClientRect:()=>({width:400,height:500})}};
    canvas.getBoundingClientRect=()=>{
      const w=parseFloat(style.width),h=parseFloat(style.height),bw=rotation?h:w,bh=rotation?w:h;
      const offset=value=>Number(value.match(/calc\(50% \+ ([-\d.e]+)px\)/)[1]);
      return {left:200+offset(style.left)-bw/2,top:250+offset(style.top)-bh/2,width:bw,height:bh};
    };
    const context={rotation,width:240,height:320,canvas,activeControls:{...UI.defaultConfig(),zoom,zoom_x:1,zoom_y:0},Math,
      document:{querySelector:()=>({getBoundingClientRect:()=>({height:30})})},
      $:id=>id==='screen'?canvas:id==='vkbd'?{hidden:true}:{style:{setProperty(){}}}};
    vm.createContext(context);vm.runInContext(fit+at+';fitCanvas();',context);
    const [x,y]=vm.runInContext('at({clientX:200,clientY:250})',context);
    const half=.5/zoom;
    assert.ok(Math.abs(x-(1-half)*240)<=1,`${rotation}/${zoom}: x ${x}`);
    assert.ok(Math.abs(y-half*320)<=1,`${rotation}/${zoom}: y ${y}`);
  }
});

test('unreadable control files remain protected until explicit reset',async()=>{
  const old=globalThis.document,body=new Element('body');globalThis.document={body,activeElement:null,createElement:tag=>new Element(tag),createTextNode:text=>({textContent:text}),querySelectorAll:()=>[]};
  try {
    let writes=0;
    const api=UI.mount({lang:()=> 'en',t:x=>x,describe:()=>'',apply:async()=>writes++,reset:async()=>UI.fromProfile('stylus')});
    api.open({id:'local',name:'Future config'},{id:'eq.e2',config:UI.defaultConfig(),invalid:true});
    const dialog=body.children[0],button=text=>dialog.walk().find(n=>n.textContent===text);
    assert.equal(button('Apply to this program').disabled,true);assert.equal(writes,0);
    await button('Restore inherited controls').onclick();
    assert.equal(button('Apply to this program').disabled,false);
    assert.equal(dialog.walk().find(n=>n.id==='software-profile').value,'stylus');
  }finally{globalThis.document=old;}
});
