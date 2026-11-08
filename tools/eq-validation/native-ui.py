"""Control only the native test process recorded in this helper's state."""
import sys,os,json,time,subprocess,ctypes
from pathlib import Path
import win32gui,win32con,win32process,win32ui
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]/'target/eq-validation'
STATE=ROOT/os.environ.get('EQ_NATIVE_STATE','native-ui-state.json')
def window():
 state=json.loads(STATE.read_text());found=[]
 def cb(h,_):
  if win32gui.IsWindowVisible(h) and win32process.GetWindowThreadProcessId(h)[1]==state['pid'] and win32gui.GetWindowText(h)=='PocketPC3630': found.append(h)
 win32gui.EnumWindows(cb,None)
 if not found: raise RuntimeError('No live test window for recorded PID')
 return found[0]
def point(h,x,y):
 _,_,w,t=win32gui.GetClientRect(h);return (int(y*t/320)<<16)|int(x*w/240)
cmd=sys.argv[1]
if cmd=='launch':

 if STATE.exists():
  try:live=window()
  except RuntimeError:live=None
  if live:raise RuntimeError('Close the recorded test process before replacing its state')
 ep=int(sys.argv[2])
 if ep not in (1,2,3):raise ValueError('Only EQ episodes 1-3 are configured')
 gid={1:'pocketeq_e1',2:'eq_e2',3:'eq_e3'}[ep];library='ep3-clean-library' if 'clean' in sys.argv[3:] else 'modified-library';game=ROOT/library/'games'/gid;meta=json.loads((game/'game.json').read_text());prefix=meta['install_dir'].rstrip('\\');reg=next(x for x in meta['registry'] if x['name']=='InstallDir');save=ROOT/'play-saves'/f'ep{ep}';save.mkdir(parents=True,exist_ok=True)
 registry='\n'.join(f'{entry["key"]}|{entry["name"]}|{entry["string"]}' for entry in meta['registry'] if entry.get('string') is not None)
 env=os.environ.copy();env.update(POCKETHLE_SEPARATE_WINDOWS='1',POCKETHLE_REGISTRY=registry,POCKETHLE_SAVE_DIR=f'{prefix}|{save}')
 if 'register' in sys.argv[3:]:
  import re
  if ep!=3:raise ValueError('Bundled registration is configured only for EP3')
  match=re.search(r'Register with:\s*([A-Za-z0-9]+)',(ROOT/'ep3-install-instructions.txt').read_text())
  if not match:raise ValueError('No registration value in supplied EP3 instructions')
  env['POCKETHLE_REGISTRY']+='\n'+reg['key']+'|RegistrationE3|'+match[1]
 if 'debug' in sys.argv[3:]:env['RUST_LOG']='pocket_winceapi::coredll=debug'
 args=[str(ROOT.parent.parent/'target/release/pockethle.exe'),'run',str(game/'extracted/PocketEQ.exe'),'--rom-dir',str(game/'extracted'),'--rom-prefix',prefix,'--module-path',prefix+'\\PocketEQ.exe','--cpu','unicorn','--message-budget','0','--max-slices','0','--display','--dump-audio-to',str(ROOT/f'ep{ep}-interactive.wav')]
 with open(ROOT/f'ep{ep}-interactive.log','w') as log:p=subprocess.Popen(args,env=env,stdout=log,stderr=log,cwd=ROOT.parent.parent,creationflags=0x00000008)
 STATE.write_text(json.dumps({'pid':p.pid,'episode':ep}));print('started',p.pid)
elif cmd=='shot':
 h=window();_,_,w,hh=win32gui.GetClientRect(h);dc=win32gui.GetWindowDC(h);src=win32ui.CreateDCFromHandle(dc);mem=src.CreateCompatibleDC();bm=win32ui.CreateBitmap();bm.CreateCompatibleBitmap(src,w,hh);mem.SelectObject(bm);ctypes.windll.user32.PrintWindow(h,mem.GetSafeHdc(),1);im=Image.frombuffer('RGB',(w,hh),bm.GetBitmapBits(True),'raw','BGRX',0,1);im.resize((240,320),Image.Resampling.NEAREST).save(sys.argv[2]);win32gui.DeleteObject(bm.GetHandle());mem.DeleteDC();src.DeleteDC();win32gui.ReleaseDC(h,dc);print('saved',w,hh)
elif cmd in ('tap','drag'):
 h=window();win32gui.ShowWindow(h,win32con.SW_RESTORE);win32gui.SetWindowPos(h,win32con.HWND_TOPMOST,0,0,0,0,win32con.SWP_NOMOVE|win32con.SWP_NOSIZE|win32con.SWP_SHOWWINDOW)
 try:win32gui.SetForegroundWindow(h)
 except Exception:pass
 def move(x,y):
  _,_,w,hh=win32gui.GetClientRect(h);sx,sy=win32gui.ClientToScreen(h,(int(x*w/240),int(y*hh/320)));
  target=win32gui.WindowFromPoint((sx,sy))
  if win32gui.GetAncestor(target,2)!=h:raise RuntimeError('Target point is covered by another window')
  ctypes.windll.user32.SetCursorPos(sx,sy)
 x,y=map(int,sys.argv[2:4]);move(x,y);time.sleep(.15);ctypes.windll.user32.mouse_event(2,0,0,0,0);time.sleep(.5)
 if cmd=='drag':
  xx,yy=map(int,sys.argv[4:6])
  for i in range(1,21):move(x+(xx-x)*i/20,y+(yy-y)*i/20);time.sleep(.05)
 ctypes.windll.user32.mouse_event(4,0,0,0,0);time.sleep(.5);print(cmd,'sent')
elif cmd=='key':
 h=window();win32gui.SetForegroundWindow(h);vk=int(sys.argv[2],16)
 if vk not in [0x1b,0x0d,0x25,0x26,0x27,0x28,0x70,0x71,0x72,0x73]:raise ValueError('Unsupported test key')
 ctypes.windll.user32.keybd_event(vk,ctypes.windll.user32.MapVirtualKeyW(vk,0),0,0);time.sleep(.2);ctypes.windll.user32.keybd_event(vk,ctypes.windll.user32.MapVirtualKeyW(vk,0),2,0);time.sleep(.5);print('key sent')
elif cmd in ('text','ep3-code'):
 h=window();win32gui.SetForegroundWindow(h)
 if cmd=='ep3-code':
  import re
  if json.loads(STATE.read_text())['episode']!=3:raise ValueError('EP3 registration command requires EP3 process')
  match=re.search(r'Register with:\s*([A-Za-z0-9]+)',(ROOT/'ep3-install-instructions.txt').read_text())
  if not match:raise ValueError('No registration value in supplied EP3 instructions')
  text=match[1]
 else:text=sys.argv[2]
 if not text.isascii() or not text.isalnum() or len(text)>16:raise ValueError('Use a short ASCII test name')
 for char in text.upper():
  vk=ord(char);scan=ctypes.windll.user32.MapVirtualKeyW(vk,0);win32gui.PostMessage(h,win32con.WM_KEYDOWN,vk,1|(scan<<16));time.sleep(.15);win32gui.PostMessage(h,win32con.WM_KEYUP,vk,0xC0000001|(scan<<16));time.sleep(.1)
 print('typed local registration value' if cmd=='ep3-code' else 'typed test name')
elif cmd=='close':win32gui.PostMessage(window(),win32con.WM_CLOSE,0,0);print('close requested')
elif cmd=='status':print('window',window(),json.loads(STATE.read_text()))
