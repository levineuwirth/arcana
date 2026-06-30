#!/usr/bin/env python3
"""Convert the Kaggle MTGTop8 dataset (camilonunez) into Arena/MTGO decklist
text files the arcana-ai deck_corpus loader consumes.

The dataset stores per-player decks as JSON under
`events/<event_id>/players_decks/player_<pid>_deck.json`, with `event_format`
in `df_events_v2.csv`. Two quirks this handles:

  1. `main_deck` is DOUBLED: it is `A + B + A` where A = the real maindeck and
     B = the sideboard (so it sums to ~135 for a 60/15 deck). We recover A via
     the prefix==suffix structural check and fall back to the list as-is for the
     minority of clean (non-doubled) decks.
  2. Counts are strings ("4"); card names are plain (canonical).

Writes one `<event>_<pid>.txt` per DISTINCT maindeck (deduped by card multiset)
per format, capped at `--cap` per format. Standard (rotated, old snapshot) and
EDH (multiplayer, wrong for the 2p engine) are skipped by default.

Usage:
  python3 kaggle_to_decklists.py --zip kaggle.zip --out /tmp/kaggle_decks \
      --formats MO LE PI VI --cap 800
"""
import argparse, csv, io, json, os, re, zipfile, hashlib, collections

EV_RE = re.compile(r"events/(\d+)/")


def recover_maindeck(main, side):
    """Undo the A+B+A doubling; return the real maindeck entry list."""
    n, s = len(main), len(side)
    if n > s and (n - s) % 2 == 0:
        half = (n - s) // 2
        if half > 0 and main[:half] == main[half + s:]:
            return main[:half]
    return main


def deck_key(entries):
    """Order-independent identity of a maindeck (for dedup)."""
    norm = sorted((name, int(cnt)) for name, cnt in entries)
    return hashlib.sha1(repr(norm).encode()).hexdigest()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--zip", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--formats", nargs="+", default=["MO", "LE", "PI", "VI"])
    ap.add_argument("--cap", type=int, default=800, help="max distinct decks/format")
    args = ap.parse_args()

    z = zipfile.ZipFile(args.zip)
    ev_fmt = {}
    with z.open("df_events_v2.csv") as f:
        for row in csv.DictReader(io.TextIOWrapper(f, "utf-8")):
            ev_fmt[row["event__id"]] = row["event_format"]

    want = set(args.formats)
    seen = {fmt: set() for fmt in want}
    written = collections.Counter()
    skipped_size = collections.Counter()

    members = [n for n in z.namelist() if n.endswith("_deck.json")]
    members.sort()  # deterministic
    for name in members:
        m = EV_RE.search(name)
        if not m:
            continue
        fmt = ev_fmt.get(m.group(1))
        if fmt not in want or written[fmt] >= args.cap:
            continue
        try:
            d = json.loads(z.read(name))
        except Exception:
            continue
        entries = recover_maindeck(d.get("main_deck", []), d.get("sideboard", []))
        total = sum(int(c) for _, c in entries)
        if not (40 <= total <= 100):  # drop malformed / non-constructed sizes
            skipped_size[fmt] += 1
            continue
        key = deck_key(entries)
        if key in seen[fmt]:
            continue
        seen[fmt].add(key)

        out_dir = os.path.join(args.out, fmt)
        os.makedirs(out_dir, exist_ok=True)
        pid = re.search(r"player_(\d+)_deck", name).group(1)
        fname = f"{m.group(1)}_{pid}.txt"
        with open(os.path.join(out_dir, fname), "w", encoding="utf-8") as out:
            out.write("Deck\n")
            for cardname, cnt in entries:
                out.write(f"{int(cnt)} {cardname}\n")
        written[fmt] += 1

    print("wrote distinct decks per format:")
    for fmt in args.formats:
        print(f"  {fmt}: {written[fmt]} (size-skipped {skipped_size[fmt]})")


if __name__ == "__main__":
    main()
