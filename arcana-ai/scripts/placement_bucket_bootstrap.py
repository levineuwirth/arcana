#!/usr/bin/env python3
"""Event-bootstrap MTGTop8 recorded-conversion buckets used for validation.

This reports uncertainty for the external validation proxy used by
placement_vs_gauntlet.py. The bootstrap unit is the event, not the recorded deck
row, because rows from the same top-8 bracket are correlated.
"""
import argparse
import csv
import io
import random
import zipfile
from collections import defaultdict

from placement_vs_gauntlet import PI_BUCKETS, parse_result


def percentile(xs, q):
    if not xs:
        return float("nan")
    xs = sorted(xs)
    pos = (len(xs) - 1) * q
    lo = int(pos)
    hi = min(lo + 1, len(xs) - 1)
    frac = pos - lo
    return xs[lo] * (1.0 - frac) + xs[hi] * frac


def bucket_events(zip_path, fmt, min_stars):
    z = zipfile.ZipFile(zip_path)
    events = {}
    ev = csv.DictReader(io.StringIO(z.read("df_events_v2.csv").decode("utf-8", "replace")))
    for row in ev:
        try:
            stars = int(row.get("event_stars") or 0)
        except ValueError:
            stars = 0
        events[row["event__id"]] = (row.get("event_format", ""), stars)

    title_to_bucket = {}
    for bucket, (_, titles) in PI_BUCKETS.items():
        for title in titles:
            title_to_bucket[title] = bucket

    names = set(z.namelist())
    out = []
    field_hist = defaultdict(int)
    for eid, (event_fmt, stars) in events.items():
        if event_fmt != fmt or stars < min_stars:
            continue
        path = f"events/{eid}/players_info.csv"
        if path not in names:
            continue
        rows = list(csv.DictReader(io.StringIO(z.read(path).decode("utf-8", "replace"))))
        parsed = []
        for row in rows:
            title = (row.get("player_title") or "").strip()
            pr = parse_result(row.get("player_result"))
            if title and pr:
                parsed.append((title, pr))
        if len(parsed) < 2:
            continue
        field = max(up for _, (_, up) in parsed)
        if field < 2:
            continue
        field_hist[field] += 1
        per_bucket = defaultdict(lambda: [0, 0.0, 0])
        for title, (lo, _up) in parsed:
            bucket = title_to_bucket.get(title)
            if bucket is None:
                continue
            per_bucket[bucket][0] += 1
            per_bucket[bucket][1] += (lo - 1) / (field - 1)
            if lo == 1:
                per_bucket[bucket][2] += 1
        out.append(dict(per_bucket))
    return out, field_hist


def add_stats(dst, event_stats):
    for bucket, (entries, sum_norm, wins) in event_stats.items():
        cur = dst[bucket]
        cur[0] += entries
        cur[1] += sum_norm
        cur[2] += wins


def rates(stats):
    entries, sum_norm, wins = stats
    if entries == 0:
        return None
    return 1.0 - sum_norm / entries, wins / entries


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--zip", required=True)
    ap.add_argument("--format", required=True)
    ap.add_argument("--min-stars", type=int, default=0)
    ap.add_argument("--samples", type=int, default=2000)
    ap.add_argument("--seed", type=int, default=1)
    args = ap.parse_args()
    if args.format != "PI":
        raise SystemExit("Only the curated Pioneer (PI) bucket map is currently defined.")

    events, field_hist = bucket_events(args.zip, args.format, args.min_stars)
    point = defaultdict(lambda: [0, 0.0, 0])
    for ev in events:
        add_stats(point, ev)

    rng = random.Random(args.seed)
    boot = defaultdict(lambda: [[], []])
    for _ in range(args.samples):
        acc = defaultdict(lambda: [0, 0.0, 0])
        for _ in range(len(events)):
            add_stats(acc, rng.choice(events))
        for bucket, stats in acc.items():
            r = rates(stats)
            if r is None:
                continue
            boot[bucket][0].append(r[0])
            boot[bucket][1].append(r[1])

    hist = ", ".join(f"{k}:{field_hist[k]}" for k in sorted(field_hist))
    print(f"# MTGTop8 recorded-conversion event bootstrap — format {args.format} "
          f"(min_stars={args.min_stars}, samples={args.samples}, seed={args.seed})")
    print(f"# events used: {len(events)}")
    print(f"# recorded field-size histogram: {hist}")
    print("# CI unit: event bootstrap. Metrics are top-finish-censored conversion proxies,")
    print("# not full-field archetype strength or top-8 probability.")
    print(f"# {'bucket':<22} {'entries':>7} {'recorded':>9} {'rec_lo':>9} {'rec_hi':>9} "
          f"{'event_win':>10} {'win_lo':>9} {'win_hi':>9}")
    for bucket in sorted(point, key=lambda b: rates(point[b])[0], reverse=True):
        recorded, win = rates(point[bucket])
        rec_bs, win_bs = boot[bucket]
        print(f"  {bucket:<22} {point[bucket][0]:>7} {recorded:>9.3f} "
              f"{percentile(rec_bs, 0.025):>9.3f} {percentile(rec_bs, 0.975):>9.3f} "
              f"{win:>10.3f} {percentile(win_bs, 0.025):>9.3f} "
              f"{percentile(win_bs, 0.975):>9.3f}")


if __name__ == "__main__":
    main()
