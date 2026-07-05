# Controlled RL benchmark — Magic play-strength on a fixed capsule

**Status: live (experiments 1–3 done).** The internally-valid successor to the
deckbuilding-as-evaluation arc. That arc tried to make deck rankings track the
*real MTGTop8 metagame*; a review + a same-referee control
([`gauntlet-results/control-arm-PI.txt`](gauntlet-results/control-arm-PI.txt))
showed that axis is blocked by a top-8-**censored** target, N=4 buckets, and a
narrow field — none cheap to fix. This benchmark drops "match reality" and
measures what we *can* control: **how strong is a policy on a fixed,
source-auditable deck capsule**, and **can an expensive search (PIMC) be
distilled into a cheap value leaf**.

## Benchmark spec

- **Domain:** the fixed Pioneer capsule (`docs/capsule-pioneer/seeds/`) — five
  real archetype decks (UW Spirit Aggro, Golgari Scales, Hardened Scales, Rakdos
  Aggro, Red Deck Wins) whose file stems are Kaggle source ids, so every result
  row is source-auditable.
- **Metric:** policy strength = point-rate (win 1, draw ½) in a **mirror**
  round-robin — for each capsule deck every contestant plays that *same* deck, so
  only the policy varies. Averaged across the five archetypes for a deck-robust
  score.
- **Held-out-opponent discipline:** a learned leaf is trained on self-play data
  from a *teacher* policy; its strength is read primarily **vs a different
  opponent** it was not trained against, so we never score a policy on the very
  distribution it was fit to (the Goodhart trap the capsule A/B exposed).
- **Code:** `arcana-ai/src/benchmark.rs` (`run_capsule_yardstick`,
  `capsule_pimc_distill`). Reuses the parked RL pipeline (`learn.rs`,
  `observation.rs` 123-feature encoder, `search.rs` policies).

**Revive check (post-engine-changes):** the parked value pipeline still runs and
the known baseline reproduces on a generic mirror deck —
`pimc 24 > vmc-material 17 > vmc-learned-from-random 14 > random 4`.

## Experiment 1 — distill PIMC's distribution into a cheap leaf

**Question.** At equal data budget, does training a `ValueMc` leaf on **PIMC
self-play** (strong, on-distribution states) beat training it on **random
self-play** (the parked pipeline's only data source)? Both leaves get 30
self-play games *per capsule deck*; everything else is identical. Panel: random,
`vmc-material`, `vmc-learned-rand`, `vmc-learned-pimc`, `pimc` (12/120). 5 decks ×
12 games/pair = 240 games/policy. → [`rl-benchmark/capsule-distill.txt`](rl-benchmark/capsule-distill.txt)

**Result (mean point-rate across the 5 capsule decks):**

| policy | mean pr | per-deck |
|---|---:|---|
| vmc-material | **0.767** | 0.67, 0.77, 0.85, 0.79, 0.75 |
| pimc (12/120) | 0.629 | 0.75, 0.54, 0.60, 0.56, 0.69 |
| vmc-learned-pimc | 0.500 | 0.54, 0.52, 0.42, 0.62, 0.40 |
| vmc-learned-rand | 0.492 | 0.44, 0.52, 0.52, 0.35, 0.62 |
| random | 0.113 | 0.10, 0.15, 0.10, 0.17, 0.04 |

**Two findings.**

1. **The hypothesis is a clean NULL.** `learned-pimc` (0.500) ≈ `learned-rand`
   (0.492); their head-to-head is exactly **0.500** (per deck 0.67/0.50/0.33/0.75/0.25
   — high variance, centered on 0.5). **Distilling PIMC's distribution did not beat
   random self-play as a cheap leaf** at this budget. Both learned leaves beat
   random (0.11) but lose to hand-tuned material (0.77) and PIMC (0.63) — i.e. the
   parked "learned value is a worse leaf than material" result reproduces on the
   capsule, and PIMC-distribution training does not rescue it.

2. **A genuine internally-valid finding:** on the capsule, cheap
   **`ValueMc(material)` is the *strongest* policy (0.767), beating PIMC 12/120
   (0.629)** — the opposite of the generic-deck baseline (where PIMC dominates).
   The likely cause is exactly the deckbuild arc's theme: the capsule is
   aggro/tempo-leaning, and a material-in-search leaf pilots aggressive decks very
   well, while PIMC at a bounded budget pays its strategy-fusion cost on longer
   real-archetype games. A stronger PIMC budget would likely narrow this.

**Held-out reads** (leaf vs a non-teacher opponent): `learned-rand` vs PIMC 0.384,
vs material 0.300; `learned-pimc` vs material 0.266. Both learned leaves are weak
against the strong opponents — consistent with the data-limited caveat.

**Caveats.**
- **Data-limited leaves.** 30 self-play games/leaf; the parked learning-curve
  diagnostic suggested ~150 to beat material's log-loss. So the null is "at 30
  games the *distribution source* doesn't matter," not "PIMC distillation can
  never help." Testing at 150 PIMC games/deck is ~5× the PIMC self-play cost.
- **Bounded PIMC.** 12/120; a higher budget could reorder pimc vs vmc-material.
- **Screen-tier per cell.** 12 games/pair/deck (48/deck, 240 aggregated); good for
  the wide splits (material/random), noisy for close pairs (the two learned leaves).
- **Per-deck training.** Each leaf is trained on its own deck's mirror self-play,
  so "learned" here means archetype-specialized, not a single general value.

## Experiment 2 — card-identity ablation (representation)

**Question.** Experiment 1 isolated the null to the value *representation* (a
linear MC-outcome model over 123 identity-**free** aggregates can't see synergy).
Does adding **card-identity features** rescue the learned leaf? Two `ValueMc`
leaves trained from the **same random self-play at the same seed** — the *only*
difference is the encoder: `basic` (123) vs `id` (123 + 57-card capsule vocabulary
× 5 visible zones = 408). **Pre-registered** (before the run): identity should
lift the leaf, especially on synergy decks (Scales/Spirits), and improve held-out
vs material/pimc; if it stays ~0.5, identity alone is not the lever and the next
move is dense target quality. → [`rl-benchmark/capsule-identity.txt`](rl-benchmark/capsule-identity.txt)

**Result (mean point-rate across the 5 decks):**

| policy | mean pr |
|---|---:|
| vmc-material | **0.771** |
| pimc (12/120) | 0.654 |
| vmc-learned-id | 0.484 |
| vmc-learned-basic | 0.483 |
| random | 0.108 |

**Fires the pre-registered FAILURE branch.** `id` (0.484) ≈ `basic` (0.483), mean
Δ = **+0.000**. On the synergy decks it *should* help most, Δ = **+0.007** (Spirit
+0.042, Golgari +0.042, Hardened −0.062 — noise). Held-out reads got **worse** with
identity (vs material 0.30→0.25, vs pimc 0.38→0.32) — the +285 features add overfit
surface on the data-limited fit. So **card identity alone does not lift the learned
leaf**, and `vmc-material` (0.77) tops the capsule a third time.

## Experiment 3 — dense PIMC-value distillation (target lever) — PRE-REGISTERED

*Committed before the run (this section written first).* Experiments 1
(distribution) and 2 (representation) were both nulls, isolating the bottleneck
to the **target**: the leaf is a linear fit to sparse terminal 0/½/1 MC outcomes.
This A/Bs two `BasicE2Encoder` leaves over the **same random self-play
distribution** — the only difference is the label:

- **vmc-learned-basic** — terminal MC outcome target (`learn_value`).
- **vmc-learned-dense** — PIMC best-action score target
  (`learn_value_from_pimc_scores`): at each sampled state the teacher PIMC scores
  the mover's actions (`PimcPolicy::score_actions`), and the best score `v∈[-1,1]`
  becomes a **soft** win-prob label `(v+1)/2` for the mover / `(1-v)/2` for the
  opponent — one dense value per state instead of one noisy terminal bit per game.

Panel adds random, vmc-material, pimc. 5 decks × 12 games/pair.

**Pre-registered readout.** Primary = mean point-rate + held-out vs vmc-material /
pimc. **Success** = dense beats basic by a meaningful margin *and* moves the
held-out reads upward. **Failure** = dense stays ~0.48 — meaning the issue is not
distribution, representation, *or* sparse labels alone, and the next plausible
blockers become **linear capacity**, **search-leaf mismatch**, or **PIMC target
quality**. Caveat registered in advance: basic trains on all states of `CAP_TRAIN`
full games (more rows); dense on ≤ `CAP_DENSE_STATES` PIMC-labeled states — a
row-count asymmetry that *disadvantages* dense, so a dense win is unambiguous while
a dense loss is partly confounded with data quantity.

**Result — fires the FAILURE branch (third controlled negative).**
→ [`rl-benchmark/capsule-dense.txt`](rl-benchmark/capsule-dense.txt)

| policy | mean pr |
|---|---:|
| vmc-material | **0.796** |
| pimc (12/120) | 0.617 |
| vmc-learned-dense | 0.508 |
| vmc-learned-basic | 0.483 |
| random | 0.096 |

Dense (0.508) ≈ basic (0.483): Δ = **+0.025**, head-to-head 0.534 — no meaningful
margin (per-deck Δ swings −0.13…+0.21, centered near zero). Held-out is **mixed,
not uniformly up**: dense improved vs its teacher PIMC (0.38→0.47) but got *worse*
vs material (0.30→0.15). So the pre-registered success condition (a real margin
*and* held-out up) is not met. **Three controlled negatives now stand:** neither a
better training distribution (exp 1), a richer representation (exp 2), nor a denser
target (exp 3) rescues the cheap linear leaf, which sits at ~0.49 while hand-tuned
material holds ~0.80. The faint dense-vs-PIMC gain reads as "learned the teacher's
distribution," not general strength.

**What the trilogy establishes + the next lever.** The bottleneck is none of
{distribution, representation, target} *alone*. Per the pre-registration, the
remaining blockers are **linear capacity**, **search-leaf mismatch** (a value tuned
as a *predictor* need not be a good *search leaf* — the original Track-A finding,
reproduced here), and **PIMC target quality** (the 12/120 teacher is itself only
mid-field on the capsule, so distilling it caps the student). The honest read: a
*single cheap linear leaf over hand-crafted features* is near its ceiling on this
benchmark regardless of how it's trained — the capsule reliably rewards
material-in-search. The next genuinely different lever is a **non-linear leaf with
card-identity features distilled from a *stronger* teacher** (higher-budget PIMC),
or accepting material-in-search as the capsule's cheap-referee ceiling and moving
the RL question to a domain where synergy pays off more than aggro tempo.

## Where the benchmark stands + next lever

**Two controlled negatives now bound the problem:** neither a better training
**distribution** (exp 1, PIMC self-play) nor a better **representation** (exp 2,
card identity) rescues the learned leaf; both sit at ~0.48, below hand-tuned
material (0.77). The bottleneck is what's left: the **target**. The value regresses
sparse, high-variance **terminal 0/½/1 MC outcomes** at a 30-game budget — no
feature space or state distribution fixes a fit to a noisy label.

**Next experiment (pre-committed):** **distill PIMC's *value*, not its
distribution.** Expose `PimcPolicy`'s per-candidate rollout scores (`sums`) as a
reusable scoring API, and regress the cheap leaf on those **dense** position values
instead of terminal outcomes. This is the one lever the two negatives point at, and
it directly tests "compress the expensive search's *evaluation* into a cheap
forward pass."

**Standing caveats:** 30-game data-limited leaves (identity's extra features make
this worse, not better); bounded PIMC 12/120; screen-tier per cell (12 games/pair,
240 aggregated); per-deck (archetype-specialized) training. The robust, repeated
finding across both experiments is `vmc-material` topping the aggro-leaning capsule
— consistent with the deckbuild arc's "material-in-search flatters aggro."

Deprioritized: scaling self-play games (expensive under PIMC; distribution is not
the lever); a bigger net on the same features/target (the parked MLP already
refuted the linear-ceiling hypothesis — same sparse target).
