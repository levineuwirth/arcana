#!/usr/bin/env python3
"""a2_field_summary.py — A2.2's field readout, fixed before the run.

    python3 -I arcana-ai/scripts/a2_field_summary.py --manifest docs/a2-field/manifest-PI.csv \\
        [--check-only | --sizing] --F FILE... --Fprime FILE... --R FILE... --P FILE... --X FILE...

Reads the shard files field.lsf writes: per file one config row, the
playable decks as `deck,<index>,<name>`, and one row per duel
(`duel,i,j,d,a_i,a_j,b_i,b_j,seconds`, arms F, F', R and P) or per block
(`block,i,j,d,g1,g2,g3,g4,seconds`, arm X, g1..g4 being vmc-material's
points in BLOCK_GAMES order). The registration is the planning vault's
"Arcana A2.2 pre-registration (2026-10-08)".

Validation comes first and is complete before any outcome is printed. Each
arm's files must hold, per file, exactly one config row naming that arm's
registered referee, budgets, one duel or block a pair, and base seed (F at
1000; F', R and P at 2000; X at 3000), with a shard field k/K; exactly the
manifest's decks by index and name, in order; and rows that are well formed
(integer pair i < j within the manifest, d = 0, points in {0, 0.5, 1}, a
duel's two games each summing to one point, finite seconds) and belong to
their file's shard (pair index p with p mod K = k). Across an arm's files
every pair appears exactly once. Any fault prints structural diagnostics only
(file, line, arm, pair, kind of fault; never a point) and exits 2.
--check-only stops there for the arms given and prints VALID or the
diagnostics. --sizing checks the sizing step's files instead (base seed 4000,
shard 0/200, and exactly the selected pairs, those whose index is a multiple
of 200, once each, so the slowest cannot be missing) and prints only their
seconds.

The reading needs all five arms. Point estimates are exact fractions of
half-points. A deck's rate in F, F', R or P is its points over its 2(n-1)
games. Spearman's rho uses average ranks of the exact rates (ties are exact);
it is a float, and thresholds compare unrounded values. It is undefined for a
constant ranking: if any of the four observed rankings is constant, the rhos
it enters are reported as null, F1 and F2 are null, and E2 is `unresolved`,
which grants no material-only clearance. L = rho(F, F') - rho(F, P) and G =
rho(F, P) - rho(F, R) take 95% percentile intervals from 2,000 paired
bootstrap resamples of the decks (random.Random(20261009), all four of a
deck's rates drawn together, average ranks within each resample, the bounds
the 51st and 1,950th sorted values). A resample in which any of the four
vectors is constant has an undefined rho; it is discarded and another drawn,
up to 20,000 draws in all, and the number discarded is reported. If 2,000
defined resamples cannot be had, E2 is `unresolved`. s_X, vmc-material's point-rate in X, has
a deck-jackknife 95% interval (primary: each deck deleted in turn, the
jackknife variance (n-1)/n * sum (s_-k - mean)^2) and the fixed-field
interval from the block means beside it. DECISIONS gives, as JSON, X1 (s_X >
1/2 and the jackknife interval above 1/2), X2 (|s_X - 17/30| <= 1/20,
exactly), F1 (rho(F, P) > rho(F, R)), F2 (L <= 0.10) and E2 (L's upper bound
<= 0.15 and G's lower bound > 0: vmc-material alone may referee descriptive
ranking on comparable fields; otherwise both referees; `unresolved` as above).
"""

import argparse
import csv
import fractions
import json
import math
import random
import sys

F = fractions.Fraction
BUDGETS = ("vmc_rollouts: 6, vmc_depth: 25, vmc_candidates: 10, pimc_samples: {s}, "
           "pimc_cap: {c}, pimc_candidates: 10")
ARMS = {
    "F": ("duel", "VmcMaterial", (8, 80), 1000),
    "Fprime": ("duel", "VmcMaterial", (8, 80), 2000),
    "R": ("duel", "Random", (8, 80), 2000),
    "P": ("duel", "Pimc", (16, 160), 2000),
    "X": ("block", "block", (32, 320), 3000),
}
SIZING_SEED, SIZING_SHARD = 4000, (0, 200)
POINTS = {"0": 0, "0.5": 1, "1": 2}  # as half-points
BOOT, BOOT_SEED, MAX_DRAWS = 2000, 20261009, 20000


def config_prefix(arm, seed):
    _, referee, (s, c), _ = ARMS[arm]
    return f"config,{referee},RefereeBudgets {{ {BUDGETS.format(s=s, c=c)} }},1,{seed},"


def pairs_of(n):
    return [(i, j) for i in range(n) for j in range(i + 1, n)]


def read_manifest(path):
    with open(path, encoding="utf-8") as f:
        return [(int(r["index"]), r["name"]) for r in csv.DictReader(f)]


def check_arm(arm, paths, manifest, sizing=False):
    """(rows, faults) for one arm. Rows are (i, j, half-points tuple, seconds);
    a fault never carries a point."""
    kind = ARMS[arm][0]
    n = len(manifest)
    index = {p: k for k, p in enumerate(pairs_of(n))}
    seed = SIZING_SEED if sizing else ARMS[arm][3]
    prefix = config_prefix(arm, seed)
    rows, faults = [], []
    seen = {}
    for path in paths:
        configs, decks, shard = 0, [], None
        body = []
        with open(path, encoding="utf-8") as f:
            for lineno, line in enumerate(f, 1):
                where = f"{arm} {path}:{lineno}"
                line = line.rstrip("\n")
                if not line.strip():
                    continue
                if line.startswith("config,"):
                    configs += 1
                    if not line.startswith(prefix):
                        faults.append(f"{where}: config row is not the registered one for {arm}")
                        continue
                    try:
                        k, K = (int(x) for x in line[len(prefix):].split("/"))
                        if not 0 <= k < K:
                            raise ValueError
                    except ValueError:
                        faults.append(f"{where}: malformed shard field")
                        continue
                    if sizing and (k, K) != SIZING_SHARD:
                        faults.append(f"{where}: sizing shard is not 0/200")
                    shard = (k, K)
                elif line.startswith("deck,"):
                    parts = line.split(",", 2)
                    try:
                        decks.append((int(parts[1]), parts[2]))
                    except (ValueError, IndexError):
                        faults.append(f"{where}: malformed deck row")
                elif line.startswith(kind + ","):
                    body.append((where, line.split(",")))
                else:
                    faults.append(f"{where}: unrecognized line")
        if configs != 1:
            faults.append(f"{arm} {path}: {configs} config rows, not one")
        if decks != manifest:
            faults.append(f"{arm} {path}: deck rows do not match the manifest by index and name")
        for where, parts in body:
            if len(parts) != 9:
                faults.append(f"{where}: malformed row ({len(parts)} fields)")
                continue
            try:
                i, j, d = int(parts[1]), int(parts[2]), int(parts[3])
            except ValueError:
                faults.append(f"{where}: pair or index field is not an integer")
                continue
            if not 0 <= i < j < n:
                faults.append(f"{where}: pair ({i}, {j}) is not one of the manifest's")
                continue
            if d != 0:
                faults.append(f"{where}: pair ({i}, {j}): index {d}, registered one a pair")
                continue
            if any(x not in POINTS for x in parts[4:8]):
                faults.append(f"{where}: pair ({i}, {j}): a points field is not 0, 0.5 or 1")
                continue
            hp = tuple(POINTS[x] for x in parts[4:8])
            if kind == "duel" and (hp[0] + hp[1] != 2 or hp[2] + hp[3] != 2):
                faults.append(f"{where}: pair ({i}, {j}): a game's points do not sum to one")
                continue
            try:
                secs = float(parts[8])
                if not math.isfinite(secs) or secs < 0:
                    raise ValueError
            except ValueError:
                faults.append(f"{where}: pair ({i}, {j}): seconds field is not a time")
                continue
            if shard is not None and index[(i, j)] % shard[1] != shard[0]:
                faults.append(f"{where}: pair ({i}, {j}) is not in shard {shard[0]}/{shard[1]}")
                continue
            seen[(i, j)] = seen.get((i, j), 0) + 1
            rows.append((i, j, hp, secs))
    wanted = [p for k, p in enumerate(pairs_of(n)) if not sizing or k % SIZING_SHARD[1] == SIZING_SHARD[0]]
    dup = [p for p, c in sorted(seen.items()) if c > 1]
    if dup:
        faults.append(f"{arm}: {len(dup)} pairs appear more than once, first {dup[0]}")
    missing = [p for p in wanted if p not in seen]
    if missing:
        faults.append(f"{arm}: {len(missing)} of {len(wanted)} pairs missing, first {missing[0]}")
    if not rows:
        faults.append(f"{arm}: no rows")
    return rows, faults


def average_ranks(values):
    order = sorted(range(len(values)), key=lambda k: values[k])
    ranks = [0.0] * len(values)
    k = 0
    while k < len(order):
        m = k
        while m + 1 < len(order) and values[order[m + 1]] == values[order[k]]:
            m += 1
        for t in range(k, m + 1):
            ranks[order[t]] = (k + m) / 2 + 1
        k = m + 1
    return ranks


def spearman(x, y):
    rx, ry = average_ranks(x), average_ranks(y)
    mx, my = sum(rx) / len(rx), sum(ry) / len(ry)
    sxy = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    sxx = sum((a - mx) ** 2 for a in rx)
    syy = sum((b - my) ** 2 for b in ry)
    if sxx == 0 or syy == 0:
        return None
    return sxy / math.sqrt(sxx * syy)


def percentile_bounds(values):
    """The 2.5% and 97.5% bounds by integer index: of 2,000 sorted values,
    the 51st and the 1,950th."""
    s = sorted(values)
    return s[25 * len(s) // 1000], s[-(-975 * len(s) // 1000) - 1]


def deck_rates(rows, n):
    hp = [0] * n
    for i, j, (ai, aj, bi, bj), _ in rows:
        hp[i] += ai + bi
        hp[j] += aj + bj
    return [F(h, 4 * (n - 1)) for h in hp]


def ranking_figures(rates):
    f, fp, r, p = (rates[a] for a in ("F", "Fprime", "R", "P"))
    rho = {"F_Fprime": spearman(f, fp), "F_P": spearman(f, p), "F_R": spearman(f, r)}
    out = {"rho": rho, "L": None, "G": None, "L_interval": None, "G_interval": None, "discarded": 0}
    if None in rho.values():
        return out
    out["L"], out["G"] = rho["F_Fprime"] - rho["F_P"], rho["F_P"] - rho["F_R"]
    rng = random.Random(BOOT_SEED)
    n = len(f)
    ls, gs = [], []
    for _ in range(MAX_DRAWS):
        if len(ls) == BOOT:
            break
        idx = [rng.randrange(n) for _ in range(n)]
        bf = [f[k] for k in idx]
        b_ff, b_fp, b_fr = spearman(bf, [fp[k] for k in idx]), spearman(bf, [p[k] for k in idx]), \
            spearman(bf, [r[k] for k in idx])
        if None in (b_ff, b_fp, b_fr):
            out["discarded"] += 1
            continue
        ls.append(b_ff - b_fp)
        gs.append(b_fp - b_fr)
    if len(ls) == BOOT:
        out["L_interval"], out["G_interval"] = percentile_bounds(ls), percentile_bounds(gs)
    return out


def head_to_head(rows, n):
    hp = {(i, j): sum(g) for i, j, g, _ in rows}
    blocks = len(hp)
    s = F(sum(hp.values()), 8 * blocks)
    means = [v / 8 for v in hp.values()]
    m = sum(means) / blocks
    sd = math.sqrt(sum((v - m) ** 2 for v in means) / (blocks - 1))
    block_half = 1.96 * sd / math.sqrt(blocks)
    loo = []
    for k in range(n):
        kept = [v for (i, j), v in hp.items() if k not in (i, j)]
        loo.append(F(sum(kept), 8 * len(kept)))
    mean_loo = sum(loo) / n
    se = math.sqrt((n - 1) / n * sum(float(v - mean_loo) ** 2 for v in loo))
    return {"s_X": s, "jackknife": (float(s) - 1.96 * se, float(s) + 1.96 * se),
            "block": (float(s) - block_half, float(s) + block_half), "blocks": blocks}


def decisions(x, rk):
    return {
        "X1": x["s_X"] > F(1, 2) and x["jackknife"][0] > 0.5,
        "X2": abs(x["s_X"] - F(17, 30)) <= F(1, 20),
        "F1": None if rk["L"] is None else rk["rho"]["F_P"] > rk["rho"]["F_R"],
        "F2": None if rk["L"] is None else rk["L"] <= 0.10,
        "E2": "unresolved" if rk["L_interval"] is None
              else "vmc-material alone" if rk["L_interval"][1] <= 0.15 and rk["G_interval"][0] > 0
              else "both referees",
    }


def jsonable(v):
    if isinstance(v, F):
        return f"{v.numerator}/{v.denominator}"
    if isinstance(v, dict):
        return {k: jsonable(x) for k, x in v.items()}
    if isinstance(v, (list, tuple)):
        return [jsonable(x) for x in v]
    return v


def report(arms, manifest):
    n = len(manifest)
    rates = {a: deck_rates(arms[a], n) for a in ("F", "Fprime", "R", "P")}
    rk = ranking_figures(rates)
    x = head_to_head(arms["X"], n)
    for a, rows in arms.items():
        secs = [r[3] for r in rows]
        print(f"{a}: {len(rows)} rows, {sum(secs) / len(secs):.1f} s mean, {max(secs):.1f} s max a row")
    print(f"\ndeck point-rates over {2 * (n - 1)} games each: F, F', R, P")
    for k, name in manifest:
        print(f"  {k:>2} {name:<44} " + "  ".join(f"{float(rates[a][k]):.3f}" for a in ("F", "Fprime", "R", "P")))
    order = sorted(range(n), key=lambda k: (-rates["F"][k], k))
    print("\nF's ranking, top eight: " + "; ".join(manifest[k][1] for k in order[:8]))
    print("F's ranking, bottom eight: " + "; ".join(manifest[k][1] for k in order[-8:]))
    fmt = lambda v, f="{:.3f}": "undefined" if v is None else f.format(v)
    r = rk["rho"]
    print(f"\nSpearman (average ranks): rho(F, F') {fmt(r['F_Fprime'])}, rho(F, P) {fmt(r['F_P'])},"
          f" rho(F, R) {fmt(r['F_R'])}")
    for name, key, label in (("L", "L", "rho(F, F') - rho(F, P)"), ("G", "G", "rho(F, P) - rho(F, R)")):
        iv = rk[key + "_interval"]
        bounds = "no interval" if iv is None else f"[{iv[0]:+.3f}, {iv[1]:+.3f}]"
        print(f"{name} = {label} {fmt(rk[key], '{:+.3f}')} {bounds}")
    print(f"bootstrap resamples discarded as undefined: {rk['discarded']}")
    print(f"\ns_X, vmc-material against PIMC 32/320 over {x['blocks']} blocks: {float(x['s_X']):.3f}"
          f" jackknife [{x['jackknife'][0]:.3f}, {x['jackknife'][1]:.3f}]"
          f" block [{x['block'][0]:.3f}, {x['block'][1]:.3f}]")
    flags = decisions(x, rk)
    print("\nDECISIONS " + json.dumps(jsonable({"flags": flags, "figures": {"ranking": rk, "X": x,
          "rates": {a: rates[a] for a in rates}}}), sort_keys=True))
    return flags


def main(argv=None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--manifest", required=True)
    mode = ap.add_mutually_exclusive_group()
    mode.add_argument("--check-only", action="store_true")
    mode.add_argument("--sizing", action="store_true")
    for a in ARMS:
        ap.add_argument(f"--{a}", nargs="+", default=None, metavar="FILE")
    args = ap.parse_args(argv)
    manifest = read_manifest(args.manifest)
    given = {a: getattr(args, a) for a in ARMS if getattr(args, a)}
    faults = []
    if not given:
        faults.append("no arm given")
    if not (args.check_only or args.sizing):
        faults += [f"{a}: the reading needs every arm" for a in ARMS if a not in given]
    arms = {}
    for a, paths in given.items():
        rows, fs = check_arm(a, paths, manifest, sizing=args.sizing)
        arms[a] = rows
        faults += fs
    if faults:
        print(f"INVALID: {len(faults)} structural fault(s); no outcome is reported")
        for f in faults[:40]:
            print(f"  {f}")
        if len(faults) > 40:
            print(f"  ... and {len(faults) - 40} more")
        return 2
    if args.sizing:
        for a, rows in arms.items():
            secs = [r[3] for r in rows]
            print(f"SIZING {a}: {len(rows)} rows, seconds mean {sum(secs) / len(secs):.1f}, max {max(secs):.1f}")
        return 0
    if args.check_only:
        print("VALID: " + ", ".join(f"{a} {len(rows)} rows" for a, rows in arms.items()))
        return 0
    report(arms, manifest)
    return 0


if __name__ == "__main__":
    sys.exit(main())
