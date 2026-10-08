# A2.1: material Monte Carlo against PIMC on the capsule, today's engine

Checked-in rows and readings of A2.1, run on DTU at `0420ef30`. The design,
predictions and decision rules were registered before any game, in the
planning vault's "Arcana A2.1 pre-registration (2026-10-08)", after three
reviews; this document is the D3 results document that registration
requires whatever the outcome. July's capsule documents (`docs/rl-benchmark*`)
are frozen and are not edited; the comparison with them below is
descriptive, since July's per-game outcomes were not recorded and no valid
significance test against them exists.

## Files

| File | What |
|---|---|
| `h2h-12-120.csv`, `h2h-16-160.csv`, `h2h-32-320.csv` | arm H: `vmc-material` (6 rollouts, depth 25, 10 candidates) against PIMC at each budget in the capsule mirror, games 1000 to 1215 of `pair_game`'s schedule on each of the five decks, 1,080 games per budget; outcome `a` = `vmc-material` won, `b` = PIMC won |
| `reading.txt` | `arcana-ai/scripts/a2_h2h_summary.py --first 1000 --games-per-deck 216 --budget 12/120 --budget 16/160 --budget 32/320` on the three files, with its DECISIONS line |
| `replication-deck0.txt` … `replication-deck4.txt` | arm R: `capsule_pimc_distill` with July's settings (12 games a pair, 30 training games, PIMC 12/120), one deck each |

## Arm H

`vmc-material`'s point-rate against PIMC (win 1, draw ½; there were no
draws), with 95% intervals from the 216 seed blocks:

| PIMC budget | point-rate | interval | seconds per game, mean / max |
|---|---:|---|---|
| 12/120 | 0.569 (41/72) | [0.538, 0.601] | 19.4 / 142 |
| 16/160 | 0.547 (197/360) | [0.513, 0.581] | 31.4 / 232 |
| 32/320 | 0.567 (17/30) | [0.532, 0.601] | 93.3 / 744 |

Paired differences of point-rate between budgets, over the same seed blocks:
12/120 minus 16/160 +0.022 [−0.009, +0.053]; 12/120 minus 32/320 +0.003
[−0.029, +0.035]; 16/160 minus 32/320 −0.019 [−0.052, +0.013].

`vmc-material` scored above half against PIMC at every budget tested, with
every interval above 0.5, and its score did not move with PIMC's budget in
this range: no paired difference's interval excludes zero.

Registered predictions: H1 (s_12 in [0.45, 0.62]) holds; H2a (strictly
decreasing), H2b (s_12 − s_32 ≥ 0.05) and H3 (s_32 < 0.5) fail. The 64/640
arm's condition is not met (32/320's interval excludes 0.5), so it was not
run. D1 names no PIMC referee, so `vmc-material` stays A2.2's referee with
PIMC at 32/320 as the comparison arm; D2 names PIMC 16/160 as the best tested
PIMC and A2.3's teacher, and `vmc-material` as the bar.

## Arm R, beside July's table

Aggregate point-rate over the five decks (240 games per policy), from exact
points:

| policy | today | July (`b795e0b2`) |
|---|---:|---:|
| `vmc-material` | 0.713 | 0.767 |
| `pimc` (12/120) | 0.637 | 0.629 |
| `vmc-learned-pimc` | 0.550 | 0.500 |
| `vmc-learned-rand` | 0.467 | 0.492 |
| `random` | 0.133 | 0.113 |

The panel's order is July's. `vmc-material` against PIMC within the panel:
0.583 over 60 games (July 0.65); 20 of those games (games 0 to 3 on each
deck) were played and seen in A2.0's pilot. Registered predictions: R1 (the
two learned leaves within 0.08) fails, at exactly 1/12 apart; R2
(`vmc-material` at least 0.15 above both leaves) holds, at 13/80. Every
engine change between `b795e0b2` and `0420ef30` lies between the two
columns, so no difference is attributed to any one of them.
