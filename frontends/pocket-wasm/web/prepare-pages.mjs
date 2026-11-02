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

const files = [
  [join(here, 'index.html'), 'index.html'],
  [join(build, 'pockethle.js'), 'pockethle.js'],
  [join(build, 'pockethle.wasm'), 'pockethle.wasm'],
];
for (const [from, name] of files) {
  if (!existsSync(from)) throw new Error(`missing ${from}: build pocket-wasm first`);
  // Debug paths in a build name the machine and account that made it. Pass
  // --remap-path-prefix to rustc instead of publishing them.
  const leak = readFileSync(from, 'latin1').match(/[A-Za-z]:\\+(?:Users|home)\\+[^\\]+|\/(?:home|Users)\/[a-z0-9_.-]+\//);
  if (leak) throw new Error(`${name} contains a local path (${leak[0]}): not publishing`);
  copyFileSync(from, join(docs, name));
  console.log(`${name}  ${statSync(from).size} bytes`);
}
// Jekyll would otherwise process the folder and drop anything it dislikes.
writeFileSync(join(docs, '.nojekyll'), '');
console.log(`ready: ${docs}`);
