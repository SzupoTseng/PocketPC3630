// Part of the PocketPC3630 page, loaded after the inline script in index.html (same global scope,
// run in order): offering the MFC DLLs from Microsoft's own SDKs on the Internet Archive.
// ---- MFC: take the DLLs from Microsoft's own SDKs on the Internet Archive ----
// Two builds of mfcce300.dll exist and neither has every export of the
// other: programs built for Pocket PC 2002 import ordinals the 2000 DLL
// lacks (Haali Reader wants 2739, 2784 and 2796), and the 2002 DLL drops
// 42 of the 2000 ones. So the import table decides, and the DLL that was
// fetched is checked against it before it goes in.
const MFC_SDKS = [
  {
    id: '2000',
    item: 'pocketpc_2000_sdk_web',
    file: 'pocketpc_2000_sdk_web.exe',
    label: 'js.sdk2000',
    cpus: ['arm', 'mips', 'sh3'],
    path: (cpu, name) => `program files/mfc/lib/${cpu}/${name}`,
    maxOrdinal: 2730,
  },
  {
    id: '2002',
    item: 'WMSDK',
    file: 'ppc2002_sdk.exe',
    label: 'js.sdk2002',
    cpus: ['arm'],
    path: (cpu, name) => `program files/pocket pc 2002/mfc/lib/arm/${name}`,
    maxOrdinal: 3566,
  },
];
function walk(dir, out = []) {
  for (const name of M.FS.readdir(dir)) {
    if (name === '.' || name === '..') continue;
    const path = `${dir}/${name}`;
    if (M.FS.isDir(M.FS.stat(path).mode)) walk(path, out);
    else out.push(path);
  }
  return out;
}
// Minimal PE reader: machine, the ordinals imported from one DLL, exports.
function pe(bytes) {
  const v = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (bytes.length < 0x40 || v.getUint16(0, true) !== 0x5a4d) return null;
  const at = v.getUint32(0x3c, true);
  if (at + 24 > bytes.length || v.getUint32(at, true) !== 0x4550) return null;
  const sections = [],
    count = v.getUint16(at + 6, true),
    opt = at + 24,
    optSize = v.getUint16(at + 20, true);
  for (let i = 0, s = opt + optSize; i < count; i++, s += 40)
    sections.push([
      v.getUint32(s + 12, true),
      Math.max(v.getUint32(s + 8, true), v.getUint32(s + 16, true)),
      v.getUint32(s + 20, true),
    ]);
  const off = (rva) => {
    for (const [va, size, raw] of sections) if (rva >= va && rva < va + size) return raw + rva - va;
    return -1;
  };
  const dir = (n) => [v.getUint32(opt + 96 + n * 8, true), v.getUint32(opt + 100 + n * 8, true)];
  const cstr = (o) => {
    let e = o;
    while (e < bytes.length && bytes[e]) e++;
    return new TextDecoder().decode(bytes.subarray(o, e));
  };
  return {
    machine: v.getUint16(at + 4, true),
    importsFrom(dll) {
      const out = new Set();
      let d = off(dir(1)[0]);
      if (d < 0) return out;
      for (; d + 20 <= bytes.length; d += 20) {
        const thunks = v.getUint32(d, true) || v.getUint32(d + 16, true),
          name = v.getUint32(d + 12, true);
        if (!name) break;
        if (cstr(off(name)).toLowerCase() !== dll) continue;
        for (let t = off(thunks); t >= 0 && t + 4 <= bytes.length; t += 4) {
          const e = v.getUint32(t, true);
          if (!e) break;
          if (e & 0x80000000) out.add(e & 0xffff);
        }
      }
      return out;
    },
    exports() {
      const out = new Set(),
        e = off(dir(0)[0]);
      if (e < 0) return out;
      const base = v.getUint32(e + 16, true),
        n = v.getUint32(e + 20, true),
        f = off(v.getUint32(e + 28, true));
      for (let i = 0; i < n; i++) if (v.getUint32(f + 4 * i, true)) out.add(base + i);
      return out;
    },
  };
}
const cpuName = (machine) =>
  [0x1c0, 0x1c2, 0x1c4].includes(machine)
    ? 'arm'
    : [0x162, 0x166, 0x169].includes(machine)
      ? 'mips'
      : [0x1a2, 0x1a3, 0x1a6].includes(machine)
        ? 'sh3'
        : null;
const sdkCache = new Map();
async function sdkFiles(sdk) {
  if (!sdkCache.has(sdk.id)) {
    const files = await unzip(await download(sdk.item, sdk.file, t(sdk.label)));
    sdkCache.set(sdk.id, new Map([...files].map(([name, read]) => [name.toLowerCase(), read])));
  }
  return sdkCache.get(sdk.id);
}
async function fixMfc(index, where = 'fix') {
  $(`${where}-go`).disabled = true;
  const show = (text) => {
    $(`${where}-text`).textContent = text;
  };
  try {
    const dir = `/library/games/${gameId(index)}`;
    const files = walk(dir).filter((p) => /\.(exe|dll)$/i.test(p) && !/\/(mfcce300|olece300)\.dll$/i.test(p));
    let cpu = null;
    const need = new Set();
    let ole = false;
    for (const path of files) {
      const info = pe(M.FS.readFile(path));
      if (!info) continue;
      if (/\.exe$/i.test(path)) cpu = cpu || cpuName(info.machine);
      for (const n of info.importsFrom('mfcce300.dll')) need.add(n);
      if (info.importsFrom('olece300.dll').size) ole = true;
    }
    cpu = cpu || 'arm';
    const newest = Math.max(0, ...need);
    const order = MFC_SDKS.filter((s) => s.cpus.includes(cpu)).sort(
      (a, b) => (newest > a.maxOrdinal) - (newest > b.maxOrdinal),
    );
    let chosen = null,
      best = null;
    for (const sdk of order) {
      show(t('js.mfcExtract', { sdk: t(sdk.label) }));
      const sdkMap = await sdkFiles(sdk);
      const read = sdkMap.get(sdk.path(cpu, 'mfcce300.dll'));
      if (!read) continue;
      const dll = await read();
      const have = pe(dll).exports();
      const missing = [...need].filter((n) => !have.has(n)).length;
      if (!best || missing < best.missing) best = { sdk, dll, missing, sdkMap };
      if (!missing) {
        chosen = best;
        break;
      }
    }
    chosen = chosen || best;
    if (!chosen) throw new Error(t('js.mfcNotFound', { cpu: cpu.toUpperCase() }));
    const targets = new Set(
      walk(dir)
        .filter((p) => /\.exe$/i.test(p))
        .map((p) => p.slice(0, p.lastIndexOf('/'))),
    );
    const ole300 = chosen.sdkMap.get(chosen.sdk.path(cpu, 'olece300.dll'));
    const oleData = ole300 ? await ole300() : null;
    for (const t of targets) {
      M.FS.writeFile(`${t}/mfcce300.dll`, chosen.dll);
      if (oleData) M.FS.writeFile(`${t}/olece300.dll`, oleData);
    }
    await syncFs(false);
    show(
      chosen.missing
        ? t('js.mfcAddedMissing', { version: chosen.sdk.id, missing: chosen.missing })
        : t('js.mfcAdded', { version: chosen.sdk.id }),
    );
    try {
      sessionStorage.setItem('pocket-autostart', gameId(index));
    } catch {}
    location.reload();
  } catch (error) {
    show(t('js.mfcFailed', { error: error.message }));
    $(`${where}-go`).disabled = false;
  }
}

