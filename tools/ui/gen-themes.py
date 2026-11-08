#!/usr/bin/env python3
"""Generate every copy of the interface themes from one file.

Source: crates/pocket-library/data/themes.json (edit only that).
Writes the generated parts of:
  crates/pocket-library/src/theme.rs                         the THEMES table (Windows)
  frontends/pocket-wasm/web/index.html (+ docs/index.html)   the --em-* CSS and EM_THEMES
  frontends/pocket-android/.../res/values/themes.xml         one style per theme
  frontends/pocket-android/.../res/values/strings.xml        the Settings list
  frontends/pocket-android/.../Themes.kt                     id -> style
Usage: python3 tools/ui/gen-themes.py [--check]
--check changes nothing and exits 1 when a file is out of date (CI runs it).
Line endings of each file are kept as they are."""
import json, os, re, sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(__file__), '..', '..'))
SRC = os.path.join(ROOT, 'crates/pocket-library/data/themes.json')
ANDROID = 'frontends/pocket-android/app/src/main'
THEMES = json.load(open(SRC, encoding='utf-8'))['themes']


def style_name(theme_id):
    if theme_id == 'red':
        return 'Theme.PocketHLE'
    return 'Theme.PocketHLE.' + ('NightRed' if theme_id == 'nightred' else theme_id.capitalize())


def hexnum(color):
    return '0x' + color.lstrip('#').lower()


def between(text, begin, end, body):
    """Replace what lies between the marker lines `begin` and `end`."""
    a = text.index(begin) + len(begin)
    b = text.index(end, a)
    return text[:a] + body + text[b:]


def rust(text):
    rows = []
    for t in THEMES:
        rows.append('    Theme {\n'
                    f'        id: "{t["id"]}",\n        zh: "{t["zh"]}",\n        en: "{t["en"]}",\n'
                    + ''.join(f'        {k}: {hexnum(t[k])},\n'
                              for k in ('flat', 'edge', 'select', 'select_ink', 'panel', 'panel2', 'ink', 'line', 'dim'))
                    + f'        dark: {str(t["dark"]).lower()},\n        square: {str(t["square"]).lower()},\n    }},\n')
    return between(text, '    // BEGIN GENERATED THEMES (tools/ui/gen-themes.py)\n',
                   '    // END GENERATED THEMES\n', ''.join(rows))


WEB_ORDER = ['radius', 'radius-lg', 'title', 'edge', 'sel', 'selink', 'flat', 'panel', 'panel2', 'ink', 'line',
             'dim', 'btn', 'btnink', 'btnline']


def web(text):
    css = []
    for t in THEMES:
        sel = ':root' if t['id'] == 'red' else f':root[data-em-theme="{t["id"]}"]'
        tokens = ''.join(f'--em-{k}:{t["web"][k]};' for k in WEB_ORDER if k in t['web'])
        css.append(f'{sel}{{{tokens}}}\n')
    text = between(text, '/* BEGIN GENERATED THEMES (tools/ui/gen-themes.py) */\n',
                   '/* END GENERATED THEMES */\n', ''.join(css))
    names = ', '.join(f"['{t['id']}', '{t['zh']}', '{t['en']}']" for t in THEMES)
    return between(text, '// BEGIN GENERATED THEME LIST (tools/ui/gen-themes.py)\n',
                   '// END GENERATED THEME LIST\n', f'const EM_THEMES = [{names}];\n')


def android_styles(text):
    out = []
    for t in THEMES:
        if t['id'] == 'red':
            continue  # the base style, written by hand below the markers' sibling, keeps red
        if t['dark']:
            out.append(f'''    <style name="{style_name(t['id'])}" parent="Theme.MaterialComponents.NoActionBar">
        <!-- Always dark, unlike the DayNight parent of the other themes: a night theme
             keeps dark surfaces even when the phone itself uses a light system theme. -->
        <item name="colorPrimary">{t['flat']}</item>
        <item name="colorPrimaryVariant">{t['edge']}</item>
        <item name="colorOnPrimary">#FFFFFF</item>
        <item name="colorSecondary">{t['select']}</item>
        <item name="colorOnSecondary">{t['select_ink']}</item>
        <item name="colorSurface">{t['panel']}</item>
        <item name="colorOnSurface">{t['ink']}</item>
        <item name="android:colorBackground">{t['panel']}</item>
        <item name="colorOnBackground">{t['ink']}</item>
        <item name="android:textColorPrimary">{t['ink']}</item>
        <item name="android:textColorSecondary">{t['dim']}</item>
        <item name="android:statusBarColor" tools:targetApi="21">{t['edge']}</item>
        <item name="android:windowBackground">?attr/colorSurface</item>
    </style>
''')
        else:
            out.append(f'''    <style name="{style_name(t['id'])}" parent="Theme.PocketHLE">
        <item name="colorPrimary">{t['flat']}</item>
        <item name="colorPrimaryVariant">{t['edge']}</item>
        <item name="colorSecondary">{t['select']}</item>
        <item name="colorOnSecondary">{t['select_ink']}</item>
        <item name="android:statusBarColor" tools:targetApi="21">{t['edge']}</item>
    </style>
''')
    text = between(text, '    <!-- BEGIN GENERATED THEMES (tools/ui/gen-themes.py) -->\n',
                   '    <!-- END GENERATED THEMES -->\n', ''.join(out))
    red = THEMES[0]
    for item, value in (('colorPrimary', red['flat']), ('colorPrimaryVariant', red['edge']),
                        ('colorSecondary', red['select'])):
        text = re.sub(rf'(<style name="Theme.PocketHLE" parent[^>]*>.*?<item name="{item}">)[^<]*(</item>)',
                      rf'\g<1>{value}\g<2>', text, count=1, flags=re.S)
    text = re.sub(r'(<style name="Theme.PocketHLE" parent[^>]*>.*?<item name="android:statusBarColor"[^>]*>)[^<]*(</item>)',
                  rf'\g<1>{red["edge"]}\g<2>', text, count=1, flags=re.S)
    return text


def android_strings(text):
    entries = ''.join(f'        <item>{t["en"]} / {t["zh"]}</item>\n' for t in THEMES)
    values = ''.join(f'        <item>{t["id"]}</item>\n' for t in THEMES)
    text = between(text, '    <string-array name="ui_theme_entries">\n', '    </string-array>', entries)
    return between(text, '    <string-array name="ui_theme_values">\n', '    </string-array>', values)


def kotlin(text):
    rows = ''.join(f'        "{t["id"]}" to R.style.{style_name(t["id"]).replace(".", "_")},\n' for t in THEMES)
    return between(text, '    // BEGIN GENERATED THEMES (tools/ui/gen-themes.py)\n',
                   '    // END GENERATED THEMES\n', rows)


TARGETS = [
    ('crates/pocket-library/src/theme.rs', rust),
    ('frontends/pocket-wasm/web/index.html', web),
    ('docs/index.html', web),
    (f'{ANDROID}/res/values/themes.xml', android_styles),
    (f'{ANDROID}/res/values/strings.xml', android_strings),
    (f'{ANDROID}/java/com/pockethle/app/Themes.kt', kotlin),
]


def main():
    check = '--check' in sys.argv
    stale = []
    for rel, fn in TARGETS:
        path = os.path.join(ROOT, rel)
        raw = open(path, encoding='utf-8', newline='').read()
        nl = '\r\n' if '\r\n' in raw else '\n'
        old = raw.replace('\r\n', '\n')
        new = fn(old)
        if new != old:
            stale.append(rel)
            if not check:
                open(path, 'w', encoding='utf-8', newline='').write(new.replace('\n', nl))
    if check and stale:
        print('out of date (run python3 tools/ui/gen-themes.py):', *stale, sep='\n  ')
        sys.exit(1)
    print('updated:' if stale else 'up to date', *stale)


if __name__ == '__main__':
    main()
