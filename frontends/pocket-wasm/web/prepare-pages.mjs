#!/usr/bin/env node
// Copies the browser build into docs/ for GitHub Pages (Settings -> Pages ->
// Deploy from a branch -> main, /docs). Only the files named here are published;
// game packages stay out of the repository because each player uploads their own.
// Usage: node frontends/pocket-wasm/web/prepare-pages.mjs [build dir]
import { copyFileSync, existsSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, '../../..');
const build = resolve(process.argv[2] ?? join(repo, 'target/wasm32-unknown-emscripten/release'));
const docs = join(repo, 'docs');

// A syntax error in the page would publish a site that never starts.
const page = readFileSync(join(here, 'index.html'), 'utf8');
new Function(page.match(/<script>([\s\S]*?)<\/script>/)[1]);
// The page script is split into index.html (core and player) and these files, loaded in
// this order into the same global scope; each must parse, and each must be referenced.
const PAGE_SCRIPTS = ['mfc.js', 'wall.js', 'library.js'];
for (const name of PAGE_SCRIPTS) {
  new Function(readFileSync(join(here, name), 'utf8'));
  if (!page.includes(`<script src="${name}"></script>`)) throw new Error(`index.html does not load ${name}`);
}

const files = [
  [join(here, 'index.html'), 'index.html'],
  [join(here, 'library-ui.js'), 'library-ui.js'],
  ...PAGE_SCRIPTS.map((name) => [join(here, name), name]),
  [join(repo, 'crates/pocket-library/data/control-guides.json'), 'library/control-guides.json'],
  [join(build, 'pockethle.js'), 'pockethle.js'],
  [join(build, 'pockethle.wasm'), 'pockethle.wasm'],
];
// Preflight every input before replacing any published file. A stale WASM
// build must not receive a page that calls exports it cannot provide.
const glue = readFileSync(join(build, 'pockethle.js'), 'utf8');
for (const name of ['pocket_controls_json', 'pocket_controls_write', 'pocket_set_paused', 'pocket_set_volume', 'pocket_guest_time', 'pocket_text']) {
  if (!glue.includes(name)) throw new Error(`stale browser build: missing ${name}; rebuild pocket-wasm first`);
}
for (const [from, name] of files) {
  if (!existsSync(from)) throw new Error(`missing ${from}: build pocket-wasm first`);
  // Debug paths in a build name the machine and account that made it. Pass
  // --remap-path-prefix to rustc instead of publishing them.
  const leak = readFileSync(from, 'latin1').match(/[A-Za-z]:\\+(?:Users|home)\\+[^\\]+|\/(?:home|Users)\/[a-z0-9_.-]+\//);
  if (leak) throw new Error(`${name} contains a local path (${leak[0]}): not publishing`);
}
for (const [from, name] of files) {
  copyFileSync(from, join(docs, name));
  console.log(`${name}  ${statSync(from).size} bytes`);
}
// Jekyll would otherwise process the folder and drop anything it dislikes.
writeFileSync(join(docs, '.nojekyll'), '');
console.log(`ready: ${docs}`);
