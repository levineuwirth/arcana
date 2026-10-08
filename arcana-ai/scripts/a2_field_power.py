#!/usr/bin/env python3
"""a2_field_power.py — planning model for A2.2's referee comparison (E2).

    uv run --no-project --with numpy --with scipy python -I arcana-ai/scripts/a2_field_power.py check
    uv run --no-project --with numpy --with scipy python -I arcana-ai/scripts/a2_field_power.py review
    uv run --no-project --with numpy --with scipy python -I arcana-ai/scripts/a2_field_power.py e2 [--shared S]

A model, not a measurement: its figures are planning estimates for the A2.2
registration and depend on every assumption below.

The model. 64 decks. Each referee sees a latent deck strength with sd 0.18,
the spread of July's 62-deck vmc-material gauntlet
(docs/gauntlet-results/pi_gauntlet_62.csv: point-rate sd 0.185, mean block
standard error 0.043 at 122 games a deck). A deck's observed rate is its
latent strength plus noise of sd 0.043 * sqrt(126 / games), 126 games being
one seat-swapped duel against each of the 63 others. Rates are ranked and
Spearman's rho is Pearson's on the ranks. Observed rates are continuous and
never tie; a bootstrap resample's duplicated decks do.

`check` asserts the tie handling on synthetic vectors: identical tied rows
correlate 1, reversed ones -1, and random tied rows match scipy's spearmanr.

`review` is historical. It reproduces the table in the planning vault's
"Arcana A2.2 pre-registration review (2026-10-08)" exactly (seed 7), as run
then: how often a 95% deck-bootstrap interval of rho(F, P) lies wholly above
0.7, for P played over all pairs, half, or a quarter. It breaks a bootstrap
duplicate's tie by position, which is not average ranks.

`e2` prices the anchored rule. F is vmc-material at seed A; F', P and the
random referee R are at seed B, so each comparison with F crosses the same
seed boundary. F' shares F's latent (same referee, independent noise); P's
latent correlates with F's at `rho_p`; R's at `--rho-r` (0.78 by default,
chosen so the mean observed rho(F, R) lands near July's 0.712 on a 64-deck
field, docs/gauntlet-results/referee-sensitivity.txt). `--shared S` makes a
fraction S of the noise variance common to the three seed-B arms (shared
shuffles); it cannot touch any arm's correlation with F, only the covariance
of the differences. With L = rho(F, F') - rho(F, P), the loss against the
repeatability reference, and G = rho(F, P) - rho(F, R), the gain over random,
a paired deck bootstrap gives L's 97.5% and G's 2.5% percentiles, and the
rule passes when L_hi <= delta and G_lo > gamma. Each resample is ranked with
exact average ranks, as the registered readout ranks. A first version broke
ties with independent jitter, which is not average ranks (identical tied rows
correlate 1 under average ranks and well below it under jitter); its four
normal draws per replication are still made and discarded, so the draw
schedule, and every figure that does not depend on tie handling, is unchanged.
"""
import argparse

import numpy as np
from scipy.stats import rankdata, spearmanr

N_DECKS = 64
SPREAD = 0.18
SE_126 = 0.043


def rank(x):
    o = np.argsort(x, kind="stable")
    r = np.empty(len(x))
    r[o] = np.arange(len(x))
    return r


def spear(a, b):
    ra, rb = rank(a), rank(b)
    return np.corrcoef(ra, rb)[0, 1]


def review(reps=400, boot=400):
    # Kept exactly as run for the review: the same seed and draw order.
    rng = np.random.default_rng(7)

    def run(rho_lat, games_p, n=N_DECKS):
        obs, lo, tr = [], [], []
        for _ in range(reps):
            z = rng.standard_normal((n, 2))
            f = SPREAD * z[:, 0]
            p = SPREAD * (rho_lat * z[:, 0] + np.sqrt(1 - rho_lat**2) * z[:, 1])
            tr.append(spear(f, p))
            fo = f + SE_126 * rng.standard_normal(n)
            po = p + SE_126 * np.sqrt(126 / games_p) * rng.standard_normal(n)
            obs.append(spear(fo, po))
            bs = []
            for _ in range(boot):
                i = rng.integers(0, n, n)
                bs.append(spear(fo[i], po[i]))
            lo.append(np.percentile(bs, 2.5))
        lo = np.array(lo)
        return np.mean(tr), np.mean(obs), np.mean(lo), np.mean(lo > 0.7)

    print("latent  P-games  true-rho  obs-rho  mean-lower  P(lower>0.7)")
    for rl in (0.75, 0.85, 0.9, 0.95):
        for g in (126, 63, 32):
            t, o, l, pp = run(rl, g)
            print(f"{rl:5.2f}  {g:6d}   {t:6.3f}   {o:6.3f}   {l:6.3f}     {pp:5.2f}")


def row_spearman(a, b):
    """Spearman per row of two (B, n) arrays, with average ranks for ties."""
    ra = rankdata(a, axis=1)
    rb = rankdata(b, axis=1)
    ra -= ra.mean(axis=1, keepdims=True)
    rb -= rb.mean(axis=1, keepdims=True)
    return (ra * rb).sum(axis=1) / np.sqrt((ra * ra).sum(axis=1) * (rb * rb).sum(axis=1))


def e2(reps, boot, rho_r, shared, deltas, gammas, seed):
    rng = np.random.default_rng(seed)
    n = N_DECKS
    print(f"# e2 model: reps={reps} boot={boot} rho_r={rho_r} shared={shared} seed={seed}")
    head = "rho_p  true(F,P)  obs F,F'  obs F,P  obs F,R  mean L_hi  mean G_lo"
    rules = [(d, g) for d in deltas for g in gammas]
    head += "".join(f"  d{d:.2f}/g{g:.2f}" for d, g in rules)
    print(head)
    for rho_p in (0.75, 0.85, 0.90, 0.95, 1.00):
        acc = {k: [] for k in ("t", "ff", "fp", "fr", "lhi", "glo")}
        passes = {r: 0 for r in rules}
        for _ in range(reps):
            z = rng.standard_normal((n, 3))
            f = SPREAD * z[:, 0]
            p = SPREAD * (rho_p * z[:, 0] + np.sqrt(max(0.0, 1 - rho_p**2)) * z[:, 1])
            r = SPREAD * (rho_r * z[:, 0] + np.sqrt(1 - rho_r**2) * z[:, 2])
            acc["t"].append(spear(f, p) if rho_p < 1 else 1.0)
            common = rng.standard_normal(n)
            def noise_b():
                return SE_126 * (np.sqrt(shared) * common + np.sqrt(1 - shared) * rng.standard_normal(n))
            fo = f + SE_126 * rng.standard_normal(n)
            f2o, po, ro = f + noise_b(), p + noise_b(), r + noise_b()
            idx = rng.integers(0, n, (boot, n))
            for _ in range(4):
                rng.standard_normal((boot, n))  # the first version's jitter draws, discarded
            bf, bf2, bp, br = fo[idx], f2o[idx], po[idx], ro[idx]
            s_ff, s_fp, s_fr = row_spearman(bf, bf2), row_spearman(bf, bp), row_spearman(bf, br)
            lhi = np.percentile(s_ff - s_fp, 97.5)
            glo = np.percentile(s_fp - s_fr, 2.5)
            acc["ff"].append(spear(fo, f2o))
            acc["fp"].append(spear(fo, po))
            acc["fr"].append(spear(fo, ro))
            acc["lhi"].append(lhi)
            acc["glo"].append(glo)
            for d, g in rules:
                passes[(d, g)] += (lhi <= d) and (glo > g)
        line = f"{rho_p:5.2f}  {np.mean(acc['t']):9.3f}  {np.mean(acc['ff']):8.3f}  {np.mean(acc['fp']):7.3f}"
        line += f"  {np.mean(acc['fr']):7.3f}  {np.mean(acc['lhi']):9.3f}  {np.mean(acc['glo']):9.3f}"
        line += "".join(f"  {passes[r] / reps:11.2f}" for r in rules)
        print(line)


def check():
    t = np.array([[0.1, 0.1, 0.3, 0.5, 0.5, 0.9]])
    assert abs(row_spearman(t, t)[0] - 1.0) < 1e-12
    assert abs(row_spearman(t, -t)[0] + 1.0) < 1e-12
    rng = np.random.default_rng(3)
    for _ in range(200):
        a = rng.integers(0, 6, (1, 20)).astype(float)
        b = a + rng.integers(0, 3, (1, 20))
        assert abs(row_spearman(a, b)[0] - spearmanr(a[0], b[0]).statistic) < 1e-12
    print("tie handling: OK")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="mode", required=True)
    sub.add_parser("check")
    sub.add_parser("review")
    e = sub.add_parser("e2")
    e.add_argument("--reps", type=int, default=400)
    e.add_argument("--boot", type=int, default=1000)
    e.add_argument("--rho-r", type=float, default=0.78)
    e.add_argument("--shared", type=float, default=0.0)
    e.add_argument("--delta", type=float, action="append")
    e.add_argument("--gamma", type=float, action="append")
    e.add_argument("--seed", type=int, default=11)
    a = ap.parse_args()
    if a.mode == "check":
        check()
    elif a.mode == "review":
        review()
    else:
        e2(a.reps, a.boot, a.rho_r, a.shared, a.delta or [0.05, 0.10, 0.15], a.gamma or [0.0, 0.05], a.seed)


if __name__ == "__main__":
    main()
