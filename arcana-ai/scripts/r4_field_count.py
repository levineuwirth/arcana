#!/usr/bin/env python3
"""r4_field_count.py — A3's field count (D14): which missing cards gate real decks.

    python3 -I arcana-ai/scripts/r4_field_count.py families EXTRACT ORACLE
    python3 -I arcana-ai/scripts/r4_field_count.py cards EXTRACT ORACLE --format PI [--top 25]
    python3 -I arcana-ai/scripts/r4_field_count.py greedy EXTRACT --format PI|ALL [--n 50]
    python3 -I arcana-ai/scripts/r4_field_count.py ltb ORACLE DECKS_PI MANIFEST CAPSULE_DIR

EXTRACT is `deck_corpus::tests::corpus_unresolved`'s output, one JSON object a
deck: format, source, resolved and listed copies, and the missing names with
copies. ORACLE is Scryfall's `oracle-cards` bulk file (JSON lines, gzipped).
docs/a3-field-count/README.md names the inputs by hash and the outputs these
commands produce.

A deck's size is its LISTED copies, resolved plus missing. A first count
filtered on resolved copies (`main_count`), which drops missing names, and so
found no blocked 60-card deck anywhere; test_r4_field_count.py holds that.

families: each missing card's requirement set, read from Scryfall's keywords,
mana cost, type line and oracle text, among R4's families: the cast-path
keywords (morph with megamorph and disguise, foretell, escape, overload,
emerge, prowl, spectacle, evoke, dash, surge, mutate), modal spells ("choose
one —" and kin, escalate, spree), {X} costs, Vehicles, and non-creature
enchantments with an activated ability. A card with none is `outside`; a name
Scryfall lacks is `unknown_name`. A 60-card deck is unblocked by a set of
families when every missing card's requirements lie inside it. The count is
a ranking signal from oracle text, not a census of engine work.

cards: the missing names touching the most decks of a format, each with the
number of decks it alone blocks.

greedy: repeatedly the card that completes the most blocked 60-card decks
(ties to the card touching most of the decks still blocked, then by name),
and how many decks the first 5, 10, 20, 30 and 50 make playable. Potential
catalog coverage, not a demonstrated faithful field.

ltb: maindeck cards in the pinned A2.2 field and the capsule whose oracle text
has a trigger of their own on dying or leaving the battlefield, the triggers
A1.0a stopped resolving to nothing. This measures oracle-ability exposure,
not effects the fix restored: a card's implementation may still stub the
ability (Spell Queller, A1.5l) or misread it (Hangarback Walker, A1.5m).
"""

import argparse
import collections
import csv
import gzip
import json
import pathlib
import re
import sys

CAST_PATHS = {"Morph": "morph", "Megamorph": "morph", "Disguise": "morph", "Foretell": "foretell",
              "Escape": "escape", "Overload": "overload", "Emerge": "emerge", "Prowl": "prowl",
              "Spectacle": "spectacle", "Evoke": "evoke", "Dash": "dash", "Surge": "surge", "Mutate": "mutate"}
MODAL = re.compile(r"\bchoose (one|two|three|four|one or more|one or both|any number)\b\s*[—-]", re.I)
ACTIVATED = re.compile(r"^(?:[^\n\"]*?)(\{[^}]+\}|\bSacrifice\b|\bPay\b|\bExile\b|\bDiscard\b|\bTap\b|\bRemove\b)"
                       r"[^\n:\"]*:", re.M)
SKIP_LAYOUTS = {"art_series", "token", "double_faced_token", "emblem"}


def faces(card):
    return card.get("card_faces") or [card]


def families(card):
    fam = set()
    keywords = card.get("keywords", [])
    fam.update(CAST_PATHS[k] for k in keywords if k in CAST_PATHS)
    for f in faces(card):
        text, tl, cost = f.get("oracle_text", ""), f.get("type_line", ""), f.get("mana_cost", "")
        if MODAL.search(text) or "Escalate" in keywords or "Spree" in keywords:
            fam.add("modal")
        if "{X}" in cost:
            fam.add("x_spell")
        if "Vehicle" in tl:
            fam.add("vehicle")
        if "Enchantment" in tl and "Creature" not in tl and ACTIVATED.search(text):
            fam.add("activated_enchantment")
    return fam


def load_oracle(path):
    by_name = {}
    with gzip.open(path, "rt", encoding="utf-8") as f:
        for line in f:
            c = json.loads(line)
            if c.get("layout") in SKIP_LAYOUTS:
                continue
            by_name.setdefault(c["name"].lower(), c)
            for face in c.get("card_faces", []):
                by_name.setdefault(face["name"].lower(), c)
    return by_name


def lookup(by_name, name):
    return by_name.get(name.lower()) or by_name.get(name.split(" // ")[0].lower())


def load_extract(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.startswith("{")]


def blocked_sixty(decks):
    """The 60-card decks (by listed copies) with at least one missing card."""
    return [d for d in decks if d["listed"] == 60 and d["missing"]]


def requirements(decks, by_name):
    req = {}
    for d in decks:
        for name, _ in d["missing"]:
            if name in req:
                continue
            card = lookup(by_name, name)
            if card is None:
                req[name] = frozenset({"unknown_name"})
            else:
                req[name] = frozenset(families(card)) or frozenset({"outside"})
    return req


def family_table(decks, req, fams):
    out = []
    for fam in fams + ["outside", "unknown_name"]:
        cards = {n for d in decks for n, _ in d["missing"] if fam in req[n]}
        copies = sum(c for d in decks for n, c in d["missing"] if fam in req[n])
        touched = sum(1 for d in decks if any(fam in req[n] for n, _ in d["missing"]))
        alone = sum(1 for d in blocked_sixty(decks) if all(req[n] <= {fam} for n, _ in d["missing"]))
        out.append({"family": fam, "cards": len(cards), "copies": copies, "touched": touched, "alone": alone})
    return sorted(out, key=lambda r: (-r["alone"], -r["touched"], r["family"]))


def cmd_families(args):
    decks = load_extract(args.extract)
    req = requirements(decks, load_oracle(args.oracle))
    fams = sorted({f for r in req.values() for f in r} - {"outside", "unknown_name"})
    r4 = set(fams)
    shapes = collections.Counter("+".join(sorted(r)) for r in req.values())
    print(f"decks {len(decks)}; distinct missing names {len(req)}; names not in Scryfall "
          f"{sum(1 for r in req.values() if 'unknown_name' in r)}")
    print("distinct missing cards by requirement: " + ", ".join(f"{k} {v}" for k, v in shapes.most_common()))
    for scope in ("PI", "MO", "LE", "VI", "ALL"):
        ds = [d for d in decks if scope == "ALL" or d["format"] == scope]
        playable = sum(1 for d in ds if d["listed"] == 60 and not d["missing"])
        print(f"\n== {scope}: {len(ds)} decks, {playable} playable at 60, {len(blocked_sixty(ds))} blocked at 60")
        print(f"{'family':<22} {'cards':>6} {'copies':>7} {'decks touched':>14} {'unblocked alone':>16}")
        for r in family_table(ds, req, fams):
            print(f"{r['family']:<22} {r['cards']:>6} {r['copies']:>7} {r['touched']:>14} {r['alone']:>16}")
        b = blocked_sixty(ds)
        allr4 = sum(1 for d in b if all(req[n] <= r4 for n, _ in d["missing"]))
        near = sum(1 for d in b if sum(1 for n, _ in d["missing"] if not req[n] <= r4) == 1)
        print(f"unblocked by closing every R4 family together: {allr4}; one card outside R4 short of it: {near}")


def cmd_cards(args):
    decks = [d for d in load_extract(args.extract) if args.format == "ALL" or d["format"] == args.format]
    by_name = load_oracle(args.oracle)
    touch = collections.Counter(n for d in decks for n in {m for m, _ in d["missing"]})
    sole = collections.Counter(d["missing"][0][0] for d in blocked_sixty(decks) if len(d["missing"]) == 1)
    b = blocked_sixty(decks)
    print(f"{args.format}: blocked 60-card decks {len(b)}; missing exactly one card {sum(sole.values())}; "
          f"exactly two {sum(1 for d in b if len(d['missing']) == 2)}")
    for name, t in sorted(touch.items(), key=lambda kv: (-kv[1], kv[0]))[:args.top]:
        card = lookup(by_name, name)
        fam = "+".join(sorted(families(card))) if card else "unknown_name"
        print(f"{t:>4} decks  sole {sole.get(name, 0):>3}  {name:<36} {fam or 'outside':<22} "
              f"{(card or {}).get('type_line', '?')[:40]}")


def greedy(decks, n=50):
    """[(card, decks it completes, running total)] and the totals at the marks."""
    sets = [{m for m, _ in d["missing"]} for d in blocked_sixty(decks)]
    picked, done, order, marks = set(), 0, [], {}
    while len(picked) < n:
        best, key = None, None
        for c in sorted({m for s in sets for m in s} - picked):
            g = sum(1 for s in sets if c in s and s - picked <= {c})
            t = sum(1 for s in sets if c in s and not s <= picked)
            if g == 0 and t == 0:
                continue
            if key is None or (g, t) > key:
                best, key = c, (g, t)
        if best is None or key[0] == 0 and key[1] == 0:
            break
        picked.add(best)
        done += key[0]
        order.append((best, key[0], done))
        if len(picked) in (5, 10, 20, 30, 50):
            marks[len(picked)] = done
    return order, marks, len(sets)


def cmd_greedy(args):
    decks = [d for d in load_extract(args.extract) if args.format == "ALL" or d["format"] == args.format]
    order, marks, blocked = greedy(decks, args.n)
    for k, (card, gain, total) in enumerate(order, 1):
        print(f"{k:>2}. {card:<36} +{gain:>3} decks (total {total})")
    print(f"{args.format}: {blocked} blocked 60-card decks; playable after the first N cards: "
          + ", ".join(f"{k}: {v}" for k, v in marks.items()))


def self_leaves_trigger(card):
    for f in faces(card):
        name = f["name"]
        selfref = "|".join(re.escape(x) for x in sorted({name, name.split(",")[0]})) + \
            r"|this (?:creature|artifact|enchantment|permanent|card|land|planeswalker|vehicle)"
        pat = re.compile(r"\bwhen(?:ever)? (?:" + selfref + r") (?:dies|leaves the battlefield|is put into "
                         r"(?:a|your|an opponent's|its owner's) graveyard from the battlefield)", re.I)
        if pat.search(f.get("oracle_text", "")):
            return True
    return False


def maindeck(path):
    out, in_deck = [], False
    for line in open(path, encoding="utf-8"):
        line = line.strip()
        if line == "Deck":
            in_deck = True
        elif line in ("Sideboard", "About"):
            in_deck = False
        elif in_deck and line:
            n, name = line.split(" ", 1)
            out.append((name, int(n)))
    return out


def cmd_ltb(args):
    by_name = load_oracle(args.oracle)
    rows = list(csv.DictReader(open(args.manifest, encoding="utf-8")))
    groups = [("A2.2 field", [pathlib.Path(args.decks_pi) / f"{r['source']}.txt" for r in rows]),
              ("capsule", sorted(pathlib.Path(args.capsule).glob("*.txt")))]
    print("oracle-ability exposure, not verified restoration: a card's implementation may still stub or misread it")
    for label, paths in groups:
        decks = [maindeck(p) for p in paths]
        names = {n for d in decks for n, _ in d}
        hit = sorted(n for n in names if (c := lookup(by_name, n)) and self_leaves_trigger(c))
        copies = sum(c for d in decks for n, c in d if n in hit)
        touched = sum(1 for d in decks if any(n in hit for n, _ in d))
        total = sum(c for d in decks for _, c in d)
        print(f"{label}: {len(decks)} decks, {len(names)} distinct cards; {len(hit)} with a trigger of their own on "
              f"dying or leaving, {copies} of {total} maindeck copies, in {touched} decks: {'; '.join(hit)}")


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("families"); p.add_argument("extract"); p.add_argument("oracle")
    p = sub.add_parser("cards"); p.add_argument("extract"); p.add_argument("oracle")
    p.add_argument("--format", default="PI"); p.add_argument("--top", type=int, default=25)
    p = sub.add_parser("greedy"); p.add_argument("extract"); p.add_argument("--format", default="PI")
    p.add_argument("--n", type=int, default=50)
    p = sub.add_parser("ltb")
    for a in ("oracle", "decks_pi", "manifest", "capsule"):
        p.add_argument(a)
    args = ap.parse_args(argv)
    {"families": cmd_families, "cards": cmd_cards, "greedy": cmd_greedy, "ltb": cmd_ltb}[args.cmd](args)
    return 0


if __name__ == "__main__":
    sys.exit(main())
