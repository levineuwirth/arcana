#!/usr/bin/env python3
"""A1.5s: lift the sorcery-speed gate from the catalog's activated abilities
that the rules let a player activate whenever they have priority.

The engine gated every non-mana activated ability to sorcery timing unless
its `ActivatedAbilityDef` set `is_instant_speed: true`, and the generator
taught `false` as the default, so most of the catalog's abilities were gated
against the rules (CR 602.2: an activated ability can be activated whenever
its controller has priority, unless it says otherwise). This script decides,
for every gated non-mana, non-loyalty ability literal under
`arcana-cards/src`, whether it flips to `true` or keeps the gate, and with
`--apply` makes the edits.

An ability keeps the gate when its file text, or its card's oracle line
matched reliably to it, names a sorcery-speed restriction (sorcery-speed
keywords included) or another timing restriction (A1.5v, left gated). It
goes to hand review, decided in `timing_review.tsv`, when:
- its card's oracle text names any timing restriction (the restriction may
  belong to it even when the per-ability match says otherwise);
- its text is a triggered ability's ("At the beginning", "When"), or is
  built non-literally;
- it has no oracle entry, or matches its card's oracle only weakly, and its
  card's oracle text is missing.
Every other gated ability flips. A kept ability whose file text drops its
oracle's sorcery clause gets that sentence appended to its text, so the
catalog ratchet (`activated_ability_timing_matches_text`) can hold it.

The oracle file is Scryfall's oracle-cards bulk, as `arcana-gen` caches it;
the sweep was decided against sha256 201cfa0a…ce8e (2026-10-06).

Usage (from the repository root):
    python3 -I arcana-gen/scripts/timing_sweep.py \
        [--oracle target/scryfall-cache/oracle-cards.jsonl.gz] \
        [--review arcana-gen/scripts/timing_review.tsv] [--table PATH] [--apply]
"""
import argparse, collections, csv, difflib, glob, gzip, json, os, re, sys

SORCERY = re.compile(r'only as a sorcery|could cast a sorcery', re.I)
SORCERY_KEYWORD = re.compile(
    r'^(equip|fortify|level up|outlast|reconfigure|scavenge|embalm|eternalize|unearth|'
    r'encore|craft with|transmute|transfigure|saddle|station|specialize|plot|spark)\b'
    r'|^(\{[^}]+\})+: level \d', re.I)
OTHER_TIMING = re.compile(
    r'only during|only before|only after|'
    r'only any time you could cast an instant during|^forecast\b|ninjutsu', re.I)
CARD_TIMING = re.compile(
    SORCERY.pattern + '|' + OTHER_TIMING.pattern + r'|^\s*(equip|fortify|level up|outlast|'
    r'reconfigure|scavenge|embalm|eternalize|unearth|encore|craft with|transmute|'
    r'transfigure|saddle|station|specialize|spark)\b', re.I | re.M)
TRIGGERED = re.compile(r'^(at the beginning|when|whenever)\b', re.I)


def block_end(s, i):
    """Index of the brace closing the one at `i`, skipping strings and comments."""
    depth, j, n = 0, i, len(s)
    while j < n:
        c = s[j]
        if c == '"':
            j += 1
            while j < n and s[j] != '"':
                j += 2 if s[j] == '\\' else 1
        elif c == '/' and s[j + 1:j + 2] == '/':
            while j < n and s[j] != '\n':
                j += 1
        elif c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return j
        j += 1
    raise ValueError('unbalanced block')


def text_literal(block):
    """(decoded text, span of the literal's contents in `block`), or a marker
    and None when the text is not one string literal."""
    m = re.search(r'\btext:\s*', block)
    if not m:
        return 'NONE', None
    rest = block[m.end():]
    if rest.startswith('format!'):
        return 'FMT', None
    m2 = re.match(r'"((?:[^"\\]|\\.)*)"', rest, re.S)
    if not m2:
        return 'NONLIT', None
    raw = m2.group(1)
    t = re.sub(r'\\\n\s*', '', raw).replace('\\"', '"')
    start = m.end() + 1
    return t, (start, start + len(raw))


def extract(root):
    out = []
    for f in sorted(glob.glob(os.path.join(root, 'arcana-cards/src/**/*.rs'), recursive=True)):
        if f.endswith(('/lib.rs', '/register_all.rs', '/mod.rs')):
            continue
        s = open(f, encoding='utf-8').read()
        name = re.search(r'intern(?:_mut\(\)\.intern)?\("([^"]+)"\)', s)
        tests = s.find('#[cfg(test)]')
        for m in re.finditer(r'ActivatedAbilityDef\s*\{', s):
            before = s[max(0, m.start() - 8):m.start()]
            if re.search(r'->\s*$|\bfor\s+$|\bimpl\s+$', before):
                continue
            bi = m.end() - 1
            be = block_end(s, bi)
            b = s[bi:be + 1]
            if 'is_instant_speed' not in b:
                continue
            flag = lambda k: (re.search(r'\b' + k + r':\s*(true|false)', b) or [None, None])[1] == 'true'
            text, span = text_literal(b)
            out.append(dict(file=os.path.relpath(f, root), line=s.count('\n', 0, m.start()) + 1,
                            start=bi, end=be + 1, card=name.group(1) if name else '',
                            text=text, span=span, mana=flag('is_mana_ability'),
                            loyalty=flag('is_loyalty_ability'), instant=flag('is_instant_speed'),
                            in_test=tests != -1 and m.start() > tests))
    return out


def load_oracle(path):
    # Art-series entries share their card's name and carry no rules text;
    # an empty entry must never stand for a card.
    cards = {}
    for line in gzip.open(path, 'rt', encoding='utf-8'):
        c = json.loads(line)
        if c.get('layout') == 'art_series':
            continue
        faces = c.get('card_faces') or []
        full = c.get('oracle_text') or '\n'.join(f.get('oracle_text', '') for f in faces)
        if not full.strip():
            continue
        cards.setdefault(c['name'], full)
        for f in faces:
            cards.setdefault(f.get('name'), full)
    return cards


def norm(t):
    return re.sub(r'\s+', ' ', re.sub(r'\([^)]*\)', '', t or '')).strip().lower()


def match_line(text, full):
    """The oracle line that `text` most likely transcribes, and whether the
    match is reliable (same cost, or a close textual match)."""
    lines = [l for l in full.split('\n') if l.strip()]
    if not lines:
        return '', False
    cost = norm(text).split(':')[0] if ':' in text else None
    same = [l for l in lines if cost and norm(l).split(':')[0].split('— ')[-1] == cost]
    best = same[0] if same else max(lines, key=lambda l: difflib.SequenceMatcher(None, norm(l), norm(text)).ratio())
    sim = difflib.SequenceMatcher(None, norm(best), norm(text)).ratio()
    return best, bool(same) or sim >= 0.6


def timing_of(line):
    if SORCERY.search(line) or SORCERY_KEYWORD.search(line.strip()):
        return 'sorcery'
    if OTHER_TIMING.search(line):
        return 'other'
    return None


def decide(r, oracle):
    """('flip' | 'keep' | 'review', reason, sorcery sentence to append or None)."""
    t = r['text']
    if t in ('NONE', 'NONLIT', 'FMT'):
        return 'review', 'text is not a single literal', None
    if TRIGGERED.search(t.strip()):
        return 'review', 'triggered wording in an activation', None
    own = timing_of(t)
    if own == 'sorcery':
        return 'keep', 'sorcery: file text', None
    if own == 'other':
        return 'keep', 'other timing: file text (A1.5v)', None
    full = oracle.get(r['card']) or oracle.get(r['card'].removeprefix('A-'))
    if full is None:
        return 'review', 'no oracle entry', None
    line, reliable = match_line(t, full)
    if reliable and timing_of(line) == 'sorcery':
        m = SORCERY.search(line)
        sentence = None
        if m:
            sentence = re.search(r'[^.]*' + re.escape(m.group(0)) + r'[^.]*\.?', line).group(0)
            sentence = sentence.strip().lstrip('"\u201d ').strip()
            sentence = sentence if sentence.endswith('.') else sentence + '.'
            assert sentence[:1].isupper(), (r['file'], sentence)
        return 'keep', 'sorcery: oracle line', sentence
    if reliable and timing_of(line) == 'other':
        return 'keep', 'other timing: oracle line (A1.5v)', None
    if CARD_TIMING.search(full):
        return 'review', 'card oracle names a timing restriction', None
    return 'flip', 'instant by rule' if reliable else 'instant by rule: card oracle has no timing restriction', None


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--root', default='.')
    ap.add_argument('--oracle', default='target/scryfall-cache/oracle-cards.jsonl.gz')
    ap.add_argument('--review', default='arcana-gen/scripts/timing_review.tsv')
    ap.add_argument('--table')
    ap.add_argument('--apply', action='store_true')
    a = ap.parse_args()

    oracle = load_oracle(a.oracle)
    review = {}
    for row in csv.DictReader(open(a.review, encoding='utf-8'), delimiter='\t'):
        review[(row['file'], row['text'])] = row
    every = [r for r in extract(a.root) if not r['in_test'] and not r['mana'] and not r['loyalty']]
    rows = [r for r in every if not r['instant']]
    used, undecided = set(), []
    for r in rows:
        r['decision'], r['reason'], r['append'] = decide(r, oracle)
        key = (r['file'], r['text'])
        if key in review:
            used.add(key)
            r['decision'], r['reason'] = review[key]['decision'], 'review: ' + review[key]['reason']
        elif r['decision'] == 'review':
            undecided.append(r)
    # A review row is stale when its ability is gone; after the sweep, the
    # rows it flipped match instant abilities and are spent, not stale.
    stale = set(review) - used - {(r['file'], r['text']) for r in every if r['instant']}

    by = collections.Counter((r['decision'], r['reason'].split(':')[0]) for r in rows)
    print(f'{len(rows)} gated non-mana, non-loyalty abilities in {len({r["file"] for r in rows})} files')
    for (d, why), n in sorted(by.items()):
        print(f'  {d:6} {why:34} {n}')
    flips = [r for r in rows if r['decision'] == 'flip']
    print(f'flip: {len(flips)} abilities in {len({r["file"] for r in flips})} files; '
          f'clauses to append: {sum(1 for r in rows if r["decision"] == "keep" and r["append"])}')
    if a.table:
        with open(a.table, 'w', encoding='utf-8', newline='') as fh:
            w = csv.writer(fh, delimiter='\t', lineterminator='\n')
            w.writerow(['file', 'line', 'card', 'decision', 'reason', 'append', 'text'])
            for r in rows:
                w.writerow([r['file'], r['line'], r['card'], r['decision'], r['reason'], r['append'] or '', r['text']])
    for r in undecided:
        print(f'UNDECIDED {r["file"]}:{r["line"]} {r["card"]}: {r["reason"]}: {r["text"][:80]}', file=sys.stderr)
    for f, t in sorted(stale):
        print(f'STALE review row {f}: {t[:80]}', file=sys.stderr)
    if undecided or stale:
        sys.exit(1)
    if not a.apply:
        return

    edits = collections.defaultdict(list)
    for r in rows:
        if r['decision'] == 'flip' or (r['decision'] == 'keep' and r['append']):
            edits[r['file']].append(r)
    for f, rs in edits.items():
        p = os.path.join(a.root, f)
        s = open(p, encoding='utf-8').read()
        for r in sorted(rs, key=lambda r: -r['start']):
            b = s[r['start']:r['end']]
            if r['decision'] == 'flip':
                assert b.count('is_instant_speed: false') == 1, (f, r['line'])
                b = b.replace('is_instant_speed: false', 'is_instant_speed: true')
            else:
                assert r['span'], (f, r['line'])
                e = r['span'][1]
                b = b[:e] + ' ' + r['append'].replace('"', '\\"') + b[e:]
            s = s[:r['start']] + b + s[r['end']:]
        open(p, 'w', encoding='utf-8').write(s)
    print(f'applied: {len(edits)} files')


if __name__ == '__main__':
    main()
