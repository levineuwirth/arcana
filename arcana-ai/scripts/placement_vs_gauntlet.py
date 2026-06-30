#!/usr/bin/env python3
"""Rank-correlate our gauntlet point-rate against real MTGTop8 placement strength
(rl-status §6.5 #4, the ground-truth sanity check).

Both name spaces are fragmented (the dump itself splits "RDW" vs "Red Deck
Wins"; our gauntlet has a dozen spellings of "Hardened Scales"), so we collapse
each side into a small set of CANONICAL archetype buckets via an explicit,
conservative alias map, then inner-join and compute Spearman rho. Only buckets
present on BOTH sides are correlated; N is reported.

  real strength  = entry-weighted (1 - mean normalized finish) over the bucket's
                   real-name variants (from placement_strength.py logic)
  gauntlet pr    = mean point-rate over the bucket's gauntlet decklist instances

Usage:
  python3 arcana-ai/scripts/placement_vs_gauntlet.py \
      --zip kaggle.zip --format PI --gauntlet docs/gauntlet-results/pi_gauntlet_62.csv
"""
import argparse
import csv
import io
import re
import zipfile
from collections import defaultdict

# Canonical bucket -> (gauntlet-name substrings, real-name exact variants).
# Matching is case-insensitive; gauntlet side is substring, real side is exact
# (the real labels are cleaner). Keep buckets conservative — only confident ones.
PI_BUCKETS = {
    "Gruul/RG Aggro":   (["gruul aggro", "rg aggro"], ["Gruul Aggro"]),
    "Mono-Red (RDW)":   (["red deck wins", "rakdos deck wins"], ["RDW", "Red Deck Wins"]),
    "UW Spirit Aggro":  (["uw spirit aggro", "spirit", "spirits"], ["UW Spirit Aggro", "Spirit Aggro"]),
    "Golgari/HardScales":(["golgari", "hardened", "harden ", "scales", "snakes"], ["Golgari Aggro"]),
    "Sultai Control":   (["sultai control"], ["Sultai Control"]),
    "Mono-Green Aggro": (["mono green", "devotion to g"], ["Mono Green Aggro", "Devotion to Green"]),
    "Rakdos Aggro":     (["rakdos aggro"], ["Mono Black Aggro"]),  # nearest BR/Bx aggro proxy
}


def parse_result(s):
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


def real_strength_by_name(zip_path, fmt):
    """name -> (entries, strength) over all events of `fmt`."""
    z = zipfile.ZipFile(zip_path)
    events = {}
    ev = csv.DictReader(io.StringIO(z.read("df_events_v2.csv").decode("utf-8", "replace")))
    for row in ev:
        events[row["event__id"]] = row.get("event_format", "")
    names = set(z.namelist())
    agg = defaultdict(lambda: {"entries": 0, "sum_norm": 0.0})
    for eid, f in events.items():
        if f != fmt:
            continue
        path = f"events/{eid}/players_info.csv"
        if path not in names:
            continue
        rows = list(csv.DictReader(io.StringIO(z.read(path).decode("utf-8", "replace"))))
        parsed = [(r.get("player_title", "").strip(), parse_result(r.get("player_result")))
                  for r in rows]
        parsed = [(t, pr) for t, pr in parsed if t and pr]
        if len(parsed) < 2:
            continue
        field = max(up for _, (_, up) in parsed)
        if field < 2:
            continue
        for t, (lo, _up) in parsed:
            agg[t]["entries"] += 1
            agg[t]["sum_norm"] += (lo - 1) / (field - 1)
    return {t: (a["entries"], 1.0 - a["sum_norm"] / a["entries"])
            for t, a in agg.items() if a["entries"] > 0}


def gauntlet_pr(path):
    """gauntlet deck-name (lowercased) -> list of point_rates."""
    out = defaultdict(list)
    with open(path) as f:
        for row in csv.reader(f):
            if not row or row[0].startswith("#") or row[0] == "deck":
                continue
            try:
                pr = float(row[6])
            except (ValueError, IndexError):
                continue
            out[row[0].strip().lower()].append(pr)
    return out


def spearman(xs, ys):
    def ranks(v):
        order = sorted(range(len(v)), key=lambda i: v[i])
        r = [0.0] * len(v)
        i = 0
        while i < len(v):
            j = i
            while j + 1 < len(v) and v[order[j + 1]] == v[order[i]]:
                j += 1
            avg = (i + j) / 2.0 + 1.0
            for k in range(i, j + 1):
                r[order[k]] = avg
            i = j + 1
        return r
    rx, ry = ranks(xs), ranks(ys)
    n = len(xs)
    mx, my = sum(rx) / n, sum(ry) / n
    cov = sum((rx[i] - mx) * (ry[i] - my) for i in range(n))
    vx = sum((rx[i] - mx) ** 2 for i in range(n)) ** 0.5
    vy = sum((ry[i] - my) ** 2 for i in range(n)) ** 0.5
    return cov / (vx * vy) if vx and vy else 0.0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--zip", required=True)
    ap.add_argument("--format", required=True)
    ap.add_argument("--gauntlet", required=True)
    args = ap.parse_args()
    buckets = PI_BUCKETS  # only PI curated for now

    real = real_strength_by_name(args.zip, args.format)
    gaunt = gauntlet_pr(args.gauntlet)

    rows = []
    for canon, (g_subs, r_vars) in buckets.items():
        g_vals = [pr for name, prs in gaunt.items()
                  for pr in prs if any(s in name for s in g_subs)]
        r_pairs = [real[v] for v in r_vars if v in real]
        if not g_vals or not r_pairs:
            continue
        g_mean = sum(g_vals) / len(g_vals)
        r_ent = sum(e for e, _ in r_pairs)
        r_str = sum(e * s for e, s in r_pairs) / r_ent
        rows.append((canon, len(g_vals), g_mean, r_ent, r_str))

    print(f"# Gauntlet point-rate vs real MTGTop8 strength — format {args.format}")
    print(f"# {'bucket':<22} {'g_n':>4} {'gauntlet_pr':>11} {'real_n':>7} {'real_str':>9}")
    rows.sort(key=lambda t: t[2], reverse=True)
    for canon, gn, gm, rn, rs in rows:
        print(f"  {canon:<22} {gn:>4} {gm:>11.3f} {rn:>7} {rs:>9.3f}")
    if len(rows) >= 3:
        rho = spearman([r[2] for r in rows], [r[4] for r in rows])
        print(f"\n# Spearman rho (gauntlet_pr vs real_str), N={len(rows)} buckets: {rho:.3f}")
        print("# (sorted-by-gauntlet order above; a NEGATIVE/near-zero rho = the cheap")
        print("#  referee does not track real-world strength on these archetypes.)")


if __name__ == "__main__":
    main()
