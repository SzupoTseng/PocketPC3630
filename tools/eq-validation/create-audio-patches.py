"""Build user-local, audio-only EQ patches without reinstalling a game."""
import hashlib
import json
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
manifest = json.loads((ROOT / 'tools/eq-validation/audio-materials.json').read_text(encoding='utf-8'))
output = ROOT / 'target/eq-validation/audio-packages'
report = {}
for episode, game in [('2', 'eq_e2'), ('3', 'eq_e3')]:
    entries = manifest['episodes'][episode]['added']
    files = []
    for entry in entries:
        source = ROOT / entry['source']
        data = source.read_bytes()
        if hashlib.sha256(data).hexdigest() != entry['sha256']:
            raise ValueError(f'Local source changed: {source}')
        files.append((entry['destination'], data))
    path = output / f'ep{episode}-audio-patch.zip'
    with zipfile.ZipFile(path, 'w') as archive:
        body = json.dumps({'version': 1, 'game': game, 'files': [name for name, _ in files]}).encode()
        for name, data in [('pockethle-audio.json', body), *files]:
            info = zipfile.ZipInfo(name, (2000, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, data)
    report[episode] = {'game': game, 'files': len(files), 'bytes': path.stat().st_size,
                       'path': str(path.relative_to(ROOT)).replace('\\', '/'),
                       'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
    print(episode, report[episode]['files'], 'sounds;', report[episode]['bytes'], 'bytes')
(ROOT / 'tools/eq-validation/audio-patches.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
