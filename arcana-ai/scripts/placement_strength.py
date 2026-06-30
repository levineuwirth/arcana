#!/usr/bin/env python3
"""Extract per-archetype real-world tournament strength from the Kaggle MTGTop8
dump, as a GROUND-TRUTH reference for the gauntlet rankings (rl-status §6.5 #4).

For one format, walk every event of that format, read each player's finish
(`player_result`, e.g. "1", "3-4", "9-16") and archetype (`player_title`),
normalize the finish within its event, and aggregate per archetype:

  entries      how many times the archetype was played (metagame share signal)
  strength     1 - mean(normalized finish); higher = finishes better on average
               (normalized finish = (rank-1)/(fieldsize-1), 0 = event winner)
  top_share    fraction of entries that placed in the top eighth of their event

This is intentionally a coarse, metagame-CONFOUNDED signal (popular archetypes
regress to the mean; field strength varies by event) — it is a sanity reference
for rank-correlation, not training truth.

Usage:
  python3 arcana-ai/scripts/placement_strength.py --zip kaggle.zip --format PI \
      --min-entries 20 > docs/gauntlet-results/metagame-placement-PI.txt
"""
import argparse
import csv
import io
import re
import zipfile
from collections import defaultdict


def parse_result(s):
    """A finish string -> (lower_rank, upper_rank). '3-4' -> (3,4); '1' -> (1,1).
    Returns None if unparseable."""
    s = (s or "").strip()
    if not s:
        return None
    m = re.match(r"^(\d+)\s*-\s*(\d+)$", s)
    if m:
        return int(m.group(1)), int(m.group(2))
    m = re.match(r"^(\d+)$", s)
    if m:
        r = int(m.group(1))
        return r, r
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--zip", required=True)
    ap.add_argument("--format", required=True, help="2-letter format code (PI, MO, LE, VI, ST)")
    ap.add_argument("--min-entries", type=int, default=20,
                    help="drop archetypes with fewer than this many total entries")
    ap.add_argument("--min-stars", type=int, default=0,
                    help="only count events with at least this many stars (event weight)")
    args = ap.parse_args()

    z = zipfile.ZipFile(args.zip)

    # 1. event id -> (format, stars)
    events = {}
    ev = csv.DictReader(io.StringIO(z.read("df_events_v2.csv").decode("utf-8", "replace")))
    for row in ev:
        try:
            stars = int(row.get("event_stars") or 0)
        except ValueError:
            stars = 0
        events[row["event__id"]] = (row.get("event_format", ""), stars)

    fmt = args.format
    target_ids = [eid for eid, (f, st) in events.items()
                  if f == fmt and st >= args.min_stars]

    # 2. walk each event's players_info.csv
    names = set(z.namelist())
    agg = defaultdict(lambda: {"entries": 0, "sum_norm": 0.0, "top": 0})
    n_events = 0
    n_entries = 0
    for eid in target_ids:
        path = f"events/{eid}/players_info.csv"
        if path not in names:
            continue
        try:
            rows = list(csv.DictReader(io.StringIO(z.read(path).decode("utf-8", "replace"))))
        except KeyError:
            continue
        parsed = []
        for r in rows:
            pr = parse_result(r.get("player_result"))
            title = (r.get("player_title") or "").strip()
            if pr is None or not title:
                continue
            parsed.append((title, pr))
        if len(parsed) < 2:
            continue
        # field size = largest upper bound seen (best proxy for # players)
        field = max(up for _, (_, up) in parsed)
        if field < 2:
            continue
        n_events += 1
        for title, (lo, _up) in parsed:
            norm = (lo - 1) / (field - 1)        # 0 = winner, 1 = last
            a = agg[title]
            a["entries"] += 1
            a["sum_norm"] += norm
            if (lo - 1) / field < 0.125:          # top eighth of the field
                a["top"] += 1
            n_entries += 1

    # 3. rank archetypes by strength (min-entries gate)
    table = []
    for title, a in agg.items():
        if a["entries"] < args.min_entries:
            continue
        mean_norm = a["sum_norm"] / a["entries"]
        table.append((title, a["entries"], 1.0 - mean_norm, a["top"] / a["entries"]))
    table.sort(key=lambda t: t[2], reverse=True)

    print(f"# Real-world archetype strength — format {fmt} "
          f"(min {args.min_entries} entries, min {args.min_stars} stars)")
    print(f"# events used: {n_events}  total entries: {n_entries}  "
          f"distinct archetypes (>= min): {len(table)}")
    print(f"# strength = 1 - mean normalized finish (higher=better); "
          f"top_share = frac in top 1/8 of field")
    print(f"# {'archetype':<34} {'entries':>7} {'strength':>9} {'top_share':>10}")
    for title, entries, strength, top in table:
        print(f"  {title:<34} {entries:>7} {strength:>9.3f} {top:>10.3f}")


if __name__ == "__main__":
    main()
