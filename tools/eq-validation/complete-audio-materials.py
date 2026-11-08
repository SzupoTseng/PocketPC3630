import json,hashlib,re,zipfile,shutil,sys
from pathlib import Path
sys.stdout.reconfigure(encoding='utf-8')
R=Path(__file__).resolve().parents[2]/'target/eq-validation'
roots={'1':R/'modified-library/games/pocketeq_e1/extracted','2':R/'modified-library/games/eq_e2/extracted','3':R/'ep3-clean-library/games/eq_e3/extracted'}
assets={e:{p.name.lower():p for p in r.rglob('*') if p.suffix.lower() in ('.wav','.mp3')} for e,r in roots.items()}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
manifest=R.parent.parent/'tools/eq-validation/audio-materials.json'
if manifest.exists():
 previous=json.loads(manifest.read_text(encoding='utf-8'))
 for episode,details in previous['episodes'].items():
  for item in details['added']:
   dest=roots[episode]/item['destination']
   if dest.exists() and sha(dest)!=item['sha256']:raise RuntimeError('Supplement was modified: '+str(dest))
   assets[episode].pop(item['name'].lower(),None)
report={'scope':'Supplement missing requested EQ sounds from user-local episode assets; originals never replaced; no synthesized substitutes','episodes':{},'common_assets':{}}
for a,b in [('1','2'),('2','3')]:
 common=assets[a].keys()&assets[b].keys();diff=[n for n in common if sha(assets[a][n])!=sha(assets[b][n])]
 report['common_assets'][a+'-'+b]={'shared':len(common),'different':diff}
for ep,folder,pkg in [('1','ep1-browser-docs','ep1-patched'),('2','ep2-browser','ep2'),('3','ep3-browser-docs','ep3')]:
 logs=json.loads((R/folder/'browser-report.json').read_text(encoding='utf-8'))['consoleErrors']
 missing=sorted({re.split(r'[\\/]+',line)[-1].rstrip('"') for line in logs if 'fmod: cannot load ' in line})
 additions=[];unresolved=[]
 for name in missing:
  if name.lower() in assets[ep]:continue
  donor=next(((e,items[name.lower()]) for e,items in assets.items() if name.lower() in items),None)
  if donor is None:unresolved.append(name);continue
  e,p=donor;dest=roots[ep]/'Sounds'/name
  dest.parent.mkdir(exist_ok=True);shutil.copyfile(p,dest)
  additions.append({'name':name,'donor_episode':e,'source':str(p.relative_to(R.parent.parent)).replace('\\','/'),'sha256':sha(p),'destination':'Sounds/'+name})
 out=R/'audio-packages'/('ep'+ep+'-audio-complete.zip');out.parent.mkdir(exist_ok=True)
 with zipfile.ZipFile(R/'packages'/(pkg+'.zip')) as src,zipfile.ZipFile(out,'w',zipfile.ZIP_DEFLATED) as z:
  for info in src.infolist():z.writestr(info,src.read(info.filename))
  for a in additions:z.write(roots[ep]/a['destination'],a['destination'])
 report['episodes'][ep]={'original_package_sha256':sha(R/'packages'/(pkg+'.zip')),'package':str(out.relative_to(R.parent.parent)).replace('\\','/'),'package_sha256':sha(out),'added':additions,'unresolved':unresolved}
 print(ep,'added',len(additions),'unresolved',unresolved)
(R.parent.parent/'tools/eq-validation/audio-materials.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
