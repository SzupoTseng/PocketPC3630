"""Build separate UI APKs and restore the default Unicorn inputs on failure.

Run on the Windows build host after build-ui-android.cmd and a separate
--no-default-features JNI build. Never run alongside another Gradle build.
"""
import argparse, hashlib, json, os, shutil, subprocess, tempfile, zipfile
from pathlib import Path

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--interpreter-jni',type=Path,default=Path('target/eq-validation/audio-interpreter-jni'))
    parser.add_argument('--gradle',default=os.environ.get('UI_GRADLE',r'G:\Android\gradle\gradle-8.7\bin\gradle.bat'))
    args=parser.parse_args()
    root=Path(__file__).resolve().parents[2]
    os.chdir(root)
    output=root/'target/ui-validation';output.mkdir(exist_ok=True)
    native=root/'frontends/pocket-android/app/src/main/jniLibs'
    apk=root/'frontends/pocket-android/app/build/outputs/apk/debug/app-debug.apk'
    abis=['arm64-v8a','x86_64']
    interpreter=args.interpreter_jni.resolve()
    for abi in abis:
        value=(interpreter/abi/'libpockethle_jni.so').read_bytes()
        if b'interpreter (ARM, MIPS, SH3)' not in value or b'Unicorn (ARM, MIPS) / interpreter (SH3)' in value:
            raise RuntimeError('Interpreter JNI backend label mismatch: '+abi)
        if b'Unicorn (ARM, MIPS) / interpreter (SH3)' not in (native/abi/'libpockethle_jni.so').read_bytes():
            raise RuntimeError('Default JNI is not verified as Unicorn: '+abi)
    backup=Path(tempfile.mkdtemp(prefix='apk-backup-',dir=output))
    for abi in abis:
        (backup/abi).mkdir();shutil.copyfile(native/abi/'libpockethle_jni.so',backup/abi/'libpockethle_jni.so')
    shutil.copyfile(apk,backup/'app-debug.apk')
    env=os.environ.copy()
    env.setdefault('JAVA_HOME',r'C:\Program Files\Amazon Corretto\jdk21.0.10_7')
    env.setdefault('ANDROID_HOME',r'G:\Android\Sdk');env.setdefault('ANDROID_SDK_ROOT',env['ANDROID_HOME'])
    records={}
    def assemble(backend):
        with (output/f'android-{backend}-package.log').open('w') as log:
            subprocess.run([args.gradle,'--offline','--no-daemon','--project-cache-dir',str(root/'target/eq-validation/gradle-project-cache'),'-p',str(root/'frontends/pocket-android'),':app:clean',':app:assembleDebug'],env=env,stdout=log,stderr=log,check=True)
        # Check managed UI too: matching JNI does not prove Kotlin rebuilt.
        with zipfile.ZipFile(apk) as archive:
            dex = b''.join(archive.read(name) for name in archive.namelist()
                           if name.startswith('classes') and name.endswith('.dex'))
        for marker in [b'Program ended. Return to the library to start again.',
                       b'Could not start program. Return to the library to retry.',
                       b'apply_run_limits', b'Lcom/pockethle/app/ControlsActivity;']:
            if marker not in dex:
                raise RuntimeError('APK has stale managed UI: ' + marker.decode())
        target=output/f'pockethle-ui-{backend}-debug.apk';shutil.copyfile(apk,target)
        records[backend]={'apk':target.name,'sha256':digest(target),'bytes':target.stat().st_size,'jni':{abi:digest(native/abi/'libpockethle_jni.so') for abi in abis},'scope':'release Rust; debug signed; build evidence only, not device acceptance'}
        (output/'android-builds.json').write_text(json.dumps(records,indent=2)+'\n')
        print(backend,'APK packaged',flush=True)
    try:
        for abi in abis:shutil.copyfile(interpreter/abi/'libpockethle_jni.so',native/abi/'libpockethle_jni.so')
        assemble('interpreter')
    finally:
        for abi in abis:shutil.copyfile(backup/abi/'libpockethle_jni.so',native/abi/'libpockethle_jni.so')
        apk.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(backup/'app-debug.apk',apk)
    try:
        assemble('unicorn')
    except Exception:
        apk.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(backup/'app-debug.apk',apk)
        raise
    print('Default JNI and APK are Unicorn; both UI APKs retained.',flush=True)

if __name__=='__main__':main()
