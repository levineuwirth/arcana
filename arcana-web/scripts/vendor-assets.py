#!/usr/bin/env python3
"""vendor-assets.py — the web client's fonts and mana symbols, from their upstream releases.

    uv run --no-project --with fonttools --with brotli arcana-web/scripts/vendor-assets.py

Writes arcana-web/static/assets/, which the server embeds and serves under
/assets/ (arcana-web/src/assets.rs lists every file):

  fonts/Spectral-*.woff2, fonts/FiraSans-*.woff2, fonts/JetBrainsMono-Regular.woff2
      WOFF2 subsets of Spectral and JetBrains Mono from the Google Fonts
      repository and of Fira Sans 4.301 from bBox Type's, each family's OFL
      beside it as fonts/OFL-<family>.txt.
  fonts/mana.woff2, fonts/OFL-Mana.txt, mana.css
      Andrew Gioia's Mana 1.17.1, the font unmodified (SIL OFL 1.1) and its
      stylesheet (MIT) with one change: its two @font-face rules, which name
      EOT, WOFF, TTF and SVG files and the MPlantin face, become one rule for
      the WOFF2 served here. MPlantin is not vendored.

and rewrites the fallback faces between the markers in static/theme.css.

This is the recipe of the author's site (levineuwirth.org's
tools/subset-fonts.py at 6f75f07), whose type the client shares, with
Arcana's faces and text:

  - Sources are pinned by revision and SHA-256 in asset-sources.json and
    cached under ~/.cache/arcana-assets. Spectral Medium and Fira Sans Medium
    are pinned here first; the rest carry the site's digests.
  - The subsets keep the site's ranges (Latin-1, Latin Extended-A, the
    punctuation block) plus every character in the catalog's string literals,
    which is what the client prints: card names and ability text. A font that
    lacks one of them is reported; the browser draws that character from a
    system font.
  - Every name record is kept (the licence text and URL are IDs 13 and 14).
  - Fira Sans 4.301 reserves the name "Fira" in its name table, and a subset
    is a Modified Version, which may not use it as its primary name; the
    subsets' name records say "Arcana Sans". Stylesheets still call the
    family "Fira Sans".
  - Fallback faces: for each system font a player may see before or instead
    of Spectral and Fira Sans, an @font-face that names it with local() and
    scales it so a line of the catalog's text sets to the web font's width,
    with the web font's ascent and descent, so text barely moves when the web
    font arrives. Reference metrics come from metric-compatible open fonts
    (Gelasio for Georgia, Tinos for Times New Roman, Arimo for Arial, Noto
    Serif and Roboto themselves).
"""

from __future__ import annotations

import collections
import hashlib
import io
import json
import re
import sys
import urllib.request
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer

WEB = Path(__file__).resolve().parent.parent
REPO = WEB.parent
ASSETS = WEB / "static" / "assets"
FONTS = ASSETS / "fonts"
THEME_CSS = WEB / "static" / "theme.css"
CATALOG = REPO / "arcana-cards" / "src"
CACHE = Path.home() / ".cache" / "arcana-assets"
SOURCE_HASHES = json.loads((WEB / "scripts" / "asset-sources.json").read_text())

GOOGLE = "https://raw.githubusercontent.com/google/fonts/9710da1eacb3be272583c3224dcb70f9da6eadbb/ofl"
BBOX = "https://raw.githubusercontent.com/bBoxType/FiraSans/f54eeb3124c63fe9b5bcd36d09d1cd46788cd15e"
FIRA_TTF = BBOX + "/Fira_Sans_4_3/Fonts/Fira_Sans_TTF_4301/Normal/Roman"
MANA = "https://raw.githubusercontent.com/andrewgioia/mana/957ad2abcde30c9ffa38e2d37c8512c6be967199"

# The site's ranges: Latin-1, Latin Extended-A, the punctuation block, and a
# few symbols.
BASE = ("U+0000-00FF,U+0100-017F,U+02BB-02BC,U+02C6,U+02DA,U+02DC,U+2000-206F,"
        "U+2074,U+20AC,U+2122,U+2190-2193,U+2212,U+2215,U+FEFF,U+FFFD")
SPECTRAL_EXTRA = "U+2248,U+2264-2265,U+25A1,U+25CB,U+25CF"
JBM_EXTRA = "U+2500-257F"

SPECTRAL_FEATURES = "liga,dlig,smcp,c2sc,onum,lnum,pnum,tnum,frac,ordn,sups,subs,ss01,ss02,ss03,ss04,ss05,kern"
FIRA_FEATURES = "smcp,c2sc,liga,kern,tnum,lnum"
JBM_FEATURES = "liga,kern,calt"


def google(family: str, name: str) -> str:
    return f"{GOOGLE}/{family}/{urllib.request.quote(name)}"


# (family, source URL, output, features, extra unicodes, wght instance).
# The faces the client asks for: Spectral 400, 500, 600, 700 and italic 400
# and 600; Fira Sans 400, 500 and 600; JetBrains Mono 400.
FACES = [
    ("spectral", google("spectral", "Spectral-Regular.ttf"),        "Spectral-Regular.woff2",        SPECTRAL_FEATURES, SPECTRAL_EXTRA, None),
    ("spectral", google("spectral", "Spectral-Italic.ttf"),         "Spectral-Italic.woff2",         SPECTRAL_FEATURES, SPECTRAL_EXTRA, None),
    ("spectral", google("spectral", "Spectral-Medium.ttf"),         "Spectral-Medium.woff2",         SPECTRAL_FEATURES, SPECTRAL_EXTRA, None),
    ("spectral", google("spectral", "Spectral-SemiBold.ttf"),       "Spectral-SemiBold.woff2",       SPECTRAL_FEATURES, SPECTRAL_EXTRA, None),
    ("spectral", google("spectral", "Spectral-SemiBoldItalic.ttf"), "Spectral-SemiBoldItalic.woff2", SPECTRAL_FEATURES, SPECTRAL_EXTRA, None),
    ("spectral", google("spectral", "Spectral-Bold.ttf"),           "Spectral-Bold.woff2",           SPECTRAL_FEATURES, SPECTRAL_EXTRA, None),
    ("firasans", FIRA_TTF + "/FiraSans-Regular.ttf",                "FiraSans-Regular.woff2",        FIRA_FEATURES,     "",             None),
    ("firasans", FIRA_TTF + "/FiraSans-Medium.ttf",                 "FiraSans-Medium.woff2",         FIRA_FEATURES,     "",             None),
    ("firasans", FIRA_TTF + "/FiraSans-SemiBold.ttf",               "FiraSans-SemiBold.woff2",       FIRA_FEATURES,     "",             None),
    ("jetbrainsmono", google("jetbrainsmono", "JetBrainsMono[wght].ttf"), "JetBrainsMono-Regular.woff2", JBM_FEATURES, JBM_EXTRA, 400),
]
# family -> (notice written beside the fonts, upstream licence text)
LICENCES = {"spectral": ("OFL-Spectral.txt", google("spectral", "OFL.txt")),
            "firasans": ("OFL-FiraSans.txt", BBOX + "/OFL.txt"),
            "jetbrainsmono": ("OFL-JetBrainsMono.txt", google("jetbrainsmono", "OFL.txt"))}

RENAME = {"firasans": ("Fira Sans", "Arcana Sans")}
PRIMARY_NAME_IDS = {1, 3, 4, 6, 16, 17, 18, 21, 22}

# What each family must cover after subsetting: the diacritics of catalog
# names (Lim-Dûl, Jötun, Æther) and, in mono, box drawing.
MUST_COVER = {
    "spectral": "ûöÆæéł←→",
    "firasans": "ûöÆæéł←→",
    "jetbrainsmono": "─│┌┐└┘",
}

# Fallback faces, as on the site: each set stands in for one web family on
# the systems that have its reference font, under its full and PostScript
# names (what local() matches). A weight range takes the semibold's measure.
STYLE_WORDS = {"Regular": "", "Italic": " Italic", "Bold": " Bold", "BoldItalic": " Bold Italic"}


def full(family: str, style: str, regular: str = "") -> str:
    return family + (STYLE_WORDS[style] or regular)


def ps(prefix: str, style: str, regular: str = "-Regular") -> str:
    return prefix + ("-" + style if style != "Regular" else regular)


def ms(prefix: str, style: str) -> str:          # Monotype's: TimesNewRomanPS-BoldMT
    return prefix + ("-" + style if style != "Regular" else "") + "MT"


def weight_of(style: str) -> int:
    return 700 if "Bold" in style else 400


SERIF = [("Regular", "400", "normal", "Spectral-Regular.woff2"),
         ("Italic", "400", "italic", "Spectral-Italic.woff2"),
         ("Bold", "600 700", "normal", "Spectral-SemiBold.woff2"),
         ("BoldItalic", "600 700", "italic", "Spectral-SemiBoldItalic.woff2")]
SANS = [("Regular", "400", "normal", "FiraSans-Regular.woff2"),
        ("Bold", "600 700", "normal", "FiraSans-SemiBold.woff2")]

FALLBACK_SETS = [
    ("Spectral on Georgia", SERIF,
     lambda s: [full("Georgia", s), ps("Georgia", s, ""), full("Gelasio", s, " Regular"), ps("Gelasio", s)],
     lambda s: (google("gelasio", "Gelasio-Italic[wght].ttf" if "Italic" in s else "Gelasio[wght].ttf"), weight_of(s))),
    ("Spectral on Times", SERIF,
     lambda s: [full("Times New Roman", s), ms("TimesNewRomanPS", s), full("Liberation Serif", s),
                ps("LiberationSerif", s, ""), full("Tinos", s), ps("Tinos", s)],
     lambda s: (google("tinos", f"Tinos-{s}.ttf"), None)),
    ("Spectral on Noto", SERIF,
     lambda s: [full("Noto Serif", s), ps("NotoSerif", s)],
     lambda s: (google("notoserif", "NotoSerif-Italic[wdth,wght].ttf" if "Italic" in s else "NotoSerif[wdth,wght].ttf"), weight_of(s))),
    ("Fira Sans on Arial", SANS,
     lambda s: [full("Arial", s), ms("Arial", s), full("Liberation Sans", s), ps("LiberationSans", s, ""),
                full("Arimo", s), ps("Arimo", s)],
     lambda s: (google("arimo", "Arimo[wght].ttf"), weight_of(s))),
    ("Fira Sans on Roboto", SANS,
     lambda s: [full("Roboto", s), ps("Roboto", s)],
     lambda s: (google("roboto", "Roboto[wdth,wght].ttf"), weight_of(s))),
]
FALLBACK_BEGIN = "/* BEGIN fallback faces — written by arcana-web/scripts/vendor-assets.py */\n"
FALLBACK_END = "/* END fallback faces */\n"

MANA_FACE = ('@font-face {\n'
             '  font-family: "Mana";\n'
             '  src: url("/assets/fonts/mana.woff2") format("woff2");\n'
             '  font-weight: normal;\n'
             '  font-style: normal;\n'
             '}\n')

MANA_HEADER = """/*
 * Mana 1.17.1, https://github.com/andrewgioia/mana at 957ad2ab, css/mana.css.
 * Vendored by arcana-web/scripts/vendor-assets.py. One change from upstream:
 * its two @font-face rules (Mana as EOT, WOFF, TTF and SVG; MPlantin) are
 * replaced by one rule for the WOFF2 this server serves. MPlantin is not
 * vendored.
 *
 * The mana, tap and card type symbols are copyright Wizards of the Coast.
 * The Mana font is licensed under the SIL OFL 1.1 (fonts/OFL-Mana.txt).
 * The stylesheet is licensed under the MIT License:
 *
 * Copyright (c) Andrew Gioia
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
"""

# Mana's repository carries no OFL.txt and its font no copyright record; its
# README states the licence, so the notice says where it comes from.
MANA_OFL_HEAD = """Mana 1.17.1 (https://mana.andrewgioia.com), by Andrew Gioia. The font
declares no copyright notice and no Reserved Font Name; its README states
that the font is licensed under the SIL OFL 1.1, and that the mana, tap and
card type symbols are copyright Wizards of the Coast.

This Font Software is licensed under the SIL Open Font License, Version 1.1."""


def fetch(url: str) -> bytes:
    expected = SOURCE_HASHES[url]
    cached = CACHE / hashlib.sha256(url.encode()).hexdigest()[:16] / url.rsplit("/", 1)[1]
    if cached.is_file():
        data = cached.read_bytes()
    else:
        with urllib.request.urlopen(url, timeout=60) as r:
            data = r.read()
    if hashlib.sha256(data).hexdigest() != expected:
        raise ValueError(f"source checksum mismatch: {url}; cache: {cached}")
    if not cached.is_file():
        cached.parent.mkdir(parents=True, exist_ok=True)
        cached.write_bytes(data)
    print(f"  {url.rsplit('/', 1)[1]}  sha256 {expected[:16]}…  ({len(data) // 1024} KB)")
    return data


def catalog_text() -> str:
    """The catalog's string literals, escapes decoded: card names and ability
    text, which is what the client prints of it."""
    parts = []
    for rs in sorted(CATALOG.rglob("*.rs")):
        src = rs.read_text(encoding="utf-8")
        for lit in re.findall(r'"((?:[^"\\\n]|\\.)*)"', src):
            lit = re.sub(r"\\u\{([0-9A-Fa-f]+)\}", lambda m: chr(int(m.group(1), 16)), lit)
            parts.append(re.sub(r"\\(.)", r"\1", lit))
    return "\n".join(parts)


def rename(font: TTFont, old: str, new: str) -> None:
    for rec in font["name"].names:
        if rec.nameID in PRIMARY_NAME_IDS:
            text = rec.toUnicode()
            rec.string = text.replace(old, new).replace(old.replace(" ", ""), new.replace(" ", ""))
    left = [r.nameID for r in font["name"].names if r.nameID in PRIMARY_NAME_IDS and "Fira" in r.toUnicode()]
    if left:
        sys.exit(f"name records {left} still carry the reserved name")


def build(family, url, out, features, extra, wght, catalog_chars) -> str:
    """Write one subset; return the font's own copyright notice."""
    # recalcTimestamp off: the same sources give the same bytes.
    font = TTFont(io.BytesIO(fetch(url)), recalcTimestamp=False)
    notice = font["name"].getDebugName(0)
    if wght is not None:
        font = instancer.instantiateVariableFont(font, {"wght": wght})
    upstream = font.getBestCmap()
    options = subset.Options()
    options.layout_features = features.split(",")
    options.name_IDs = ["*"]
    options.hinting = False
    options.desubroutinize = True
    unicodes = set(subset.parse_unicodes(BASE + ("," + extra if extra else "")))
    unicodes |= {ord(c) for c in catalog_chars}
    sub = subset.Subsetter(options)
    sub.populate(unicodes=sorted(unicodes))
    sub.subset(font)
    if family in RENAME:
        rename(font, *RENAME[family])
    dest = FONTS / out
    before = dest.stat().st_size if dest.is_file() else 0
    font.flavor = "woff2"
    font.save(str(dest))
    cmap = TTFont(str(dest)).getBestCmap()
    missing = [c for c in MUST_COVER[family] if ord(c) not in cmap]
    if missing:
        sys.exit(f"{out}: upstream lacks {''.join(missing)}")
    lacking = sorted(c for c in catalog_chars if ord(c) not in upstream)
    print(f"  → {out}: {before // 1024} → {dest.stat().st_size // 1024} KB"
          + (f"; upstream lacks catalog characters {''.join(lacking)!r}" if lacking else ""))
    return notice


def write_licence(family: str, notice: str) -> None:
    """The upstream OFL.txt, headed by the copyright notice the font itself
    carries (they differ for Fira Sans, whose 4.301 name table keeps the
    Reserved Font Name bBox's OFL.txt drops)."""
    name, url = LICENCES[family]
    text = fetch(url).decode("utf-8").replace("\r\n", "\n")
    head, sep, body = text.partition("\n\n")
    if head.strip() != notice:
        print(f"  {name}: notice taken from the font, not the upstream text")
        text = notice + sep + body
    (FONTS / name).write_text(text, encoding="utf-8")


def instance(font: TTFont, wght: int | None) -> TTFont:
    """A static instance: wght as asked, every other axis at its default."""
    if wght is None or "fvar" not in font:
        return font
    axes = {a.axisTag: (wght if a.axisTag == "wght" else a.defaultValue) for a in font["fvar"].axes}
    return instancer.instantiateVariableFont(font, axes)


def mean_advance(font: TTFont, freq: collections.Counter, chars: set[str]) -> float:
    cmap, hmtx, upm = font.getBestCmap(), font["hmtx"], font["head"].unitsPerEm
    total = sum(freq[c] for c in chars)
    return sum(freq[c] * hmtx[cmap[ord(c)]][0] for c in chars) / upm / total


def vertical_metrics(font: TTFont) -> tuple[int, int, int]:
    """Ascent, descent and line gap as browsers read them: OS/2's typo
    values when USE_TYPO_METRICS is set, else hhea's."""
    os2 = font["OS/2"]
    if os2.fsSelection & (1 << 7):
        return os2.sTypoAscender, -os2.sTypoDescender, os2.sTypoLineGap
    hhea = font["hhea"]
    return hhea.ascent, -hhea.descent, hhea.lineGap


def fallback_rule(family, weight, style, names, webfont, ref, freq) -> tuple[str, float]:
    common = {c for c in freq if ord(c) in webfont.getBestCmap() and ord(c) in ref.getBestCmap()}
    adjust = mean_advance(webfont, freq, common) / mean_advance(ref, freq, common)
    upm = webfont["head"].unitsPerEm
    # The overrides are scaled by size-adjust in turn, so divide it out.
    asc, desc, gap = (f"{100 * v / upm / adjust:.1f}%" for v in vertical_metrics(webfont))
    src = ", ".join(f'local("{n}")' for n in dict.fromkeys(names))
    return (f'@font-face {{\n'
            f'  font-family: "{family}";\n'
            f'  src: {src};\n'
            f'  font-weight: {weight};\n'
            f'  font-style: {style};\n'
            f'  size-adjust: {100 * adjust:.1f}%;\n'
            f'  ascent-override: {asc};\n'
            f'  descent-override: {desc};\n'
            f'  line-gap-override: {gap};\n'
            f'}}\n'), adjust


def write_fallbacks(text: str) -> None:
    freq = collections.Counter(re.sub(r"\s+", " ", text))
    rules = []
    for family, faces, names_for, reference in FALLBACK_SETS:
        for style_name, weight, style, web in faces:
            ref_url, wght = reference(style_name)
            ref = instance(TTFont(io.BytesIO(fetch(ref_url))), wght)
            rule, adjust = fallback_rule(family, weight, style, names_for(style_name),
                                         TTFont(str(FONTS / web)), ref, freq)
            rules.append(rule)
            print(f"  {family} {weight} {style}: size-adjust {100 * adjust:.1f}%")
    css = THEME_CSS.read_text(encoding="utf-8")
    start, end = css.find(FALLBACK_BEGIN), css.find(FALLBACK_END)
    if start < 0 or end < start:
        sys.exit(f"{THEME_CSS.name}: no fallback-face markers")
    THEME_CSS.write_text(css[:start + len(FALLBACK_BEGIN)] + "".join(rules) + css[end:], encoding="utf-8")


def vendor_mana(ofl_body: str) -> None:
    (FONTS / "mana.woff2").write_bytes(fetch(MANA + "/fonts/mana.woff2"))
    css = fetch(MANA + "/css/mana.css").decode("utf-8")
    faces = re.findall(r"@font-face \{[^}]*\}\n", css)
    if len(faces) != 2 or not css.startswith(faces[0] + faces[1]):
        sys.exit("mana.css: expected its two @font-face rules first")
    (ASSETS / "mana.css").write_text(MANA_HEADER + MANA_FACE + css[len(faces[0] + faces[1]):], encoding="utf-8")
    (FONTS / "OFL-Mana.txt").write_text(MANA_OFL_HEAD + ofl_body, encoding="utf-8")
    print("  → mana.woff2, mana.css, OFL-Mana.txt")


def main() -> None:
    FONTS.mkdir(parents=True, exist_ok=True)
    text = catalog_text()
    catalog_chars = {c for c in text if ord(c) > 0x7E}
    notices = {}
    for spec in FACES:
        notice = build(*spec, catalog_chars)
        if notices.setdefault(spec[0], notice) != notice:
            sys.exit(f"{spec[2]}: copyright differs from the family's other faces")
    for family, notice in notices.items():
        write_licence(family, notice)
    # The OFL's own text, from below Spectral's notice, under Mana's.
    spectral_ofl = fetch(LICENCES["spectral"][1]).decode("utf-8").replace("\r\n", "\n")
    vendor_mana(spectral_ofl[spectral_ofl.index("\nThis license is copied below"):])
    write_fallbacks(text)


if __name__ == "__main__":
    main()
