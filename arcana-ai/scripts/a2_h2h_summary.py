#!/usr/bin/env python3
"""a2_h2h_summary.py — A2.1's head-to-head readout, fixed before the run.

    python3 -I arcana-ai/scripts/a2_h2h_summary.py --first 1000 --games-per-deck 216 \\
        --budget 12/120 --budget 16/160 --budget 32/320 [--check-only] FILE...

Reads capsule_timing rows (with or without the leading `timing,`; the one
header line `deck,pairing,game,outcome,seconds` is allowed, any other line
starting with `deck` is a fault): deck, pairing, game, outcome, seconds;
outcome `a` means vmc-material won, `b` its opponent, `draw` neither.

Validation comes first and is complete before any outcome is printed. The
input must hold, for every requested budget, exactly the five capsule decks
in EXPECTED_DECKS, each with games first .. first + N - 1 once and nothing
else, in pairing `vmc-material vs pimc-<budget>`, every row well formed. Any
fault prints structural diagnostics only (deck, budget, game index, kind of
fault; never an outcome) and exits 2. --check-only stops there: it prints
VALID or the diagnostics and never an outcome, for checking a wave's
completeness before the registered reading.

The readout, for a valid input. pair_game seeds game g with g and its
policies with 2g + 1 and 2g + 2 on every deck and budget, so game g's five
deck outcomes form a seed block, and block g is matched across budgets.
For each budget: vmc-material's point-rate (win 1, draw 1/2), its 95%
interval from the seed blocks (the mean of the N block means, ± 1.96·sd/√N
of the block means), the same per deck (games within a deck share no seed,
so its interval is from its games), win/draw/loss counts, and seconds per
game. For each pair of budgets in the order given: the mean paired
difference of block means with its 95% interval from the N paired
differences. Point estimates and differences are exact fractions of
integer half-point totals (a win is two half-points, a draw one), so a
threshold such as a drop of 0.05 is met exactly when it is; the seed blocks
supply only the intervals' half-widths, centred on those exact values. The
figures print to three decimals for reading; every threshold is applied to
the unrounded values, which a final DECISIONS line gives as JSON (exact
fractions as "num/den", floats at full precision) together with the
registered flags when the budgets are 12/120, 16/160 and 32/320, optionally
then 64/640: H1, H2a, H2b, H3, the 64/640 condition, D1 and D2. Only D2's
tie rule rounds, to three decimals (half to even, on the exact rates). The decisions themselves are registered in the planning
vault, "Arcana A2.1 pre-registration (2026-10-08)".
"""

import argparse
import collections
import fractions
import json
import math
import sys

EXPECTED_DECKS = (
    "UW Spirit Aggro [23729_364730]",
    "Golgari Scales [23743_364886]",
    "Hardened Scales [23846_365831]",
    "Rakdos Aggro [24126_368389]",
    "Red Deck Wins [24148_368536]",
)
POINTS = {"a": 1.0, "draw": 0.5, "b": 0.0}
HALF_POINTS = {"a": 2, "draw": 1, "b": 0}
HEADER = ["deck", "pairing", "game", "outcome", "seconds"]
REGISTERED = ["12/120", "16/160", "32/320"]


def parse(paths):
    """Rows and structural faults; a fault never carries an outcome."""
    rows, faults = [], []
    for path in paths:
        with open(path, encoding="utf-8") as f:
            for lineno, line in enumerate(f, 1):
                where = f"{path}:{lineno}"
                line = line.rstrip("\n")
                if not line.strip():
                    continue
                parts = line.split(",")
                if parts[0] == "timing":
                    parts = parts[1:]
                if parts == HEADER:
                    continue
                if parts[:1] == ["deck"]:
                    faults.append(f"{where}: malformed header")
                    continue
                if len(parts) != 5:
                    faults.append(f"{where}: malformed row ({len(parts)} fields)")
                    continue
                deck, pairing, game, outcome, secs = parts
                try:
                    g = int(game)
                except ValueError:
                    faults.append(f"{where}: game index is not an integer")
                    continue
                if outcome not in POINTS:
                    faults.append(f"{where}: unrecognized outcome field")
                    continue
                try:
                    s = float(secs)
                    if not math.isfinite(s) or s < 0:
                        raise ValueError
                except ValueError:
                    faults.append(f"{where}: seconds field is not a time")
                    continue
                rows.append((deck, pairing, g, outcome, s))
    return rows, faults


def validate(rows, faults, budgets, first, n):
    """All structural faults of the input against the registered design."""
    faults = list(faults)
    if not rows:
        faults.append("no rows")
    wanted = {f"vmc-material vs pimc-{b}": b for b in budgets}
    seen = collections.Counter()
    for deck, pairing, g, _, _ in rows:
        if pairing not in wanted:
            faults.append(f"unexpected pairing {pairing!r}")
            continue
        if deck not in EXPECTED_DECKS:
            faults.append(f"unexpected deck {deck!r} ({wanted[pairing]})")
            continue
        if not first <= g < first + n:
            faults.append(f"{deck} ({wanted[pairing]}): game {g} outside {first}..{first + n - 1}")
            continue
        seen[(pairing, deck, g)] += 1
    for (pairing, deck, g), k in sorted(seen.items()):
        if k > 1:
            faults.append(f"{deck} ({wanted[pairing]}): game {g} appears {k} times")
    for pairing, b in wanted.items():
        for deck in EXPECTED_DECKS:
            missing = [g for g in range(first, first + n) if (pairing, deck, g) not in seen]
            if missing:
                faults.append(f"{deck} ({b}): {len(missing)} of {n} games missing, first {missing[0]}")
    return faults


def half_width(values):
    """1.96 standard errors of the mean of `values` (sample sd)."""
    if len(values) < 2:
        return 0.0
    m = sum(values) / len(values)
    sd = math.sqrt(sum((v - m) ** 2 for v in values) / (len(values) - 1))
    return 1.96 * sd / math.sqrt(len(values))


def cost(budget):
    samples, cap = budget.split("/")
    return int(samples) * int(cap)


def exact(x):
    return f"{x.numerator}/{x.denominator}"


def statistics_of(rows, budgets, first, n):
    """Exact point estimates from integer half-point totals; interval
    half-widths from the games (per deck) or the seed blocks (pooled and
    paired). Exact values are Fractions under keys ending in `_exact`."""
    table = collections.defaultdict(dict)
    for deck, pairing, g, outcome, secs in rows:
        table[pairing.removeprefix("vmc-material vs pimc-")][(deck, g)] = (outcome, secs)
    games = range(first, first + n)
    k = len(EXPECTED_DECKS)
    out = {"budgets": {}, "differences": {}}
    blocks = {}
    for b in budgets:
        cells = table[b]
        decks = {}
        for deck in EXPECTED_DECKS:
            hp = [HALF_POINTS[cells[(deck, g)][0]] for g in games]
            r = fractions.Fraction(sum(hp), 2 * n)
            half = half_width([h / 2 for h in hp])
            secs = [cells[(deck, g)][1] for g in games]
            c = collections.Counter(cells[(deck, g)][0] for g in games)
            decks[deck] = {"rate_exact": r, "rate": float(r), "lo": float(r) - half, "hi": float(r) + half,
                           "wdl": [c["a"], c["draw"], c["b"]], "mean_s": sum(secs) / n, "max_s": max(secs)}
        block_hp = [sum(HALF_POINTS[cells[(deck, g)][0]] for deck in EXPECTED_DECKS) for g in games]
        blocks[b] = block_hp
        r = fractions.Fraction(sum(block_hp), 2 * k * n)
        half = half_width([h / (2 * k) for h in block_hp])
        c = collections.Counter(v[0] for v in cells.values())
        out["budgets"][b] = {"rate_exact": r, "rate": float(r), "lo": float(r) - half, "hi": float(r) + half,
                             "games": len(cells), "wdl": [c["a"], c["draw"], c["b"]], "decks": decks}
    for i, b1 in enumerate(budgets):
        for b2 in budgets[i + 1:]:
            diff_hp = [x - y for x, y in zip(blocks[b1], blocks[b2])]
            m = fractions.Fraction(sum(diff_hp), 2 * k * n)
            half = half_width([h / (2 * k) for h in diff_hp])
            out["differences"][f"{b1} - {b2}"] = {"mean_exact": m, "mean": float(m),
                                                  "lo": float(m) - half, "hi": float(m) + half}
    return out


def decisions(st, budgets):
    """The registered flags; None unless the budgets are 12/120, 16/160 and
    32/320, optionally followed by 64/640. Rates and differences compare as
    exact fractions; interval endpoints are floats."""
    if budgets[:3] != REGISTERED or budgets[3:] not in ([], ["64/640"]):
        return None
    F = fractions.Fraction
    r = {b: st["budgets"][b] for b in budgets}
    s12, s16, s32 = (r[b]["rate_exact"] for b in REGISTERED)
    d = st["differences"]
    by_cost = sorted(budgets, key=cost)
    d1 = next((b for b in by_cost if r[b]["hi"] < 0.5), None)
    best = min(by_cost, key=lambda b: (round(r[b]["rate_exact"], 3), cost(b)))
    return {
        "H1": F(45, 100) <= s12 <= F(62, 100),
        "H2a": s12 > s16 > s32,
        "H2b": d["12/120 - 32/320"]["mean_exact"] >= F(5, 100),
        "H3": s32 < F(1, 2),
        "H3_interval_below_half": r["32/320"]["hi"] < 0.5,
        "run_64_640": r["32/320"]["lo"] <= 0.5 <= r["32/320"]["hi"]
                      and d["16/160 - 32/320"]["mean_exact"] >= F(3, 100),
        "D1_referee": d1,
        "D2_best_pimc": best,
        "D2_bar": best if r[best]["rate_exact"] < F(1, 2) else "vmc-material",
        "D2_teacher": d1 if d1 is not None else best,
    }


def jsonable(x):
    if isinstance(x, fractions.Fraction):
        return exact(x)
    if isinstance(x, dict):
        return {k: jsonable(v) for k, v in x.items()}
    if isinstance(x, list):
        return [jsonable(v) for v in x]
    return x


def report(rows, budgets, first, n):
    st = statistics_of(rows, budgets, first, n)
    for b in budgets:
        x = st["budgets"][b]
        print(f"\nvmc-material vs pimc-{b}: point-rate of vmc-material, {len(EXPECTED_DECKS)} decks x {n} games")
        for deck, y in x["decks"].items():
            w, dr, l = y["wdl"]
            print(f"  {deck:<32} point-rate {y['rate']:.3f} [{y['lo']:.3f}, {y['hi']:.3f}]  w/d/l {w}/{dr}/{l}"
                  f"  {y['mean_s']:.1f} s/game, max {y['max_s']:.1f}")
        w, dr, l = x["wdl"]
        print(f"  {'ALL (seed blocks)':<32} point-rate {x['rate']:.3f} [{x['lo']:.3f}, {x['hi']:.3f}]"
              f"  w/d/l {w}/{dr}/{l}  n={x['games']}")
    for k, y in st["differences"].items():
        b1, b2 = k.split(" - ")
        print(f"\npaired difference, point-rate at {b1} minus at {b2}: {y['mean']:+.3f} [{y['lo']:+.3f}, {y['hi']:+.3f}]"
              f" over {n} seed blocks")
    flags = decisions(st, budgets)
    print("\nDECISIONS " + json.dumps(jsonable({"flags": flags, "figures": st}), sort_keys=True))
    return st, flags


def main(argv=None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--first", type=int, required=True)
    ap.add_argument("--games-per-deck", type=int, required=True)
    ap.add_argument("--budget", action="append", required=True)
    ap.add_argument("--check-only", action="store_true")
    ap.add_argument("files", nargs="+")
    args = ap.parse_args(argv)
    if args.games_per_deck < 2:
        print("INVALID: --games-per-deck must be at least 2")
        return 2
    rows, faults = parse(args.files)
    faults = validate(rows, faults, args.budget, args.first, args.games_per_deck)
    if faults:
        print(f"INVALID: {len(faults)} structural fault(s); no outcome is reported")
        for f in faults[:40]:
            print(f"  {f}")
        if len(faults) > 40:
            print(f"  ... and {len(faults) - 40} more")
        return 2
    if args.check_only:
        print(f"VALID: {len(rows)} rows, budgets {', '.join(args.budget)}, games {args.first}"
              f"..{args.first + args.games_per_deck - 1} on {len(EXPECTED_DECKS)} decks")
        return 0
    report(rows, args.budget, args.first, args.games_per_deck)
    return 0


if __name__ == "__main__":
    sys.exit(main())
