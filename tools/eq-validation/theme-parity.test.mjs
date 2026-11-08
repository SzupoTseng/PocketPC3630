// The interface colors live in three places: crates/pocket-library/src/theme.rs (the
// Windows window), the web page's CSS and Android's themes.xml. They must name the
// same themes with the same main color, or a theme would look different per platform.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

const rust = readFileSync('crates/pocket-library/src/theme.rs', 'utf8');
const themes = [...rust.matchAll(/id: "(\w+)",[\s\S]*?flat: 0x(\w+),\s*edge: 0x(\w+),/g)]
  .map(([, id, flat, edge]) => ({id, flat: '#' + flat, edge: '#' + edge}));

test('theme.rs lists red first and eight themes', () => {
  assert.equal(themes.length, 8);
  assert.equal(themes[0].id, 'red');
});

test('the web page has every theme with the same main and edge colors', () => {
  const html = readFileSync('frontends/pocket-wasm/web/index.html', 'utf8');
  for (const {id, flat, edge} of themes) {
    const selector = id === 'red' ? ':root{' : `:root[data-em-theme="${id}"]{`;
    const start = html.indexOf(selector);
    assert.ok(start >= 0, `web theme ${id}`);
    const block = html.slice(start, html.indexOf('}', start));
    assert.ok(block.includes(`--em-flat:${flat}`), `${id} flat ${flat}`);
    assert.ok(block.includes(`--em-edge:${edge}`), `${id} edge ${edge}`);
  }
});

test('Android has a style for every theme with the same main color', () => {
  const xml = readFileSync('frontends/pocket-android/app/src/main/res/values/themes.xml', 'utf8');
  const name = (id) => (id === 'red' ? 'Theme.PocketHLE' : 'Theme.PocketHLE.' + (id === 'nightred' ? 'NightRed' : id[0].toUpperCase() + id.slice(1)));
  for (const {id, flat, edge} of themes) {
    const start = xml.indexOf(`<style name="${name(id)}"`);
    assert.ok(start >= 0, `android style ${id}`);
    const block = xml.slice(start, xml.indexOf('</style>', start));
    assert.ok(block.toLowerCase().includes(`>${flat}<`), `${id} colorPrimary ${flat}`);
    assert.ok(block.toLowerCase().includes(`>${edge}<`), `${id} variant ${edge}`);
  }
  const kotlin = readFileSync('frontends/pocket-android/app/src/main/java/com/pockethle/app/Themes.kt', 'utf8');
  for (const {id} of themes) assert.ok(kotlin.includes(`"${id}" to`), `Themes.kt ${id}`);
});
