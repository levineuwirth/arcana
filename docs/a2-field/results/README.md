# A2.2: the field under material, PIMC and random referees, and information on the capsule

Checked-in rows and readings of A2.2, run on DTU at `5e82c4b7`. The design,
predictions and decision rules were registered before any game, in the
planning vault's "Arcana A2.2 pre-registration (2026-10-08)", frozen naming
that commit after two design reviews and two implementation reviews; this is
the results document that registration requires whatever the outcome. The
field is the 64 decks pinned in `../manifest-PI.csv`. Claims generalize to
decks like these, MTGTop8 Pioneer maindecks that today's catalog covers in
full, and not to Pioneer: the catalog's coverage selects the field.

## Files

| File | What |
|---|---|
| `F.csv`, `Fprime.csv` | arms F and F′: `vmc-material` (6 rollouts, depth 25, 10 candidates, rolling out from the true state) refereeing one seat-swapped duel per pair of the 64 decks, at base seeds 1000 and 2000 |
| `R.csv` | arm R: the same duels under the random referee, base seed 2000 |
| `P.csv` | arm P: the same duels under PIMC 16/160, base seed 2000 |
| `X.csv` | arm X: one block of four games per pair, `vmc-material` against PIMC 32/320, base seed 3000 |
| `I.csv` | arm I: true-state `vmc-material` (outcome `a`) against the same search in sampled-state mode (`b`) in the capsule mirror, games 2000 to 2215 of `pair_game`'s schedule on each of the five capsule decks |
| `reading-field.txt` | `arcana-ai/scripts/a2_field_summary.py --manifest docs/a2-field/manifest-PI.csv --F F.csv --Fprime Fprime.csv --R R.csv --P P.csv --X X.csv`, with its DECISIONS line |
| `reading-I.txt` | `arcana-ai/scripts/a2_h2h_summary.py --first 2000 --games-per-deck 216 --budget sampled I.csv`, with its DECISIONS line |

A duel row is `duel,i,j,d,a_i,a_j,b_i,b_j,seconds`: each deck's points in
game A, which seats deck i first, and in game B, which swaps the seats. A
block row is `block,i,j,d,g1,g2,g3,g4,seconds`, with `g1` to `g4` being
`vmc-material`'s points in the order of `BLOCK_GAMES`
(`arcana-ai/src/deckeval_runner.rs`). Its games 1 and 4, and its games 2 and 3,
start the same decks in the same seats from identical shuffles, with the
pilots exchanged. Seconds are for the whole duel or block.

The arms ran in shards `k/K`. Each merged file keeps one config row, with its
shard field written `0/1`, then the 64 deck rows and the 2,016 pair rows,
sorted by pair. Both readings were run once, on the run's shard files, after
every arm had passed `--check-only`. The scripts at `5e82c4b7` reproduce both
byte for byte from these files.

## X: material against PIMC 32/320 on the field

`vmc-material`'s point-rate over the 2,016 blocks (8,064 games) is
s_X = 0.547 (4415/8064). Its deck-jackknife 95% interval is [0.533, 0.562];
the fixed-field interval from the block means is [0.538, 0.557]. On the
capsule, A2.1 measured 0.567 (17/30).

X1 holds: s_X is above half, and the jackknife interval excludes 0.5. X2
holds: |s_X − 17/30| = 0.019, within 1/20. Material Monte Carlo's margin over
PIMC 32/320 carries from the capsule's five decks to this field at about the
same size, in point estimate.

This compares the two registered implementations, information included.
`vmc-material` rolls out from the true state. PIMC's worlds inherit the
game's engine RNG seed. X does not separate search from information.

## Ranking: F, F′, R and P

Each deck's point-rate over its 126 games in each arm is in
`reading-field.txt`. Spearman's ρ with average ranks, against F:

| arm | referee | ρ with F |
|---|---|---:|
| F′ | `vmc-material`, seed 2000 | 0.921 |
| P | PIMC 16/160, seed 2000 | 0.898 |
| R | random, seed 2000 | 0.749 |

The intervals come from 2,000 paired bootstrap resamples of the decks; no
resample was discarded as undefined.

- The loss against `vmc-material`'s own repeatability is
  L = ρ(F, F′) − ρ(F, P) = +0.024 [−0.024, +0.088].
- The gain over random is G = ρ(F, P) − ρ(F, R) = +0.149 [+0.046, +0.279].

F1 holds: ρ(F, P) > ρ(F, R). F2 holds: L ≤ 0.10.

E2's condition is met. L's upper bound, 0.088, is at most δ = 0.15, and G's
lower bound, 0.046, is above γ = 0. So `vmc-material` alone may referee
descriptive ranking work on comparable fields. Passing means "within the
accepted correlation loss and demonstrably above random", not equivalence.
As registered, the clearance does not extend to adaptive deck optimization,
or to claims about the decks a search selects. Those keep an independent
referee check.

The random referee alone ranks the field at 0.749 against F; July's figure
was 0.71. Much of the order `vmc-material` gives these decks is already
present when both seats play at random.

## I: information on the capsule

True-state `vmc-material`'s point-rate against the same search in
sampled-state mode is s_I = 0.503 (181/360). Its 95% interval from the 216
seed blocks is [0.467, 0.539]. There were no draws.

| deck | point-rate | interval |
|---|---:|---|
| UW Spirit Aggro | 0.542 | [0.475, 0.608] |
| Golgari Scales | 0.509 | [0.442, 0.576] |
| Hardened Scales | 0.500 | [0.433, 0.567] |
| Rakdos Aggro | 0.509 | [0.442, 0.576] |
| Red Deck Wins | 0.454 | [0.387, 0.520] |

I1 fails: the interval contains 0.5. I2 holds: s_I < 17/30.

E3's direction is "unresolved": at 6/25/10, in the capsule mirror, seeing the
true state has no effect this arm can resolve. The interval's upper end,
0.539, also lies below 17/30. That is read descriptively, as I2 is. Arm I
does not decompose s_X or A2.1's s_32: the opponents differ, and the two
effects need not add.

E3 stands whatever I shows. A2.3 evaluates each learned leaf in sampled-state
mode whenever it claims honest-play strength. An unresolved I cannot show
that a leaf is insensitive to information.

## Beside July's field

This comparison is descriptive and was registered as such. The two fields
were drawn from one dump, and their overlap cannot be reconstructed
(`../README.md`). Every engine change since July lies between them, and no
test is applied.

| | today's F, 64 decks | July, 62 decks (`docs/gauntlet-results/pi_gauntlet_62.csv`) |
|---|---|---|
| top eight | Gruul Aggro ×2, Rg Aggro, UW Spirit Aggro, Red Deck Wins ×4 | Gruul Aggro ×2, Rg Aggro, Red Deck Wins ×4, and at eighth Rhino Rock tied with a third Gruul Aggro at 0.762 |
| bottom eight | Golgari Aggro ×3, Golgari Scales ×2, Devotion to Golgari, Golgari Hardened Scales, Temur Midrange | Golgari Scales ×3, Golgari Aggro, Golgari Hardened Scales, Hardened Scales, Hardened Snakes, Temur Midrange |

Temur Midrange is last in both, at 0.087 today and 0.090 in July.

## On this engine

Two cards in the field are known to play unfaithfully at `5e82c4b7`, as
recorded small items A1.5l and A1.5m:

- Spell Queller's two triggers are no-ops. It is in 14 of the 64 decks.
- Hangarback Walker's dies trigger reads its object from the live arena, so
  it makes no Thopters. It is in 19 decks, none of which also holds Spell
  Queller.

Every arm played the same engine, so the ranking arms share these errors. No
figure here measures their effect on any deck.

## Cost

Seconds per row, from the rows' own timings (a duel is two games, a block
four), and core-hours of game time against the registration's planning
estimates:

| arm | mean / max seconds a row | core-hours | planned |
|---|---|---:|---:|
| F | 7.4 / 370.9 | 4.1 | about 4 |
| F′ | 7.6 / 112.7 | 4.2 | about 4 |
| R | 0.0 / 0.7 | 0.0 | under 1 |
| P | 121.2 / 1,675.4 | 67.9 | about 67 |
| X | 394.0 / 7,717.8 | 220.6 | about 210 |
| I | 3.5 to 5.8 a game, by deck | 1.3 | about 1 |

Seven tasks hit their time limit, and 7.1 core-hours were lost to them, for
about 305 in all against the planned 290.

X has a long tail. The sizing step's slowest block took 707 s. In the run,
three blocks took longer than an hour:

- pair 68, Sultai Control [23589_363393] against Rhino Rock, 7,718 s;
- pair 1162, Temur Midrange against Temur Aggro, 6,802 s;
- pair 1175, Temur Midrange against Red Deck Wins, 4,390 s.

That Sultai Control deck has the slowest rows on average under both PIMC
arms, and Temur Midrange the next slowest under X. A later field run at
32/320 should size its shards from a tail model, or leave more headroom than
the sizing step's maximum suggests.

## Deviations from the registered operations

- **An environment setting for every field task.** On a one-slot LSF task,
  libtest runs single-threaded and prints `test <name> ... ` before the
  test's own output, gluing it onto the first row. The first field sizing
  wave lost deck 0's row that way, and `--sizing` refused its files. From
  then on, every field and arm-I task ran with `RUST_TEST_THREADS=2` and
  `RUST_MIN_STACK=67108864` (64 MiB for the test thread libtest then
  spawns), passed through `bsub -env`. No code changed. On the login node,
  pinned to one core as a task is, at seed 9999, which no arm uses, a
  three-pair shard's duel rows were identical by checksum with and without
  the setting. The four refused sizing files were set aside, and the
  field sizing ran again at its registered seed, 4000. Arm I's sizing ran
  without the setting and passed its gate; `timing.lsf` keeps only its
  `timing,` rows, so the line it glued was the optional header.
- **Two single pairs went straight to the four-hour limit.** X's shards 68,
  487, 490 and 503 of 672 timed out at one hour, and their eight halves were
  run as registered. Three of the halves timed out again:
  - `1162/1344` and `1175/1344` each held a single pair, and were rerun alone
    at `-W 4:00`, as registered;
  - `68/1344` held two pairs. Its halves, `68/2688` and `1412/2688`, each a
    single pair, were run straight at `-W 4:00` rather than first at one
    hour.

  A row's content does not depend on its task's time limit.
