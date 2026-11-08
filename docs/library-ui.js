// Catalog introductions and shared control preferences. No guest calls belong
// in this module: trying a preview must never press a key in a running game.
(() => {
  'use strict';
  const canonicalName = value => ({escape:'escape',esc:'escape',control:'ctrl',ctrlleft:'ctrl',ctrlright:'ctrl',shiftleft:'shift',shiftright:'shift'})[value.toLowerCase()] || value.toLowerCase().replace(/^arrow(?=up$|down$|left$|right$)/,'');
  // BEGIN generated PPC2003 panel
  const ppc2003 = Object.freeze({"id":"ppc2003","name":"Pocket PC 2003","reference":"PPC2003 CE 4.21 build 1088 / PPC_2003_SE.xml","scope":"Functional key panel; duplicate rocker directions share the D-pad; not a device skin or power emulation.","buttons":[{"id":"up","vk":38,"label":"Up","x":0.3,"y":0.43,"size":0.18,"mode":"hold"},{"id":"down","vk":40,"label":"Down","x":0.3,"y":0.85,"size":0.18,"mode":"hold"},{"id":"left","vk":37,"label":"Left","x":0.1,"y":0.64,"size":0.18,"mode":"hold"},{"id":"right","vk":39,"label":"Right","x":0.5,"y":0.64,"size":0.18,"mode":"hold"},{"id":"action","vk":13,"label":"Enter","x":0.3,"y":0.64,"size":0.18,"mode":"click"},{"id":"soft1","vk":112,"label":"F1","x":0.1,"y":0.14,"size":0.18,"mode":"click"},{"id":"soft2","vk":113,"label":"F2","x":0.36,"y":0.14,"size":0.18,"mode":"click"},{"id":"soft3","vk":119,"label":"F8","x":0.62,"y":0.14,"size":0.18,"mode":"click"},{"id":"soft4","vk":120,"label":"F9","x":0.88,"y":0.14,"size":0.18,"mode":"click"},{"id":"record","vk":121,"label":"Rec","x":0.84,"y":0.64,"size":0.18,"mode":"hold"}]});
  // END generated PPC2003 panel
  const words = (zh, en) => ({ zh, en });
  const keys = [
    [38, '↑', '上', 'Up'], [40, '↓', '下', 'Down'],
    [37, '←', '左', 'Left'], [39, '→', '右', 'Right'],
    [13, '●', '動作／Enter', 'Action / Enter'],
    [9, '1', '軟鍵 1／Tab', 'Soft 1 / Tab'], [27, '2', '軟鍵 2／Esc', 'Soft 2 / Esc'],
    [17, 'A', 'A／Ctrl', 'A / Ctrl'], [32, 'B', 'B／Space', 'B / Space'],
    [16, 'C', 'C／Shift', 'C / Shift'],
    [33, 'PgUp', '上一頁', 'Page up'], [34, 'PgDn', '下一頁', 'Page down'],
    ...ppc2003.buttons.filter(b => b.vk >= 112).map(b => [b.vk, b.label, b.vk === 121 ? '錄音／F10' : b.label, b.vk === 121 ? 'Record / F10' : b.label]),
    [134, 'Action', 'CE 中央鍵', 'CE Action'],
    ...Array.from({length:6}, (_, i) => [193+i, `App ${i+1}`, `應用鍵 ${i+1}`, `Application ${i+1}`]),
  ];
  const profiles = [
    ['stylus', 'OP-01', words('觸控筆優先', 'Stylus first'), [], false],
    ['direction', 'OP-02', words('方向＋動作鍵', 'Direction + action'), [38, 40, 37, 39, 13], false],
    ['full', 'OP-03', words('完整 Pocket PC', 'Full Pocket PC'), keys.slice(0,10).map(k => k[0]), false],
    ['text', 'OP-04', words('文字／閱讀', 'Text / reading'), [9, 27, 33, 34], true],
    ['desktop', 'OP-05', words('滑鼠＋鍵盤', 'Mouse + keyboard'), [], false],
    ['custom', 'OP-06', words('自訂', 'Custom'), keys.slice(0,10).map(k => k[0]), false],
    ['ppc2003', 'OP-08', words('Pocket PC 2003 面板', 'Pocket PC 2003 panel'), ppc2003.buttons.map(b => b.vk), false],
    ['device', 'OP-07', words('CE 裝置按鍵', 'CE device buttons'), [38,40,37,39,134,193,194,195,196,197,198], false],
  ].map(([id, code, name, visible, keyboard]) => Object.freeze({ id, code, name, visible, keyboard, available: true }));
  const defaultConfig = () => ({ version: 1, profile: 'full', keys: keys.slice(0,10).map(k => k[0]), keyboard: false, menu_key: 'F1',
    bindings: [], portrait: [], landscape: [], opacity: .7, volume: 1, zoom: 1, zoom_x: .5, zoom_y: .5,
    auto_hide_gamepad: true, relative_mouse: false, sensitivity: 1 });
  function normalize(value) {
    if (!value || value.version !== 1) return defaultConfig();
    const profile = profiles.find(p => p.id === value.profile);
    if (!profile) return defaultConfig();
    const number = (name, min, max, fallback) => Number.isFinite(value[name]) && value[name] >= min && value[name] <= max ? value[name] : fallback;
    const layout = name => Array.isArray(value[name]) ? value[name].filter(b => Number.isInteger(b.vk) && b.vk > 0 && b.vk <= 255 && Number.isFinite(b.x) && b.x >= 0 && b.x <= 1 && Number.isFinite(b.y) && b.y >= 0 && b.y <= 1 && Number.isFinite(b.size) && b.size >= .03 && b.size <= .4).slice(0,64) : [];
    return { version: 1, profile: profile.id,
      keys: Array.isArray(value.keys) ? [...new Set(value.keys)].filter(k => Number.isInteger(k) && k > 0 && k <= 255).slice(0,64) : [...profile.visible],
      keyboard: typeof value.keyboard === 'boolean' ? value.keyboard : profile.keyboard,
      menu_key: typeof value.menu_key === 'string' && value.menu_key.length > 0 && value.menu_key.length <= 64 ? value.menu_key : 'F1',
      bindings: Array.isArray(value.bindings) ? value.bindings.filter(b => typeof b.input === 'string' && b.input.length > 0 && b.input.length <= 64 && Number.isInteger(b.vk) && b.vk >= 0 && b.vk <= 255).slice(0,256) : [],
      portrait: layout('portrait'), landscape: layout('landscape'),
      opacity: number('opacity', .1, 1, .7), volume: number('volume', 0, 1, 1), zoom: number('zoom', 1, 4, 1),
      zoom_x: number('zoom_x', 0, 1, .5), zoom_y: number('zoom_y', 0, 1, .5),
      sensitivity: number('sensitivity', .1, 10, 1), auto_hide_gamepad: value.auto_hide_gamepad !== false,
      relative_mouse: value.relative_mouse === true };
  }
  const fromProfile = id => normalize({ version: 1, profile: id });
  const commonSave = words('請先使用軟體內的保存／退出功能，再備份進度 ZIP。備份只包含已寫出的進度，不是目前執行畫面的快照。',
    'Save inside the program before exporting progress. A progress ZIP contains written files, not a snapshot of the running screen.');
  let guides = {};
  function setGuides(value) { guides = value && typeof value === 'object' ? value : {}; }
  const needLabels = {
    stylus: words('觸控筆', 'Stylus'), direction: words('方向鍵', 'Direction'), text: words('文字輸入', 'Text'),
    hardware: words('硬體按鈕', 'Hardware keys'), mouse: words('滑鼠', 'Mouse'), keyboard: words('鍵盤', 'Keyboard'),
  };
  function info(entry) {
    return guides[entry.id] || { recommended: null, needs: [], steps: words(
      ['這個版本尚無專屬操作教學。請依軟體畫面與說明選擇觸控筆、按鍵或文字模式。', '可以預覽全部方案；沒有測試證據時不標為已驗證。'],
      ['There is no version-specific control guide yet. Choose stylus, buttons or text using the program’s own instructions.', 'All profiles can be previewed; no control verification is claimed without evidence.']), save: commonSave };
  }
  // Sync callers must wait for their own write, including when another sync is
  // in flight. Serializing also makes control-setting failures observable.
  function createSyncQueue(sync) {
    let tail = Promise.resolve();
    return populate => {
      const next = tail.catch(() => {}).then(() => new Promise((resolve, reject) => {
        sync(populate, error => error ? reject(error) : resolve());
      }));
      tail = next;
      return next;
    };
  }
  function mount({ lang, t, describe, apply, reset, onOpen = () => {}, onClose = () => {} }) {
    const tr = value => typeof value === 'string' ? value : value[lang().startsWith('zh') ? 'zh' : 'en'];
    const label = (zh, en) => tr(words(zh, en));
    const el = (tag, text, className) => {
      const node = document.createElement(tag);
      if (text) node.textContent = text;
      if (className) node.className = className;
      return node;
    };
    const dialog = el('dialog', '', 'software-dialog');
    dialog.setAttribute('aria-labelledby', 'software-title');
    document.body.append(dialog);
    let opener, current, localReturnId;
    const close = () => dialog.close();
    dialog.addEventListener('close', () => {
      if (dialog.open) return;
      onClose();
      current = null;
      if (opener?.isConnected) opener.focus();
      else if (localReturnId) [...document.querySelectorAll('#games .game')]
        .find(row => row.dataset.gameId === localReturnId)?.querySelector('.play')?.focus();
    });
    function open(entry, local = null) {
      const alreadyOpen = dialog.open;
      onOpen();
      opener = document.activeElement;
      current = entry;
      localReturnId = local?.id;
      const data = info(entry);
      let selected = local?.config?.profile || data.recommended || 'full';
      let draft = normalize(local?.config);
      let landscape = false;
      const article = el('article');
      const header = el('div', '', 'software-header');
      const title = el('h2', entry.name); title.id = 'software-title';
      const exit = el('button', label('關閉', 'Close')); exit.type = 'button'; exit.onclick = close;
      header.append(title, exit);
      const section = (name, contents) => {
        const block = el('section'); block.append(el('h3', name), ...contents); article.append(block); return block;
      };
      const desc = describe(entry) || label('此版本的簡介待補。', 'Description for this version is not available.');
      section(label('軟體與畫面', 'Program and screen'), [el('p', desc)]);
      if (entry.shot) {
        const picture = el('img', '', 'software-shot'); picture.src = `library/shots/${entry.shot}`;
        picture.alt = label(`${entry.name} 的既有畫面紀錄；不是本次操作驗證`, `Existing screen record for ${entry.name}; not a new control verification`);
        picture.onerror = () => picture.replaceWith(el('p', label('畫面暫時無法載入。', 'Screen image is unavailable.')));
        article.lastChild.append(picture, el('p', label('既有畫面紀錄；不代表目前平台操作已完整驗證。', 'Existing screen record; it does not verify controls on the current platform.'), 'muted'));
      } else article.lastChild.append(el('p', label('此版本尚無可用實測截圖。', 'No test screenshot is available for this version.'), 'muted'));
      const steps = el('ol'); for (const step of tr(data.steps)) steps.append(el('li', step));
      section(label('如何開始與軟體操作', 'Getting started and program controls'), [steps]);
      section(label('保存與退出', 'Saving and leaving'), [el('p', tr(data.save))]);
      const state = el('dl', '', 'software-state');
      const fact = (name, value) => state.append(el('dt', name), el('dd', value));
      fact(label('目錄啟動紀錄', 'Catalog startup record'), local
        ? label('本機版本未與目錄檔案核對', 'Local version is not matched to a catalog binary')
        : t(`lib.status.${entry.status}`));
      fact(label('操作／進度驗證', 'Control / progress verification'), label('此版本與平台尚未逐項驗證；請查看限制', 'Not individually verified for this version and platform; see limitations'));
      fact(label('操作介紹', 'Control guidance'), data.recommended
        ? label('參考建議，可自行改選；不是驗證通過', 'Suggested starting point, not a verification result')
        : label('通用說明，尚無專屬方案', 'General guidance; no specific recommendation'));
      section(label('驗證與限制', 'Verification and limitations'), [state]);
      const form = el('section'); form.append(el('h3', label('操作方式與預覽', 'Controls and preview')));
      const selectLabel = el('label', label('選擇方案 ', 'Profile '));
      const select = el('select'); select.id = 'software-profile'; selectLabel.htmlFor = select.id;
      for (const p of profiles) { const option = el('option', `${p.code} · ${tr(p.name)}`); option.value = p.id; select.append(option); }
      select.value = selected; selectLabel.append(select); form.append(selectLabel);
      const note = el('p', '', 'muted');
      const turn = el('button', label('切換直／橫向預覽', 'Rotate preview')); turn.type = 'button';
      const preview = el('div', '', 'control-preview'); preview.setAttribute('aria-label', label('操作示意，按鈕只供預覽', 'Control illustration; buttons are preview only'));
      const feedback = el('p', label('點選示意按鈕可查看用途，不會操作程式。', 'Try an illustration button to see its purpose; it does not control the program.'));
      feedback.setAttribute('role', 'status');
      const custom = el('fieldset', '', 'control-custom'); custom.append(el('legend', label('自訂顯示按鍵', 'Custom visible buttons')));
      const checks = new Map();
      for (const [vk, glyph, zh, en] of keys) {
        const row = el('label'); const box = el('input'); box.type = 'checkbox'; box.value = String(vk);
        box.checked = draft.keys.includes(vk); checks.set(vk, box);
        row.append(box, document.createTextNode(`${glyph} ${label(zh, en)}`)); custom.append(row);
      }
      const kbdLabel = el('label'); const kbd = el('input'); kbd.type = 'checkbox'; kbd.checked = draft.keyboard;
      kbdLabel.append(kbd, document.createTextNode(label('顯示文字輸入入口（支援主機 IME）', 'Show text entry (host IME supported)'))); custom.append(kbdLabel);
      const result = el('p'); result.setAttribute('role', 'status');
      if(local?.invalid)result.textContent=label('無法讀取此配置；保留原檔。請明確選擇恢復繼承／預設後再修改。','Cannot read these controls; original file retained. Explicitly restore inherited/default controls before editing.');
      const actions = el('div', '', 'software-actions');
      const save = el('button', label('套用至此軟體', 'Apply to this program'), 'primary'); save.type = 'button';
      const restore = el('button', label('恢復繼承設定', 'Restore inherited controls')); restore.type = 'button';
      const redraw = () => {
        const profile = profiles.find(p => p.id === selected);
        custom.hidden = selected !== 'custom';
        draft = selected === 'custom' ? normalize({ ...draft, version: 1, profile: 'custom', keys: [...checks].filter(([, box]) => box.checked).map(([vk]) => vk), keyboard: kbd.checked }) : normalize({...draft, ...fromProfile(selected), bindings: draft.bindings, portrait: draft.portrait, landscape: draft.landscape, opacity: draft.opacity, volume: draft.volume, zoom: draft.zoom, zoom_x: draft.zoom_x, zoom_y: draft.zoom_y, menu_key: draft.menu_key, auto_hide_gamepad: draft.auto_hide_gamepad});
        note.textContent = selected === 'ppc2003'
          ? label('PPC2003 參照：快捷鍵與 Enter 點擊一次；方向／錄音可長按。不含 Power／Reset；重複的側邊方向鍵合併於方向盤。', 'PPC2003 reference: shortcuts and Enter click once; directions / Record hold. No Power / Reset; rocker directions share the D-pad.')
          : selected === 'device'
          ? label('通用 CE Action／App1～6；不是特定機型 skin。Device Emulator 逐鍵參照尚待驗證。', 'Generic CE Action / App1–6; not an OEM skin. Device Emulator comparison is pending.')
          : selected === 'desktop' ? label('滑鼠目前是絕對位置觸控筆；相對滑鼠與游標擷取尚未提供。', 'Mouse input is an absolute stylus. Relative mouse and pointer lock are not available.')
          : label('畫面皆可直接點擊。這些方案調整 Web 螢幕按鍵與鍵盤顯示，不改動遊戲內綁定；手把支援標準 Gamepad API；文字另以確認文字送入。', 'Direct screen input stays available. Profiles change Web button/keyboard visibility, not in-game bindings. Standard Gamepad API and committed text entry are supported.');
        preview.replaceChildren(); preview.classList.toggle('landscape', landscape);
        const screen = el('div', label('程式畫面區', 'Program screen'), 'preview-screen');
        preview.append(screen);
        if (selected === 'device') {
          preview.classList.add('device-preview');
          preview.append(el('p', label('通用 CE 按鍵；機型／OS／skin 參照待取得', 'Device / OS / skin: reference pending')));
          for (const name of ['↑', '←', 'Action', '→', '↓', 'App 1', 'App 2', 'App 3', 'App 4']) {
            const button = el('button', name); button.type = 'button';
            button.onclick = () => { feedback.textContent = label(`${name}：示意位置，鍵碼待驗證。`, `${name}: illustrative position; key mapping unverified.`); };
            preview.append(button);
          }
        } else {
          preview.classList.remove('device-preview');
          for (const [vk, glyph, zh, en] of keys.filter(k => draft.keys.includes(k[0]))) {
            const button = el('button', glyph); button.type = 'button'; button.setAttribute('aria-label', label(zh, en));
            button.onclick = () => { feedback.textContent = label(`${zh}：預覽，不會送出按鍵。`, `${en}: preview; no key is sent.`); };
            preview.append(button);
          }
          if (draft.keyboard) preview.append(el('p', label('螢幕鍵盤', 'On-screen keyboard')));
        }
        save.disabled = !local || !profile.available || local.invalid === true;
      };
      select.onchange = () => { selected = select.value; redraw(); };
      custom.onchange = redraw;
      turn.onclick = () => { landscape = !landscape; redraw(); };
      save.onclick = async () => {
        const submitted = structuredClone(draft);
        save.disabled = true; restore.disabled = true; select.disabled = true; custom.disabled = true;
        try {
          await apply(local.id, submitted);
          local.config = submitted;
          draft = normalize(submitted);
          result.textContent = label('已保存此軟體的 操作配置，不影響進度或其他軟體。', 'Controls saved for this program; progress and other programs are unchanged.');
        } catch { result.textContent = label('無法保存配置，請重試；未回報為成功。', 'Could not save controls. Please retry; success was not reported.'); }
        finally { restore.disabled = false; select.disabled = false; custom.disabled = false; redraw(); }
      };
      // Filled in once the advanced editors exist; Restore uses it so they show the restored values.
      let refreshEditors = () => {};
      restore.disabled = !local;
      restore.onclick = async () => {
        save.disabled = true; restore.disabled = true; select.disabled = true; custom.disabled = true;
        try {
          draft = normalize(await reset(local.id)); local.invalid=false; selected = draft.profile; select.value = selected;
          for (const [vk, box] of checks) box.checked = draft.keys.includes(vk); kbd.checked = draft.keyboard;
          refreshEditors();
          result.textContent = label('已恢復 繼承設定，其他軟體與進度不變。', 'Inherited controls restored; other programs and progress are unchanged.');
        } catch { result.textContent = label('無法恢復設定，請重試。', 'Could not restore controls. Please retry.'); }
        finally { restore.disabled = false; select.disabled = false; custom.disabled = false; redraw(); }
      };
      const advanced = el('details'); advanced.append(el('summary', label('按鍵映射、直橫向位置與顯示', 'Bindings, orientation layouts and display')));
      const help = el('p', label('映射範例：{"input":"PadA","vk":13}；鍵盤使用 Up、A、Space；位置 x/y 為 0～1，size 為 0.03～0.4。空布局使用預設位置。', 'Binding example: {"input":"PadA","vk":13}; keyboard names: Up, A, Space. Layout x/y: 0–1, size: 0.03–0.4. Empty layouts use defaults.'));
      advanced.append(help);
      const shortcutLabel=el('label',label('選單快捷鍵','Menu shortcut')); const shortcut=el('input');shortcut.value=draft.menu_key;shortcut.oninput=()=>{draft.menu_key=shortcut.value;};shortcutLabel.append(shortcut);advanced.append(shortcutLabel);
      const editors = {}, ranges = {};
      for (const name of ['bindings', 'portrait', 'landscape']) {
        const row = el('label', name); const input = el('textarea'); input.rows = 4; input.value = JSON.stringify(draft[name], null, 2); input.setAttribute('aria-label', name);
        editors[name] = input; row.append(input); advanced.append(row);
      }
      for (const [name,min,max,step] of [['opacity',.1,1,.05],['volume',0,1,.05],['zoom',1,4,.25],['zoom_x',0,1,.05],['zoom_y',0,1,.05]]) {
        const row = el('label', name); const input = el('input'); input.type = 'range'; input.min = min; input.max = max; input.step = step; input.value = draft[name];
        input.oninput = () => { draft[name] = Number(input.value); }; row.append(input); advanced.append(row); ranges[name] = input;
      }
      const auto = el('label', label('連接手把時隱藏觸控盤', 'Hide touch controls with gamepad')); const autoBox = el('input'); autoBox.type = 'checkbox'; autoBox.checked = draft.auto_hide_gamepad;
      autoBox.onchange = () => { draft.auto_hide_gamepad = autoBox.checked; }; auto.append(autoBox); advanced.append(auto);
      // Without this, Restore changed the draft but the editors kept the old text, and
      // Apply then copied the old override back in.
      refreshEditors = () => {
        for (const name of Object.keys(editors)) editors[name].value = JSON.stringify(draft[name], null, 2);
        for (const name of Object.keys(ranges)) ranges[name].value = draft[name];
        autoBox.checked = draft.auto_hide_gamepad; shortcut.value = draft.menu_key;
      };
      form.append(advanced);
      const originalSave = save.onclick;
      save.onclick = async () => {
        try { for (const name of Object.keys(editors)) draft[name] = JSON.parse(editors[name].value); }
        catch { result.textContent = label('JSON 格式錯誤，未保存。', 'Invalid JSON; not saved.'); return; }
        await originalSave();
      };
      actions.append(save, restore);
      form.append(note, turn, preview, feedback, custom, actions, result,
        el('p', local ? label('保存此安裝的共用操作配置，不包含在進度 ZIP。設定視窗會暫停執行中的程式。', 'Shared control settings for this installation, outside progress ZIPs. Opening this dialog pauses the running program.')
          : label('尚未選定本機版本：可先預覽；匯入後請從「我的遊戲庫 → 操作方式」保存。目錄版本不會自動綁定。', 'No local version selected: preview now, then save via My library → Controls after import. Catalog versions are not matched automatically.'), 'muted'));
      article.append(form); dialog.replaceChildren(header, article); redraw(); if (!alreadyOpen) dialog.showModal(); exit.focus();
    }
    return { open, refresh: () => { if (current) dialog.close(); },
      summary: entry => {
        const data = info(entry);
        return data.needs.length ? data.needs.map(n => tr(needLabels[n])).join(' · ') : label('操作資訊待補', 'Control guide pending');
      },
      name: config => tr(profiles.find(p => p.id === normalize(config).profile).name),
      needs: entry => info(entry).needs,
    };
  }
  globalThis.PocketLibraryUI = Object.freeze({ profiles, keys, ppc2003, canonicalName, normalize, defaultConfig, fromProfile, info, setGuides, createSyncQueue, mount });
})();
