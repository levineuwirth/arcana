# Deckbuilding-as-evaluation — synthesis (peer-review package)

**Status: frozen.** This packages a closed experimental arc for review. Self-contained;
every claim links to a committed artifact. The headline result rests on **pre-registered
bucket movement**, not on small-N rank correlations (see *Caveats*).

## Thesis / conclusion

We tried to turn our MTG engine into a **deck-strength evaluator** — rank real decklists by
simulated performance — as the objective for a deckbuilding optimizer. The arc's conclusion:

> **The evaluator's objective is *jointly* limited by three things — referee strength, card
> fidelity, and field breadth — and no single one of them is the fix.** Each experiment moved
> one lever and measured against real MTGTop8 finishes; each lever helped where it mechanically
> could and left a residual the others owned.

This is a negative-but-precise result: it says *why* "strength under our referee" is not yet
"strength in Magic," and it decomposes the gap into three addressable causes.

## The question

Given real MTGTop8 decklists, does our gauntlet's ranking (deck point-rate vs a field, under a
chosen referee policy) track **real-world archetype strength** (tournament finishes)? If yes,
it is a usable optimization objective. If no, *what* breaks it?

Ground truth = per-archetype strength from the Kaggle MTGTop8 dump's `player_result` finishes
(`arcana-ai/scripts/placement_strength.py`), correlated against our gauntlet point-rate via a
conservative canonical-bucket map (`placement_vs_gauntlet.py`).

## Experimental chain (each step moved one lever)

1. **Build the gauntlet + ingest real decks.** Paired-seed, seat-swapped, block-bootstrap CIs.
   Lesson: the binding constraint was **catalog coverage** — 0% of real decks were playable
   until ~62 targeted cards + a Crew feature were implemented.
   → `docs/gauntlet-results/coverage-snapshot.txt`, `pi_gauntlet_62.csv`

2. **Referee sensitivity.** vmc↔Random ρ=0.71 (64-deck); vmc↔small-PIMC ρ=0.88 (12-deck). The
   cheap referee tracks a stronger one better than noise, but mildly over-rates aggro /
   under-rates synergy. → `referee-sensitivity.txt`

3. **Constrained deckbuilding inside a capsule (the objective test).** A (1+1) hill-climb
   maxed the VmcMaterial gauntlet score (fitness 0.04→0.52) and built a coherent aggro-midrange
   pile — which is the **single worst** deck in its own field under higher-budget PIMC (0.13 vs
   every seed ≥0.21). The VMC gain doesn't evaporate under a stronger referee, it **inverts**.
   A rebalanced "v2" leaf is a *better judge* (mean-abs-error vs PIMC halves) but **still a
   gameable target** (the optimizer relocates the exploit). Textbook Goodhart.
   → `docs/capsule-pioneer/results.txt`, `pimc-select.txt`

4. **Ground truth: VMC vs real.** Spearman ρ = **0.00** (mean finish) / **−0.39** (top-8) over
   the 62-deck field; +0.26 / −0.03 over the 12-deck subset. The extremes invert (our #1 Gruul
   Aggro is real-world weakest). → `gauntlet-vs-real-PI.txt`

5. **Direct PIMC vs real.** Small-PIMC (8/80) on the same 12 decks: ρ = **−0.09 / −0.49** — no
   better than VMC. So "PIMC tracks reality" is false *directly*; a stronger referee is *less*
   aggro-biased but not aligned. → `pimc-vs-real-PI.txt`

6. **Fidelity control (why both referees fail).** Restrict to decks with **no** GAP-approximated
   cards: **0 of 65** covered PI decks are zero-GAP; the field collapses. The load-bearing
   approximations concentrate on **synergy payoffs** (Hangarback Walker, Spirit/flier anthems,
   Soul-Scar Mage), which weaken non-aggro decks *regardless* of referee — the mechanism behind
   (4)+(5). → `fidelity-subset-PI.txt`

7. **Fidelity grind + payoff check.** Implement the payoff cards (Hangarback death-Thopters +
   counter-pump, Supreme Phantom / Empyrean Eagle anthems, Winding Constrictor counter-
   replacement, Smuggler's Copter loot, Fatal Push Revolt + a new
   `permanent_left_battlefield_this_turn` accessor). The relaxed field recovers **4 → 32 decks,
   3 → 15 archetypes**. Re-running VMC-vs-real: **ρ = −0.40 / −0.20 (N=4)** — no *global*
   improvement, **but** the confounds decompose by mechanism: the fix aligned the synergy whose
   payoff is *immediate board material* (UW Spirit anthems: vmc 0.65 ≈ real 0.59) and left
   *grindy* synergy under-rated (Golgari Scales 0.37 vs 0.54) and aggro over-rated (Gruul 0.78
   vs 0.51). → `fidelity-vmc-vs-real-PI.txt`, `fidelity32_vmc.csv`

8. **Referee arm (the crux).** See next section.

## Crux experiment: higher-budget PIMC on the fidelity-fixed field

**Design.** Same 8 fidelity-fixed decks (2 each × the 4 mapped buckets) under **VMC** and
**PIMC 16/160**, `CORPUS_DUELS=2`. Controlled: identical decks, only the referee differs.
(PIMC on all 32 decks ≈ 992 games ≈ 15 h — infeasible; hence the balanced subset.)

**Pre-registration** (committed *before* the PIMC run, commit `3da04f46`,
`pimc-arm-prereg-PI.txt`): if referee strength is the fix → Gruul **down**, Scales/Sultai
**up**, spread less aggro-skewed, ρ improves. Failure read also registered.

**Bucket movement (the robust readout):**

| bucket | VMC | PIMC 16/160 | Δ | real_str | prediction |
|---|---|---|---|---|---|
| Gruul/RG Aggro | 0.750 | 0.661 | **−0.089** | 0.511 | #1 down — ✅ (largest) |
| Golgari Scales | 0.268 | 0.411 | **+0.143** | 0.542 | #2 up — ✅ (2nd largest) |
| UW Spirit Aggro | 0.625 | 0.607 | −0.018 | 0.594 | #4 stable — ✅ |
| Sultai Control | 0.357 | 0.321¹ | −0.036 | 0.512 | #3 up — ❌ (¹n=1, noisy) |
| **spread (max−min)** | **0.482** | **0.340** | — | — | #5 compress — ✅ |

Spearman (secondary, N=4): VMC −0.40 / −0.20 → PIMC −0.20 / −0.40 (both within noise).

**Verdict: directional confirmation, partial, noisy.** The two headline pre-registered moves
both happened and were the largest — a stronger referee pulls over-rated aggro *down* and
under-rated grind *up*, toward real, on faithful decks (vs the small-PIMC 8/80 arm, which
barely moved: *more budget → more movement*). **But** 16/160 does not achieve alignment: Gruul
is still #1 and over-rated, and the global ρ stays negative. So the residual is **partly**
referee strength (confirmed, and it scales with budget) **and partly** not (field / metric /
still-approximated shocklands). → `pimc-arm-result-PI.txt`, `fidelity_sub8_{vmc,pimc}.csv`

## Caveats (read before citing any number)

- **Small-N ρ is secondary.** Every correlation is N=4–7 fuzzy-matched archetype buckets over a
  narrow, sometimes synergy-heavy field, with wide CIs. The ρ values are *not* robust and
  sometimes disagree between the mean-finish and top-8 metrics. **The load-bearing evidence is
  the pre-registered bucket *movement*** (direction + relative magnitude), which is stable
  across the experiments; treat exact ρ as illustrative only.
- **Field ≠ metagame.** Point-rate in an 8–62-deck field we can represent is not real-world
  strength; the field is a coverage-selected subset (Modern collapsed to a Jund mirror).
- **Real strength is confounded.** Tournament finishes reflect metagame/popularity/pilot skill;
  strengths sit in a narrow band (~0.51–0.60), so rank order is itself low-signal.
- **Residual approximations.** The recovered field is the *relaxed* tier — shocklands still
  approximated (lowest fidelity impact, but nonzero). Lord anthems self-include (accepted
  caveat, no exclude-source primitive). PIMC has a known strategy-fusion weakness.
- **One capsule, one format.** All fidelity/referee-arm numbers are Pioneer.

## Reproducibility

Dataset: `kaggle.zip` (repo root, gitignored; sha256 in `docs/gauntlet-results/README.md`).
Pipelines + exact commands: `docs/gauntlet-results/README.md`. Key code:
`build_fidelity_subset` (`arcana-ai/src/deck_corpus.rs`), `MaterialValueV2` /
`permanent_left_battlefield_this_turn` (`arcana-core/src/{search,script}.rs`), the capsule
optimizer (`arcana-ai/src/deckbuild.rs`), and the two Python ETLs
(`arcana-ai/scripts/placement_*.py`). PIMC budget is env-configurable
(`CORPUS_PIMC_SAMPLES` / `CORPUS_PIMC_CAP`).

## If you resume (recommended order)

1. **Field-breadth engine batch first, not a distilled referee.** Implement the remaining
   high-leverage blockers — Once Upon a Time (free-first-spell alt-cost), Soul-Scar Mage /
   Torbran (damage-**replacement** `ReplacementKind` variants), Tarmogoyf (dynamic `*/1+*`
   graveyard CDA) — to add aggro/midrange archetype buckets and escape N=4. A distilled referee
   trained/validated on the N=4 field would be impossible to distinguish from one that learned
   *this capsule's quirks* rather than Magic strength; it needs a broad validation set first.
2. **Then** build the stronger referee (higher PIMC budget and/or a PIMC-distilled cheap value).
3. **Re-run the same pre-registered bucket-movement check** with VMC / PIMC / distilled on the
   broadened field. Keep pre-registration + bucket-movement as the primary readout.
