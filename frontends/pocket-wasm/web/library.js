// Part of the PocketPC3630 page, loaded after wall.js (same global scope, run in order):
// the Pocket PC software library and, at the end, the page start-up.
// ---- Pocket PC software library: every program we know of, one page, filters instead of tabs ----
// library/index.json holds the facts (status, CPUs, platforms, images);
// library/story.zh-TW.json: a short story for each program, in Chinese,
// loaded once, the first time a reader opens one.
// Stories exist in Traditional Chinese, Simplified Chinese and English; every
// other interface language shows the English one (its label says so).
function storyLang() {
  const lang = I18N.lang || '';
  if (!/^zh/i.test(lang)) return 'en';
  return /^zh-(cn|sg|hans)/i.test(lang) ? 'zh-CN' : 'zh-TW';
}
const storiesLoad = {};
function libraryStories() {
  const lang = storyLang();
  storiesLoad[lang] ??= fetch(`library/story.${lang}.json`)
    .then((r) => (r.ok ? r.json() : {}))
    .catch(() => ({}));
  return storiesLoad[lang];
}

// library/desc.<lang>.json holds the descriptions in each language.
const LIB = {
  items: [],
  desc: {},
  shown: 0,
  filter: { status: 'all', cat: 'all', cpu: 'all' },
  input: 'all',
  q: '',
  pic: false,
  pick: false,
  sort: 'default',
};
const LIB_PAGE = 48;
const STATUS_ORDER = ['play', 'runs', 'partial', 'untested', 'nostart', 'notppc'];
const CATS = ['game', 'tool', 'office', 'media', 'net', 'dev', 'system', 'emu', 'book', 'edu', 'other'];
const CPUS = ['ARM', 'MIPS', 'SH3', 'SH4', 'x86'];

async function loadLibrary() {
  try {
    const response = await fetch('library/index.json');
    if (!response.ok) throw new Error('Catalog unavailable');
    LIB.items = await response.json();
    const controls = await fetch('library/control-guides.json');
    if (!controls.ok) throw new Error('Control guides unavailable');
    PocketLibraryUI.setGuides(await controls.json());
    if (!Array.isArray(LIB.items)) throw new Error('Invalid catalog');
  } catch (error) {
    console.error(error);
    const retry = document.createElement('button');
    retry.type = 'button';
    retry.className = 'ghost-btn';
    retry.textContent = uiText('無法載入圖書館，重試', 'Library unavailable — retry');
    retry.onclick = () => {
      retry.disabled = true;
      loadLibrary();
    };
    $('lib-cards').replaceChildren(retry);
    return;
  }
  await loadLibraryText();
  buildLibraryFilters();
  renderLibrary(true);
}
async function loadLibraryText() {
  // Descriptions exist in a few languages; the others read the English ones.
  for (const lang of [I18N.lang, 'en', 'zh-TW']) {
    try {
      const response = await fetch(`library/desc.${lang}.json`);
      if (response.ok) {
        LIB.desc = await response.json();
        return;
      }
    } catch {}
  }
}
function libMatches(entry, skip) {
  const f = LIB.filter;
  if (skip !== 'status' && f.status !== 'all' && entry.status !== f.status) return false;
  if (skip !== 'cat' && f.cat !== 'all' && entry.cat !== f.cat) return false;
  if (skip !== 'cpu' && f.cpu !== 'all' && !entry.cpus.includes(f.cpu)) return false;
  if (LIB.pic && !libHasPicture(entry)) return false;
  if (LIB.pick && !entry.pick) return false;
  if (
    LIB.input !== 'all' &&
    (LIB.input === 'unknown'
      ? softwareUI.needs(entry).length > 0
      : !softwareUI.needs(entry).includes(LIB.input))
  )
    return false;
  if (LIB.q) {
    // The collection folder and the Mobile01 thread are searchable too.
    const hay =
      `${entry.name} ${entry.kw} ${softwareUI.summary(entry)} ${LIB.desc[entry.id] || ''} ${entry.platforms.join(' ')} ${entry.devices.join(' ')} ${entry.cpus.join(' ')} ${entry.coll || ''} ${entry.m01 ? 'mobile01 ' + entry.m01 + ' ' + (entry.m01note || '') : ''}`.toLowerCase();
    const squashed = hay.replace(/\s+/g, '');
    if (!LIB.q.split(/\s+/).every((word) => hay.includes(word) || squashed.includes(word))) return false;
  }
  return true;
}
function buildLibraryFilters() {
  $('lib-input-caption').textContent = uiText('操作需求', 'Input needs');
  $('lib-input').replaceChildren();
  for (const [value, zh, en] of [
    ['all', '全部操作', 'All inputs'],
    ['stylus', '觸控筆', 'Stylus'],
    ['direction', '方向鍵', 'Direction'],
    ['text', '文字輸入', 'Text'],
    ['hardware', '硬體按鈕', 'Hardware keys'],
    ['mouse', '滑鼠', 'Mouse'],
    ['keyboard', '鍵盤', 'Keyboard'],
    ['unknown', '操作資料待補', 'Guide pending'],
  ]) {
    const option = document.createElement('option');
    option.value = value;
    option.textContent = uiText(zh, en);
    $('lib-input').append(option);
  }
  $('lib-input').value = LIB.input;
  const groups = [
    ['status', STATUS_ORDER, (v) => t(`lib.status.${v}`)],
    ['cat', CATS, (v) => t(`lib.cat.${v}`)],
    ['cpu', CPUS, (v) => v],
  ];
  for (const [group, values, label] of groups) {
    const box = $(`lib-${group}`);
    box.replaceChildren();
    for (const value of ['all', ...values]) {
      const chip = document.createElement('button');
      chip.type = 'button';
      chip.className = 'chip';
      chip.dataset.value = value;
      chip.setAttribute('aria-pressed', String(LIB.filter[group] === value));
      chip.addEventListener('click', () => {
        LIB.filter[group] = value;
        renderLibrary(true);
      });
      chip.dataset.label = value === 'all' ? t(`lib.all.${group}`) : label(value);
      box.append(chip);
    }
  }
}
function updateLibraryCounts() {
  for (const group of ['status', 'cat', 'cpu']) {
    for (const chip of $(`lib-${group}`).children) {
      const value = chip.dataset.value;
      const n = LIB.items.filter(
        (e) =>
          libMatches(e, group) &&
          (value === 'all' || (group === 'cpu' ? e.cpus.includes(value) : e[group] === value)),
      ).length;
      chip.textContent = `${chip.dataset.label} ${n}`;
      chip.setAttribute('aria-pressed', String(LIB.filter[group] === value));
      chip.hidden = n === 0 && value !== 'all' && LIB.filter[group] !== value;
    }
  }
}
function renderLibrary(reset) {
  const list = LIB.items.filter((e) => libMatches(e));
  // Sorting copies the filtered list; "default" keeps the curated order.
  const year = (e) => parseInt(e.year, 10) || 0;
  const coll = new Intl.Collator(I18N.lang, { sensitivity: 'base', numeric: true });
  if (LIB.sort === 'name') list.sort((a, b) => coll.compare(a.name, b.name));
  else if (LIB.sort === 'new') list.sort((a, b) => (year(b) || -1) - (year(a) || -1));
  else if (LIB.sort === 'old') list.sort((a, b) => (year(a) || 1e4) - (year(b) || 1e4));
  $('lib-pic').setAttribute('aria-pressed', String(LIB.pic));
  $('lib-pick').setAttribute('aria-pressed', String(LIB.pick));
  if (reset) {
    LIB.shown = 0;
    $('lib-cards').replaceChildren();
  }
  updateLibraryCounts();
  const next = list.slice(LIB.shown, LIB.shown + LIB_PAGE);
  for (const entry of next) $('lib-cards').append(libraryCard(entry));
  LIB.shown += next.length;
  $('lib-count').textContent = t('lib.total', { total: list.length });
  $('lib-more').hidden = LIB.shown >= list.length;
  $('lib-more').textContent = t('lib.moreN', { n: list.length - LIB.shown });
  $('lib-empty').hidden = list.length > 0;
}
// A cover, or a screenshot from our own test run for programs that run.
// Line drawings by category for cards with neither a picture nor the
// program's icon (24x24 viewBox, white strokes on the device screen).
const CAT_GLYPHS = {
  game: '<rect x="2" y="7" width="20" height="11" rx="5"/><path d="M7 10v5M4.5 12.5h5"/><circle cx="15.5" cy="11" r="1"/><circle cx="18" cy="13.5" r="1"/>',
  tool: '<path d="M14.5 4.5a4 4 0 0 0 5 5L12 17a2.5 2.5 0 1 1-3.5-3.5z"/><path d="M4 20l4.5-4.5"/>',
  office: '<path d="M6 3h8l4 4v14H6z"/><path d="M14 3v4h4M9 11h6M9 14h6M9 17h4"/>',
  media: '<path d="M9 18V6l10-2v12"/><circle cx="6.5" cy="18" r="2.5"/><circle cx="16.5" cy="16" r="2.5"/>',
  net: '<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3c3 3 3 15 0 18M12 3c-3 3-3 15 0 18"/>',
  dev: '<path d="M8 7l-5 5 5 5M16 7l5 5-5 5M14 4l-4 16"/>',
  system:
    '<circle cx="12" cy="12" r="3"/><path d="M12 2v3M12 19v3M2 12h3M19 12h3M4.9 4.9l2.1 2.1M17 17l2.1 2.1M4.9 19.1L7 17M17 7l2.1-2.1"/>',
  emu: '<rect x="6" y="2" width="12" height="20" rx="2"/><rect x="8.5" y="4.5" width="7" height="6"/><path d="M10 15v3M8.5 16.5h3"/><circle cx="15" cy="15.5" r=".8"/><circle cx="14" cy="18" r=".8"/>',
  book: '<path d="M4 5c3-1.5 6-1.5 8 0v15c-2-1.5-5-1.5-8 0zM20 5c-3-1.5-6-1.5-8 0v15c2-1.5 5-1.5 8 0z"/>',
  edu: '<path d="M2 9l10-5 10 5-10 5z"/><path d="M6 11v5c3 2.5 9 2.5 12 0v-5M22 9v6"/>',
  other: '<rect x="4" y="4" width="16" height="16" rx="3"/><path d="M8 12h8M12 8v8"/>',
};
function libHasPicture(entry) {
  return Boolean(entry.cover || ((entry.play || entry.pic) && (entry.shot || entry.thumb)));
}
function libraryCard(entry) {
  const card = document.createElement('article');
  card.className = `card lib-card st-${entry.status}${entry.pick ? ' is-pick' : ''}`;
  const thumb = document.createElement('div');
  thumb.className = 'thumb';
  // No picture, a picture that fails to load, or a program we cannot run:
  // show the iPAQ with the program's name on its screen instead.
  const placeholder = () => {
    const dev = document.createElement('div');
    dev.className = 'dev';
    const img = document.createElement('img');
    img.src = 'library/device.webp';
    img.alt = '';
    img.loading = 'lazy';
    img.width = 335;
    img.height = 526;
    // Each program gets its own screen colour (from its name) and initials,
    // so a run of programs without pictures does not look like one image
    // pasted over and over.
    let hash = 0;
    for (const ch of entry.name) hash = (hash * 31 + ch.codePointAt(0)) >>> 0;
    const screen = document.createElement('div');
    screen.className = 'dev-screen';
    screen.style.setProperty('--h1', String(hash % 360));
    screen.style.setProperty('--h2', String((hash >>> 9) % 360));
    const initials = document.createElement('b');
    const small = new Set(['of', 'the', 'for', 'a', 'an', 'and', 'to', 'in', 'on', 'pocket', 'pc']);
    const words = entry.name
      .replace(/[^\p{L}\p{N} ]/gu, ' ')
      .trim()
      .split(/\s+/);
    const main = words.filter((w) => !small.has(w.toLowerCase()));
    initials.textContent = (main.length ? main : words)
      .slice(0, 2)
      .map((w) => [...w][0])
      .join('')
      .toUpperCase();
    const label = document.createElement('span');
    label.textContent = entry.name;
    // The program's own icon, taken from its executable, stands in for the
    // initials when we have it.
    if (entry.icon) {
      const icon = document.createElement('img');
      icon.className = 'icon';
      icon.alt = '';
      icon.src = `library/icons/${entry.icon}`;
      icon.loading = 'lazy';
      icon.width = 64;
      icon.height = 64;
      icon.addEventListener('error', () => icon.replaceWith(initials));
      screen.append(icon, label);
    } else {
      // No icon either: a drawing of the program's category.
      const glyph = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
      glyph.setAttribute('viewBox', '0 0 24 24');
      glyph.setAttribute('class', 'glyph');
      glyph.setAttribute('aria-hidden', 'true');
      glyph.innerHTML = CAT_GLYPHS[entry.cat] || CAT_GLYPHS.other;
      screen.append(glyph, label);
    }
    dev.append(img, screen);
    thumb.classList.add('device');
    thumb.replaceChildren(dev);
  };
  if (libHasPicture(entry)) {
    const img = document.createElement('img');
    img.src = entry.cover
      ? `library/covers/${entry.cover}`
      : entry.shot
        ? `library/shots/${entry.shot}`
        : `https://archive.org/services/img/${entry.id}`;
    img.alt = t('lib.shotAlt', { name: entry.name });
    img.loading = 'lazy';
    img.width = 240;
    img.height = 320;
    if (entry.cover) img.classList.add('cover');
    else if (!entry.shot) img.classList.add('web');
    img.addEventListener('error', placeholder);
    thumb.append(img);
  } else placeholder();
  const body = document.createElement('div');
  body.className = 'body';
  const h = document.createElement('h3');
  h.textContent = entry.name;
  // Same-name entries are different downloads; a line under the name tells
  // them apart (year, platform, processor, or a retail-disc record).
  if (entry.variant) {
    const v = document.createElement('small');
    v.className = 'variant';
    v.textContent = entry.variant === '@retail' ? t('lib.variant.retail') : entry.variant;
    h.append(v);
  }
  const tags = document.createElement('div');
  tags.className = 'tags';
  const badge = document.createElement('span');
  badge.className = `tag badge st-${entry.status}`;
  badge.textContent = t(`lib.status.${entry.status}`);
  tags.append(badge);
  if (entry.pick) {
    const pk = document.createElement('span');
    pk.className = 'tag pick';
    pk.textContent = t('lib.pick');
    tags.append(pk);
  }
  for (const text of [t(`lib.cat.${entry.cat}`), ...entry.cpus]) {
    const tag = document.createElement('span');
    tag.className = 'tag';
    tag.textContent = text;
    tags.append(tag);
  }
  const desc = document.createElement('p');
  desc.className = 'desc';
  desc.textContent = LIB.desc[entry.id] || '';
  const facts = document.createElement('dl');
  facts.className = 'facts';
  const fact = (key, value) => {
    if (!value) return;
    const dt = document.createElement('dt');
    dt.textContent = t(key);
    const dd = document.createElement('dd');
    dd.textContent = value;
    facts.append(dt, dd);
  };
  const sep = /^(zh|ja)/.test(I18N.lang) ? '、' : ', ';
  fact('lib.platforms', entry.platforms.join(sep));
  fact('lib.devices', entry.devices.join(sep));
  if (entry.year) fact('lib.year', entry.year);
  if (entry.coll) fact('lib.coll', entry.coll);
  if (entry.m01) fact('lib.m01', entry.m01);
  if (entry.m01note) fact('lib.m01note', `「${entry.m01note}」`);
  fact(
    'lib.difficulty',
    `${t(`lib.diff.${entry.diff}`)}${entry.reason ? ' — ' + t(`lib.reason.${entry.reason}`) : ''}`,
  );
  const kw = document.createElement('div');
  kw.className = 'kw';
  const kwLabel = document.createElement('span');
  kwLabel.textContent = t('lib.keywords');
  const kwText = document.createElement('code');
  kwText.textContent = entry.kw;
  const copy = document.createElement('button');
  copy.type = 'button';
  copy.className = 'copy';
  copy.textContent = t('lib.copy');
  copy.addEventListener('click', async () => {
    try {
      await navigator.clipboard.writeText(entry.kw);
      copy.textContent = t('lib.copied');
    } catch {
      const r = document.createRange();
      r.selectNodeContents(kwText);
      getSelection().removeAllRanges();
      getSelection().addRange(r);
    }
    setTimeout(() => {
      copy.textContent = t('lib.copy');
    }, 1500);
  });
  kw.append(kwLabel, kwText, copy);
  // Platforms, year, difficulty and search keywords wait behind a toggle so
  // a card stays short and more programs fit on a screen.
  const more = document.createElement('details');
  more.className = 'lib-info';
  const summary = document.createElement('summary');
  const hasStory = Boolean(entry.story);
  summary.textContent = t(hasStory ? 'lib.detailsStory' : 'lib.details');
  more.append(summary, facts, kw);
  const controls = document.createElement('button');
  controls.type = 'button';
  controls.className = 'lib-controls';
  controls.textContent = `${uiText('介紹與操作方式', 'About and controls')} · ${softwareUI.summary(entry)}`;
  controls.addEventListener('click', () => softwareUI.open(entry));
  const verification = document.createElement('small');
  verification.className = 'game-controls';
  verification.textContent = uiText(
    '目錄啟動紀錄；操作／進度尚未逐項驗證',
    'Catalog startup record; controls / progress not individually verified',
  );
  // The story is fetched the first time a card with one is opened.
  if (hasStory) {
    more.addEventListener('toggle', async () => {
      card.classList.toggle('reading', more.open);
      if (!more.open || more.dataset.story) return;
      more.dataset.story = '1';
      const text = (await libraryStories())[entry.id];
      if (!text) return;
      const box = document.createElement('div');
      box.className = 'story';
      box.lang = storyLang();
      const title = document.createElement('h4');
      title.textContent = t('lib.story');
      box.append(
        title,
        ...text.split(/\n\s*\n/).map((part) => {
          const para = document.createElement('p');
          para.textContent = part;
          return para;
        }),
      );
      more.insertBefore(box, facts);
    });
  }
  body.append(h, tags, verification, desc, controls, more);
  if (entry.guide) {
    const a = document.createElement('a');
    a.className = 'guide';
    a.href = entry.guide;
    a.textContent = t('lib.guideCard');
    body.append(a);
  }
  if (entry.play) {
    const go = document.createElement('button');
    go.type = 'button';
    go.className = 'go';
    go.textContent = t('lib.play');
    go.addEventListener('click', () =>
      getFree({ item: entry.id, file: entry.play.file, build: entry.play.build, title: entry.name }, go),
    );
    body.append(go);
  }
  card.append(thumb, body);
  return card;
}

// The controls card: open beside the screen on wide desktops, behind a
// button everywhere else; the choice is remembered.
function setHelp(open) {
  $('controls-help').hidden = !open;
  $('help-toggle').setAttribute('aria-pressed', String(open));
  try {
    localStorage.setItem('controls-help', open ? '1' : '0');
  } catch {}
}
$('help-toggle').addEventListener('click', () => setHelp($('controls-help').hidden));
function initHelp() {
  let saved = null;
  try {
    saved = localStorage.getItem('controls-help');
  } catch {}
  setHelp(saved === null ? innerWidth >= 1100 && !document.body.classList.contains('touch') : saved === '1');
}

// An on-screen keyboard, the Pocket PC "SIP": phones have no keys to type
// a character name or a file name with. Each key is the virtual key the
// device's own keyboard would send; Shift capitalises the next letter.
(() => {
  const rows = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', '⌫'],
    ['⇧', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '⏎'],
    ['␣'],
  ];
  const special = { '⌫': 8, '⏎': 13, '␣': 32, '⇧': 16 };
  let shift = false;
  for (const keys of rows) {
    const row = document.createElement('div');
    row.className = 'row';
    for (const label of keys) {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = label;
      const vk = special[label] ?? label.charCodeAt(0);
      if (label === '⇧' || label === '⌫' || label === '⏎') b.className = 'wide';
      if (label === '␣') b.className = 'space';
      b.addEventListener('pointerdown', (event) => {
        event.preventDefault();
        navigator.vibrate?.(6);
        if (vk === 16) {
          shift = !shift;
          b.classList.toggle('on', shift);
          return;
        }
        const upper = shift && vk >= 65 && vk <= 90;
        if (upper) press(16, 'sip');
        press(vk, 'sip');
        release(vk, 'sip');
        if (upper) {
          release(16, 'sip');
          shift = false;
          row.parentNode.querySelector('.wide.on')?.classList.remove('on');
        }
      });
      row.append(b);
    }
    $('vkbd').append(row);
  }
  $('kbd-toggle').addEventListener('click', () => {
    const open = $('vkbd').hidden;
    $('vkbd').hidden = !open;
    $('kbd-toggle').setAttribute('aria-pressed', String(open));
    fitCanvas();
  });
})();

// Empty the whole library in one go, after one confirmation.
$('clear-all').addEventListener('click', async () => {
  if (!M || !confirm(t('js.confirmClear', { n: M._pocket_game_count() }))) return;
  for (let i = M._pocket_game_count() - 1; i >= 0; i--) {
    if (M._pocket_delete_game(i) !== 0) {
      try {
        removeTree(`/library/games/${gameId(i)}`);
      } catch {}
    }
  }
  await syncFs(false);
  location.reload();
});
$('lib-more').addEventListener('click', () => renderLibrary(false));
$('lib-pic').addEventListener('click', () => {
  LIB.pic = !LIB.pic;
  renderLibrary(true);
});
$('lib-pick').addEventListener('click', () => {
  LIB.pick = !LIB.pick;
  renderLibrary(true);
});
$('lib-sort').addEventListener('change', () => {
  LIB.sort = $('lib-sort').value;
  renderLibrary(true);
});
$('lib-input').addEventListener('change', () => {
  LIB.input = $('lib-input').value;
  renderLibrary(true);
});
// More cards arrive on their own as the list scrolls into view; the button
// stays for browsers without IntersectionObserver.
if ('IntersectionObserver' in window) {
  new IntersectionObserver(
    (entries) => {
      if (entries.some((e) => e.isIntersecting) && !$('lib-more').hidden) renderLibrary(false);
    },
    { rootMargin: '600px 0px' },
  ).observe($('lib-more'));
}
let libSearchTimer = 0;
$('lib-q').addEventListener('input', () => {
  clearTimeout(libSearchTimer);
  libSearchTimer = setTimeout(() => {
    LIB.q = $('lib-q').value.trim().toLowerCase();
    renderLibrary(true);
  }, 150);
});

let APP_VERSION = '';
setLang(pickLang()).then(loadLibrary);
// The newest release on GitHub; the download links above always point at it.
fetch('https://api.github.com/repos/SzupoTseng/PocketPC3630/releases/latest')
  .then((r) => (r.ok ? r.json() : null))
  .then((release) => {
    if (release?.tag_name) {
      APP_VERSION = release.tag_name;
      $('app-ver').textContent = t('lib.dl.version', { version: APP_VERSION });
    }
  })
  .catch(() => {});

$('pick').addEventListener('click', () => $('file').click());
$('pick-hero').addEventListener('click', () => $('file').click());

// The hero's Pocket PC shows what the emulator runs, a few seconds each.
(() => {
  const shots = [
    ['everquest', 'EverQuest · ARM'],
    ['quake', 'Pocket Quake · ARM'],
    ['slay', 'Pocket Slay · ARM'],
    ['freecell', 'FreeCell · SH3'],
    ['everquest-create', 'EverQuest · ARM'],
  ];
  if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  let i = 0;
  setInterval(() => {
    if (document.hidden || document.body.classList.contains('playing')) return;
    i = (i + 1) % shots.length;
    const img = $('shot');
    img.style.opacity = 0;
    setTimeout(() => {
      img.src = `images/shots/${shots[i][0]}.png`;
      $('shot-caption').textContent = shots[i][1];
      img.style.opacity = 1;
    }, 350);
  }, 3500);
})();
$('help-open').addEventListener('click', () => $('help').showModal());
$('help-close').addEventListener('click', () => $('help').close());

function toast(text, ms = 9000) {
  const box = $('toast');
  box.textContent = text;
  box.style.display = 'block';
  clearTimeout(toast.timer);
  toast.timer = setTimeout(() => {
    box.style.display = 'none';
  }, ms);
}
// Several files at once (a game plus the DLLs it needs) become one package,
// so nobody has to build a zip by hand.
async function importFiles(files) {
  if (!files.length || !M) return;
  try {
    if (files.length === 1) {
      await importFile(files[0]);
      return;
    }
    say(t('js.bundling', { n: files.length }));
    const main = files.find((f) => !/\.dll$/i.test(f.name)) ?? files[0];
    const entries = [];
    for (const f of files) entries.push([f.name, new Uint8Array(await f.arrayBuffer())]);
    await importFile(new File([zipStore(entries)], main.name.replace(/\.[^.]+$/, '') + '.zip'));
  } catch (error) {
    say(String(error), true);
  }
}
$('file').addEventListener('change', (event) => {
  const files = [...event.target.files];
  event.target.value = '';
  importFiles(files);
});
// Drop files anywhere on the page. Counting enters and leaves keeps the
// overlay up while the pointer crosses child elements.
let dragDepth = 0;
const hasFiles = (event) => [...(event.dataTransfer?.types || [])].includes('Files');
const overlay = (on) => {
  $('drop-overlay').classList.toggle('on', on);
  $('pick-hero').classList.toggle('over', on);
};
window.addEventListener('dragenter', (event) => {
  if (!hasFiles(event) || document.body.classList.contains('playing')) return;
  event.preventDefault();
  dragDepth++;
  overlay(true);
});
window.addEventListener('dragover', (event) => {
  if (hasFiles(event)) event.preventDefault();
});
window.addEventListener('dragleave', () => {
  if (--dragDepth <= 0) {
    dragDepth = 0;
    overlay(false);
  }
});
window.addEventListener('drop', (event) => {
  if (!hasFiles(event)) return;
  event.preventDefault();
  dragDepth = 0;
  overlay(false);
  if (document.body.classList.contains('playing')) return;
  importFiles([...event.dataTransfer.files]);
});
$('save-file').addEventListener('change', (event) => {
  const [file] = event.target.files;
  event.target.value = '';
  if (file && saveTarget !== null) importSave(saveTarget, file).catch((error) => say(String(error), true));
});
$('leave').addEventListener('click', async () => {
  setPaused('leaving', true);
  try {
    await preferenceWrites.catch(() => {});
    await syncFs(false);
    finishWebSession();
    location.reload();
  } catch {
    setPaused('leaving', false);
    say(
      uiText(
        '進度寫入失敗，請重試返回或先備份。',
        'Could not write progress. Retry leaving or export a backup first.',
      ),
      true,
    );
    toast($('status').textContent);
    const menuError = $('game-menu-error');
    if (gameMenu.open && menuError) menuError.textContent = $('status').textContent;
  }
});
// A closing tab gets no second chance, so save as soon as it is hidden.
window.addEventListener('pagehide', () => {
  if (M) syncFs(false).catch(() => {});
});
document.addEventListener('visibilitychange', () => {
  if (M && document.hidden) syncFs(false).catch(() => {});
});
if (matchMedia('(pointer: coarse)').matches) document.body.classList.add('touch');

// Two tabs would each restore the same IndexedDB state and overwrite each other's
// progress, so only one may play. The browser frees the lock when a tab goes away.
function holdTabLock() {
  if (!navigator.locks) return Promise.resolve(true);
  return new Promise((resolve) => {
    navigator.locks
      .request('pockethle-library', { ifAvailable: true }, (lock) => {
        resolve(Boolean(lock));
        return lock ? new Promise(() => {}) : undefined;
      })
      .catch(() => resolve(true));
  });
}

(async () => {
  if (!(await holdTabLock())) {
    say(t('js.otherTab'), true);
    $('pick').hidden = true;
    $('pick-hero').hidden = true;
    return;
  }
  M = await PocketHLE();
  M._pocket_set_lang(I18N.lang.startsWith('zh') ? 0 : 1);
  M.FS.mkdir('/library');
  let stored = true;
  try {
    M.FS.mount(M.FS.filesystems.IDBFS, {}, '/library');
    await syncFs(true);
    // Games deleted by earlier versions of this page lost their files but
    // kept their entry in the library index; drop those entries now.
    let pruned = false;
    for (let i = M._pocket_game_count() - 1; i >= 0; i--) {
      try {
        M.FS.stat(`/library/games/${gameId(i)}`);
      } catch {
        M._pocket_delete_game(i);
        pruned = true;
      }
    }
    if (pruned) await syncFs(false);
    navigator.storage?.persist?.().catch(() => {});
  } catch (error) {
    // Some private-browsing modes have no IndexedDB. The game still plays.
    console.warn('IndexedDB unavailable', error);
    stored = false;
  }
  listGames();
  try {
    const want = sessionStorage.getItem('pocket-autostart');
    sessionStorage.removeItem('pocket-autostart');
    if (want)
      for (let i = 0; i < M._pocket_game_count(); i++)
        if (gameId(i) === want) {
          setTimeout(() => start(i), 0);
          break;
        }
  } catch {}
  say(stored ? '' : t('js.noStorage'), !stored);
  window.pocketReady = true;
})().catch((error) => say(t('js.loadFailed', { error }), true));
