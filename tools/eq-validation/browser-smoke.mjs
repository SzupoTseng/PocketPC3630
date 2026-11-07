// Real Chromium page, UI package/save imports, pointer input and IndexedDB reload.
import {createServer} from 'node:http';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {resolve,join,extname,sep} from 'node:path';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
const [siteArg,pkgArg,saveArg,outArg,chrome,profileArg,slotArg='0',patchArg]=process.argv.slice(2);
if(!profileArg) throw Error('site package progress output chrome isolated-profile [slot] required');
const site=resolve(siteArg),out=resolve(outArg),pkg=resolve(pkgArg),save=resolve(saveArg),profile=resolve(profileArg),slot=Number(slotArg);
if(!Number.isInteger(slot)||slot<0||slot>3)throw Error('Invalid character slot');
mkdirSync(out,{recursive:true});
const sha=path=>createHash('sha256').update(readFileSync(path)).digest('hex');
const patch=patchArg?resolve(patchArg):null;
const inputs=Object.fromEntries([pkg,save,...(patch?[patch]:[]),...['index.html','pockethle.js','pockethle.wasm'].map(f=>join(site,f))].map(p=>[p,sha(p)]));
const report={inputs,scope:'Real browser page; supplied package and progress; screenshots need visual review',errors:[],consoleErrors:[],steps:[]};
const server=createServer((req,res)=>{
 try{let name=decodeURIComponent((req.url??'/').split('?')[0]);const file=resolve(site,'.'+(name==='/'?'/index.html':name));if(!file.startsWith(site+sep))throw Error('outside site');const data=readFileSync(file);res.writeHead(200,{'content-type':({'.html':'text/html','.js':'text/javascript','.wasm':'application/wasm','.png':'image/png'})[extname(file)]??'application/octet-stream'});res.end(data);}catch{res.writeHead(404);res.end();}
});
await new Promise(r=>server.listen(8174,'127.0.0.1',r));
const browser=spawn(chrome,['--headless=new','--remote-debugging-port=9336',`--user-data-dir=${profile}`,'--no-first-run','--no-default-browser-check','--disable-background-networking','--window-size=1000,950','--autoplay-policy=no-user-gesture-required','about:blank'],{stdio:'ignore'});
const sleep=ms=>new Promise(r=>setTimeout(r,ms));let ws,sequence=0;const pending=new Map();
async function command(method,params={}){return new Promise((done,reject)=>{const id=++sequence;const timer=setTimeout(()=>{pending.delete(id);reject(Error('CDP timeout: '+method));},30000);pending.set(id,x=>{clearTimeout(timer);x.error?reject(Error(JSON.stringify(x.error))):done(x.result)});ws.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const r=await command('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true,userGesture:true});if(r.exceptionDetails)throw Error(JSON.stringify(r.exceptionDetails));return r.result?.value;}
async function until(expression,ms=60000){const end=Date.now()+ms;while(Date.now()<end){if(await evaluate(expression))return;await sleep(250);}throw Error('Timed out: '+expression);}
async function file(selector,path){const {root}=await command('DOM.getDocument');const {nodeId}=await command('DOM.querySelector',{nodeId:root.nodeId,selector});await command('DOM.setFileInputFiles',{nodeId,files:[path]});}
async function shot(name){const page=await command('Page.captureScreenshot',{format:'png'});writeFileSync(join(out,name+'-page.png'),Buffer.from(page.data,'base64'));if(await evaluate("document.body.classList.contains('playing')")){const data=await evaluate("document.getElementById('screen').toDataURL('image/png')");writeFileSync(join(out,name+'-canvas.png'),Buffer.from(data.split(',')[1],'base64'));}}
async function tap(x,y){const b=await evaluate("(()=>{const b=document.getElementById('screen').getBoundingClientRect();return {x:b.x,y:b.y,w:b.width,h:b.height}})()");const at={x:b.x+(x+.5)*b.w/240,y:b.y+(y+.5)*b.h/320,button:'left',clickCount:1};await command('Input.dispatchMouseEvent',{type:'mousePressed',...at});await sleep(180);await command('Input.dispatchMouseEvent',{type:'mouseReleased',...at});await sleep(600);}
const menuReady = `(()=>{const c=document.getElementById('screen');if(c.width!==240||c.height!==320)return false;const p=c.getContext('2d').getImageData(0,0,240,320).data;return [15,85,155,225].every(y=>{const i=(y*240+11)*4;return p[i]>=120&&p[i+1]>=90&&p[i+2]<80;});})()`;
async function audioShot(name){
 if(process.env.EQ_AUDIO_PROBE!=='1')return;
 const audio=await evaluate('window.__eqAudioReport?.()');if(!audio)throw Error('Audio probe not initialized');
 const {wav,...metrics}=audio;writeFileSync(join(out,name+'-audio.wav'),Buffer.from(wav,'base64'));
 (report.audio??={})[name]=metrics;
}
async function progressSnapshot(){
 return evaluate(`(async()=>{
  const id=gameId(0),dir='/library/games/'+id,result={};
  const paths=[dir+'/game.json','/library/library.json'];
  const walk=p=>{for(const n of M.FS.readdir(p)){if(n==='.'||n==='..')continue;const f=p+'/'+n;if(M.FS.isDir(M.FS.stat(f).mode))walk(f);else paths.push(f);}};
  walk(dir+'/saves');
  for(const p of paths.sort()){
   const bytes=M.FS.readFile(p);const digest=new Uint8Array(await crypto.subtle.digest('SHA-256',bytes));result[p]=Array.from(digest,x=>x.toString(16).padStart(2,'0')).join('');
  }
  return result;
 })()`);
}
async function load(){await command('Page.navigate',{url:'http://127.0.0.1:8174/'});await until('window.pocketReady===true',120000);}
try{
 let target;for(let n=0;n<60&&!target;n++){await sleep(500);try{target=(await(await fetch('http://127.0.0.1:9336/json')).json()).find(t=>t.type==='page');}catch{}}
 if(!target)throw Error('Chromium did not expose test page');ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise((done,reject)=>{ws.onopen=done;ws.onerror=reject;});ws.onmessage=event=>{const data=JSON.parse(event.data);if(data.id)pending.get(data.id)?.(data),pending.delete(data.id);else if(data.method==='Runtime.exceptionThrown')report.errors.push(data.params.exceptionDetails);else if(data.method==='Runtime.consoleAPICalled'&&data.params.type==='error')report.consoleErrors.push(data.params.args.map(a=>a.value??a.description).join(' ').slice(0,500));};
 await command('Page.enable');await command('Runtime.enable');await command('DOM.enable');await command('Page.setInterceptFileChooserDialog',{enabled:true});if(process.env.EQ_AUDIO_PROBE==='1')await command('Page.addScriptToEvaluateOnNewDocument',{source:readFileSync(new URL('./browser-audio-probe.js',import.meta.url),'utf8')});await load();console.log('Browser ready');
 await file('#file',pkg);await until("document.querySelectorAll('#games button.play').length===1",120000);report.steps.push('UI package import');console.log('Package imported');
 await evaluate("document.querySelector('#games details.more').open=true; document.querySelectorAll('#games details.more .menu button')[1].click()");await file('#save-file',save);await until("M.FS.analyzePath('/library/games/'+gameId(0)+'/saves/pockethle-registry.dat').exists");await sleep(1000);report.steps.push('UI progress import');await shot('library');
 if(patch){
  const before=await progressSnapshot();
  await file('#file',patch);await until("document.getElementById('status').textContent.includes('Added')");
  const after=await progressSnapshot();
  if(JSON.stringify(before)!==JSON.stringify(after))throw Error('Audio patch changed progress or game manifests');
  report.audioPatch={preserved:true,before,after};report.steps.push('Audio-only patch preserves existing save files and manifests');
  await shot('patched-library');
 }
 await evaluate("document.querySelector('#games button.play').click()");await until("document.body.classList.contains('playing')");await until(menuReady,90000);await shot('character-list');await tap(120,45+slot*70);await tap(40,305);await sleep(16000);await shot('world');await audioShot('world');report.steps.push('World-load input sequence; image requires review');console.log('World screenshot saved');
 await tap(90,130);await sleep(2500);await shot('moved');await tap(229,10);await tap(185,290);await sleep(1500);await shot('saved-character');await evaluate("document.getElementById('leave').click()");await until("!document.body.classList.contains('playing')");await sleep(1500);
 await load();await until("document.querySelectorAll('#games button.play').length===1");report.steps.push('Library survives page reload');await evaluate("document.querySelector('#games button.play').click()");await until(menuReady,90000);await shot('reload-character-list');await tap(120,45+slot*70);await tap(40,305);await sleep(16000);await shot('reloaded-world');await audioShot('reloaded-world');report.steps.push('Reload-world input sequence; image requires review');
 report.hud=await evaluate("document.getElementById('hud').textContent");if(process.env.EQ_AUDIO_PROBE==='1'){
   report.missingAudio=report.consoleErrors.filter(s=>/fmod: (?:cannot load|missing audio|unsupported)/i.test(s));
   if(report.missingAudio.length)throw Error('Game requested missing audio: '+report.missingAudio.length);
   for(const phase of ['world','reloaded-world'])if(!report.audio?.[phase]?.nonzero||!report.audio[phase].capturedSamples)throw Error('No audible Web Audio callback capture: '+phase);
 }
 report.ok=report.errors.length===0;
}catch(error){report.error=String(error.stack??error);report.ok=false;}finally{writeFileSync(join(out,'browser-report.json'),JSON.stringify(report,null,2)+'\n');if(ws?.readyState===1){try{await command('Browser.close');}catch{}ws.close();}await sleep(500);browser.kill();await new Promise(r=>server.close(r));}
console.log(JSON.stringify({ok:report.ok,error:report.error,errors:report.errors.length,output:out}));process.exit(report.ok?0:1);
