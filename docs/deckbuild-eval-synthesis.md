# Deckbuilding-as-evaluation — synthesis (peer-review package)

**Status: repaired after validation review.** This packages a closed experimental
arc for review. The internal results are strong; the external "vs real" axis is
now relabeled as what it actually is: **MTGTop8 recorded-finish conversion**, a
top-finish-censored proxy, not full-field Magic strength.

## Thesis / conclusion

We tried to turn the MTG engine into a **deck-strength evaluator** — rank real
decklists by simulated performance — as the objective for a deckbuilding
optimizer. The repaired conclusion is:

> **The evaluator is jointly limited by referee strength, card fidelity, field
> breadth, and the validation target.** The harness and Goodhart findings are
> internally solid. Claims about "real Magic strength" are not established from
> MTGTop8 alone, because the dump only exposes recorded top finishers.

The negative result survives, but at reduced strength: VMC is gameable and does
not track the external conversion proxy; PIMC changes rankings in structured
ways but is not validated as aligned with reality.

## The Question

Given real MTGTop8 decklists, does our gauntlet ranking (deck point-rate vs a
field, under a chosen referee policy) track an external competitive signal? If
yes, it is a usable optimization objective. If no, what breaks it?

Validation target = per-archetype conversion from the Kaggle MTGTop8 dump's
recorded `player_result` finishes (`arcana-ai/scripts/placement_strength.py`),
correlated against gauntlet point-rate via a repaired conservative bucket map
(`placement_vs_gauntlet.py`). For Pioneer, all 528 usable events have only 2-8
recorded rows, so this target is not P(make top 8) and not full-field strength.

## Experimental Chain

1. **Build the gauntlet + ingest real decks.** Paired-seed, seat-swapped,
   duel-block-bootstrap CIs. Lesson: catalog coverage was binding — 0% of real
   decks were playable until ~62 targeted cards + Crew were implemented.
   → `coverage-snapshot.txt`, `pi_gauntlet_62.csv`

2. **Referee sensitivity.** vmc↔Random ρ=0.71 (64-deck); vmc↔small-PIMC ρ=0.88
   (12-deck). The cheap referee tracks a stronger one better than noise, but
   mildly over-rates aggro / under-rates synergy. → `referee-sensitivity.txt`

3. **Constrained deckbuilding inside a capsule.** A (1+1) hill-climb maxed the
   VmcMaterial gauntlet score and built a coherent aggro-midrange pile, but that
   deck became the **single worst** deck in its own field under higher-budget
   PIMC. MaterialValueV2 halves judge MAE vs PIMC but remains gameable as a
   target. This is the cleanest Goodhart result in the arc.
   → `docs/capsule-pioneer/results.txt`, `pimc-select.txt`

4. **External validation: VMC vs MTGTop8 conversion.** After repairing aliases
   (drop Rakdos→Mono-Black proxy, remove Devotion-to-Golgari misjoin, widen
   Scales variants), VMC has Spearman ρ = **−0.20** vs recorded finish and
   **−0.30** vs event-win share over 5 conservative Pioneer buckets. This says
   VMC does not track the MTGTop8 conversion proxy; it does not prove anything
   about full-field Magic strength. → `gauntlet-vs-real-PI.txt`,
   `validation-target-audit-PI.txt`

5. **Direct small-PIMC vs the same proxy.** On the same 12-deck subset scored
   under both referees, vmc is **+0.10 / −0.10** and small-PIMC is **−0.10 /
   −0.50** vs recorded / event-win. PIMC is less material-biased in some bucket
   movements, but not externally aligned. → `pimc-vs-real-PI.txt`

6. **Fidelity control.** Strict zero-GAP Pioneer field: **0 of 65** covered decks
   survive. The load-bearing approximations concentrate on synergy payoffs
   (Hangarback Walker, Spirit/flying anthems, Soul-Scar Mage, Torbran), so
   fidelity is a real confound and cannot be dodged by field restriction.
   → `fidelity-subset-PI.txt`

7. **Fidelity grind + payoff check.** Implementing the payoff cards recovered the
   relaxed field **4 → 32 decks, 3 → 15 title strings**. Re-running VMC on the
   recovered field still fails the conversion proxy (**ρ = −0.40 / −0.20, N=4**).
   Spirits sit near the target post-fix, but the fields differ and pre-fix Spirits
   were already near the target, so causal attribution is not proven.
   → `fidelity-vmc-vs-real-PI.txt`, `fidelity32_vmc.csv`

8. **Referee arm.** Same 8 fidelity-fixed decks under VMC and PIMC 16/160. Gruul
   fell and Scales rose, the two most interesting movements, but the strict
   pre-registration score is mixed: two clean passes, one direction-only threshold
   miss, Sultai down at n=2, Gruul still top, and event-win rho worsened. This
   supports "referee choice matters," not "PIMC fixes alignment."
   → `pimc-arm-prereg-PI.txt`, `pimc-arm-result-PI.txt`

## Crux Experiment: Higher-Budget PIMC

**Design.** Same 8 fidelity-fixed decks (2 each × 4 mapped buckets) under **VMC**
and **PIMC 16/160**, `CORPUS_DUELS=2`. Controlled: identical decks, only the
referee differs.

| bucket | VMC | PIMC 16/160 | delta | recorded | strict prereg read |
|---|---:|---:|---:|---:|---|
| Gruul/RG Aggro | 0.750 | 0.661 | -0.089 | 0.511 | direction yes; <0.60 threshold missed |
| Golgari Scales | 0.268 | 0.411 | +0.143 | 0.526 | pass, barely |
| UW Spirit Aggro | 0.625 | 0.607 | -0.018 | 0.594 | pass |
| Sultai Control | 0.357 | 0.321 | -0.036 | 0.512 | fail; n=2 decks |
| spread (max-min) | 0.482 | 0.340 | - | - | compressed, but Gruul still top |

Spearman (secondary, N=4): VMC **−0.40 / −0.20** → PIMC **−0.20 / −0.40**.

**Verdict: one bucket survives the control.** The regression-to-the-mean null is
now *calibrated*: VMC was re-run at four independent seeds on a fresh
source-auditable spanning-8 (`control-arm-PI.txt`, `control_sub8_vmc_s{0..3}.csv`).
The null band (2·SD of the per-bucket VMC mean across seeds) resolves the arm:

| bucket | arm Δ (VMC→PIMC) | VMC null band (2·SD) | survives? |
|---|---:|---:|---|
| Golgari Scales | +0.143 | 0.040 | **yes** — Scales is stable under VMC |
| Gruul/RG Aggro | −0.089 | 0.120 | **no** — Gruul's own VMC rate wanders 0.68–0.84 |
| UW Spirit Aggro | −0.018 | 0.102 | n/a (stable, uninformative) |
| spread compress | −0.142 | ±0.196 swing | **no** — spread itself is that noisy |

So the arm's "directional confirmation" reduces to **one bucket**: a stronger
referee moves *grindy synergy* (Scales) up toward the target, and that is real.
The headline **Gruul-down move and the spread compression were regression to the
mean** — exactly the risk this control was built to catch. "Referee strength is a
real lever" holds for grindy synergy specifically, **not** for aggro calibration,
and the earlier "more budget → more movement" gloss is unsupported (the 8/80 arm
also differed in deck set / field / fidelity).

A **clean PIMC 16/160 redo on the *same* source-auditable 8** (the apples-to-apples
the historical arm couldn't do — `control_sub8_pimc_s0.csv`) confirms this: Scales
**+0.134** survives (3.4× the null), while Gruul is only **−0.045** (well within the
0.120 null — the retraction is firmer than the historical −0.089). Two honest
wrinkles: the Scales lift is carried by **one of the two decks** (Gb Hardened Scales
0.25→0.54; Hardened Snakes flat), so it is real but modest and deck-specific; and UW
Spirit clears the null (−0.112) but moves *away* from the censored target, not toward
it. Net: the only surviving referee-strength claim is "a stronger referee gives a
narrow, deck-specific lift to grindy synergy" — a real-but-modest lever, not a fix.

## Caveats

- **The validation target is censored.** MTGTop8/Kaggle provides recorded top
  finishers, not full tournament fields. `recorded_strength` is within-recorded
  finish conversion; `event_win_share` is event wins / recorded rows.
- **The external target is low-signal.** Event-bootstrap CIs overlap heavily; the
  target supports only coarse contrasts, not precise rank claims.
- **Small-N ρ is secondary.** Correlations use 4-5 fuzzy matched buckets in narrow
  fields. Treat exact ρ values as diagnostics, not headline proof.
- **Field ≠ metagame.** Point-rate in an 8-62 deck representable field is not a
  metagame win rate.
- **Residual approximations remain.** The recovered field is relaxed; shocklands
  are still approximated, lord anthems self-include, and PIMC has known
  strategy-fusion weaknesses.
- **Historical subset provenance is limited.** The old subset CSVs identify rows
  by archetype title, not source deck file stem, and duplicate archetype rows are
  not source-auditable. This is documented in `subset-provenance-PI.txt`; future
  corpus gauntlets append `[source_id]` to deck names and preserve config headers.

## Reproducibility

Dataset: `kaggle.zip` (repo root, gitignored; sha256 in
`docs/gauntlet-results/README.md`). Pipelines + exact commands:
`docs/gauntlet-results/README.md`. Key code: `build_fidelity_subset`
(`arcana-ai/src/deck_corpus.rs`), `MaterialValueV2` (`arcana-ai/src/search.rs`),
`permanent_left_battlefield_this_turn` (`arcana-core/src/script.rs`), the capsule
optimizer (`arcana-ai/src/deckbuild.rs`), and the placement ETLs
(`arcana-ai/scripts/placement_*.py`). PIMC budget is env-configurable
(`CORPUS_PIMC_SAMPLES` / `CORPUS_PIMC_CAP`).

## If You Resume

1. **Fix validation before optimizing harder.** Best is a full-field or match-level
   external data source. If MTGTop8 remains the target, pre-register only coarse
   bucket contrasts supported by event-bootstrap CIs.
2. **Run the missing same-referee control.** A fresh VMC re-run on the same 8
   source-auditable decks calibrates the regression-to-the-mean null for any
   future referee arm.
3. **Then broaden the field.** Implement Once Upon a Time, Soul-Scar Mage /
   Torbran replacement effects, and Tarmogoyf CDA to add archetype buckets and
   escape N=4. A distilled referee trained on the current narrow validation set is
   premature.
4. **Keep the strongest internal thread separate.** The harness, coverage/fidelity
   audits, and Goodhart/referee-holdout results are already useful even if the
   "vs real" branch stays limited by data.
