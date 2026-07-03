# Peer review — deckbuilding-as-evaluation arc (external skeptical pass)

*Review of the frozen package `docs/deckbuild-eval-synthesis.md` (+ `docs/rl-status.md`,
`docs/gauntlet-results/`, `docs/capsule-pioneer/`), conducted 2026-07-02 against commit
`7f7676c8`. Every load-bearing number was recomputed from the committed CSVs, the code,
and the raw Kaggle dump (`kaggle.zip`, sha256 matches README). Two new quantitative
checks were run for this review: the event field-size census and the event-level
bootstrap of the real-side bucket strengths (methods in the Appendix).*

**Verdict in one paragraph.** The internal results — the harness, the capsule Goodhart
experiments, and the fidelity audit — are solid, carefully hedged, and reproducible from
committed artifacts; every number recomputed (bucket means, all four N=4 Spearmans,
ρ=0.884, the v1/v2 MAE 0.175→0.092, field counts, games arithmetic) checks out, and the
pre-registration commit ordering is genuine (`3da04f46` 18:48 → `d6cbb5a7` 20:45 same
day). But the entire "vs real" axis rests on a ground-truth metric that is structurally
weaker than any document discloses: **the MTGTop8 dump records only top-8 finishers**,
so "real strength" is a within-top-8 conversion metric, not archetype strength — and the
real-side rank order is statistically indistinguishable across most buckets. The
three-lever thesis survives as a directional claim; several of its quantitative supports
do not.

---

## Findings — unsupported or overclaimed

### U1 (most severe). The ground truth is a top-8-censored conversion metric, mislabeled everywhere

Checked directly against the dump: **all 528 Pioneer events used have a recorded field of
≤ 8 players** (333 events report 8, 79 report 4, 70 report only 2; census in Appendix A).
Consequences:

- `real_str` (`arcana-ai/scripts/placement_strength.py:100` — `norm = (lo-1)/(field-1)`)
  is mean placement *among top-8 finishers*, not finish rate in a tournament field. Swiss
  performance and top-8 attainment rate are invisible to it.
- `real_top8` (`arcana-ai/scripts/placement_vs_gauntlet.py:87` —
  `(lo-1)/field < 0.125`) can only fire at `lo == 1` when field ≤ 8. It is literally
  **event-win share among top-8 appearances**, yet it is labeled "top-8 share" in
  `docs/gauntlet-results/README.md:53`, `docs/deckbuild-eval-synthesis.md:49`, and every
  artifact header. A reader will inevitably read it as P(make top 8), which this data
  cannot express.
- This plausibly biases the target *against aggro* on exactly the contested axis: aggro
  archetypes classically reach elimination rounds often (unobservable here) and convert
  them less. "Our gauntlet over-rates aggro vs real" is partly confounded with "our
  real-strength metric only measures elimination-round conversion." No caveat list
  mentions the censoring; `placement_strength.py:14` says "metagame-confounded" but not
  this.

### U2. The real-side rank order carries almost no signal — more severely than the "narrow band" caveat admits

Event-level bootstrap of the real-side estimates (2,000 resamples, same bucket map;
Appendix B): Golgari/HardScales `real_str` = 0.542 with 95% CI **[0.441, 0.641]** (n=36
entries, one title: "Golgari Aggro"); **P(Scales > Gruul on real_str) ≈ 0.71**. So "our
worst cluster is real-world strong / the extremes invert" is a ~70/30 proposition on the
strength metric, not a fact. Sultai (0.512) vs Gruul (0.511) — load-bearing for the N=4
rank correlations — differ by 0.001 with fully overlapping CIs. The only real-side
contrast that is actually solid is conversion: Gruul top-share 0.093 [0.037, 0.160], 0/40
at ≥2 stars, vs Spirit 0.192 (P≈0.99). The synthesis correctly demotes ρ to
"illustrative," but headline ρ values (0.00 / −0.39 / −0.49) are quoted throughout
(`deckbuild-eval-synthesis.md:49-55`) against a target whose ranking is mostly noise; the
honest summary is: **the target supports ~2 distinguishable strata.**

### U3. The referee arm is scored more softly than its own pre-registration

Strictly against `docs/gauntlet-results/pimc-arm-prereg-PI.txt:18-23`:

| pred | registered | actual | strict verdict |
|---|---|---|---|
| #1 Gruul down | "< 0.60" | 0.661 | direction met, **threshold missed** |
| #2 Scales up | "> 0.40" | 0.411 | met (barely) |
| #3 Sultai up | "> 0.40" | 0.321 (down) | **failed** |
| #4 Spirit stable | 0.50–0.65 | 0.607 | met |
| #5 Gruul no longer single top; spread less skewed | — | still #1; spread 0.482→0.340 | **half-failed** |
| #6 both ρ improve | — | mean −0.40→−0.20, top-8 −0.20→**−0.40** | **failed** |

That is 2 clean passes out of 6. The result artifact's "directional confirmation,
partial, noisy" is defensible; the synthesis table (`deckbuild-eval-synthesis.md:87-93`)
with four ✅ marks — and row #5 silently renamed to just the compression half — overstates
it.

### U4. The Sultai miss is excused with a claim contradicted by the artifact in the same commit

`pimc-arm-result-PI.txt:10` and the synthesis table footnote say "only 1 of 2 Sultai
decks produced a point-rate row this run (n=1, noisy)." The CSV committed in the *same*
commit `d6cbb5a7` (`docs/gauntlet-results/fidelity_sub8_pimc.csv:2-3`) has **two** Sultai
rows (0.2500, 0.3929), and the reported 0.321 is exactly their mean. The one failed
pre-registered direction is a full n=2 miss, not an n=1 artifact.

### U5. The headline confirmations coincide with regression-to-the-mean, and no control arm calibrates that null

Gruul and Scales were selected as the extreme buckets under VMC-family measurements;
under a pure-noise null, any independent re-measurement pulls the top down, the bottom
up, and compresses spread — precisely preds #1, #2, #5. Effect sizes are ~1.0σ (Gruul
−0.089) and ~1.6σ (Scales +0.143) on 56 games/bucket, *before* accounting for duel-block
correlation, and same-referee cross-field wander of the same buckets is comparable
(Gruul VMC: 0.822 → 0.782 → 0.750 across the three fields; Scales: 0.349 → 0.268 →
0.368). A fresh same-referee VMC re-run on the same 8 decks would have quantified the
re-measurement null cheaply; it wasn't run. No document mentions this threat.

### U6. "More budget → more movement" is confounded and partly contradicted

`deckbuild-eval-synthesis.md:99-100` claims the 8/80 arm "barely moved" vs 16/160. In
fact the 8/80 arm moved Gruul **up** (0.670→0.750, away from real;
`pimc-vs-real-PI.txt:31,49`), and its mean |Δ| (0.095) *exceeds* the 16/160 arm's
(0.072). The two PIMC arms differ in decks, field composition, and fidelity state as well
as budget — the budget-monotonicity inference has no controlled support, and the sign
flip on Gruul across arms is as consistent with noise.

### U7. "The fix aligned Spirits" is a cross-field before/after that the underlying artifact disclaims

Pre-fix, the Spirit bucket was *already* aligned in the 62-deck field: 0.592 vs real
0.594 (`gauntlet-vs-real-PI.txt:20`) — with the anthems still GAP'd. Post-fix it is
0.651, slightly *further* from 0.594, in a different field.
`fidelity-vmc-vs-real-PI.txt:38-39` admits "the three VMC fields are different decks (not
a controlled before/after)," yet synthesis step 7 asserts the causal mechanism ("the fix
aligned the synergy whose payoff is immediate board material"). Also note the implemented
anthems self-include (`arcana-cards/src/eld/supreme_phantom.rs:4-6`,
`arcana-cards/src/m20/empyrean_eagle.rs:5-7`), making them mildly stronger than oracle —
in the direction of the observed over-shoot. The defensible claim is only "post-fix
Spirits sit near real"; attribution to the fix is unsupported.

---

## Findings — supported but caveated / methodological gaps

### C1. Alias-map defects materially move the small-N ρ

(a) "Devotion to Golgari" (n=1, 0.303) matches **both** the Scales bucket (substring
`"golgari"`, `placement_vs_gauntlet.py:33`) *and* is the **entire gauntlet side** of the
"Mono-Green Aggro" bucket (substring `"devotion to g"`, line 35) — a Golgari devotion
deck joined against real "Mono Green Aggro"/"Devotion to Green" finishes, and
double-counted across buckets. Dropping that bucket alone moves the headline 62-deck ρ
from 0.000→**−0.14** (mean) and −0.393→**−0.31** (top-8): direction unchanged, exact
headline numbers not robust. (b) "Rakdos Aggro"↔"Mono Black Aggro" (line 36) is an
admitted proxy, not "conservative." (c) The Scales bucket's real side counts only decks
titled "Golgari Aggro," while the gauntlet side sweeps 11 title variants whose own
finishes exist in the dump under their own titles but are not counted.

### C2. Subset provenance is not committed

No manifest identifies which 8 (or 12) decklists form the "spanning subsets" — selection
was manual and undocumented, and within-bucket per-deck spread (Gruul 0.679 vs 0.821 in
`fidelity_sub8_vmc.csv`) is as large as the headline movement, so which-two-decks
matters. The committed CSVs are also stripped of the runner's self-describing header
(`GauntletReport::to_csv` emits `# gauntlet referee=... base_seed=...`,
`arcana-ai/src/deckeval_runner.rs:304-313`; the committed files start at the column
header), so referee/seed/duels aren't auditable from the artifacts. The Random-arm
64-deck CSVs are absent entirely (disclosed at `rl-status.md:285-289`, not in the
synthesis).

### C3. The pre-grind fidelity finding is no longer regenerable from committed code

`build_fidelity_subset`'s GAP list (`arcana-ai/src/deck_corpus.rs:443-454`) already has
the six fixed cards removed, so re-running it reproduces the recovered field, not the
"0/65 strict" result. Regenerating the original finding requires re-adding six names by
hand.

### C4. "15 archetypes" counts title strings

Of the 32 recovered decks, 12 are Scales-family and 8 Spirits; canonical diversity is ~8
buckets, of which 4 map to real. The caveats do note the synergy-heavy field; the
"3 → 15 archetypes" framing flatters it.

### C5. Even the "strong" referee is weak in absolute terms

PIMC 16/160/8-candidates vs the Track-A default 60/250/16
(`arcana-ai/src/search.rs:777-779`, `deckeval_runner.rs:58-60`); VMC is 4 rollouts ×
20-step cap × 8 candidates (`arcana-ai/src/deckeval.rs:56-60`). "Referee strength scales
toward alignment" is an extrapolation from two points in the weak regime, one of them
confounded (U6).

---

## What is genuinely supported

- **The harness does what it claims.** Seat-swapped shared-seed duels with an honest
  disclosure that shuffles key off physical seat (`deckeval_runner.rs:14-22`), correct
  duel-block bootstrap (lines 387-414, 480-500), correct scoring and CSV semantics,
  conservation and determinism tested. Verified in full.
- **The capsule Goodhart results** — optimizer output inverting under PIMC (0.133 vs
  every seed ≥ 0.208), v2 halving judge MAE (0.175→0.092, recomputed) while remaining
  gameable as a target, PIMC-select recovering a coherent mid-field deck — are internally
  valid, do not depend on the flawed real-world axis, carry honest noise annotations, and
  are the strongest results of the arc.
- **The fidelity audit** (100% of covered decks approximated; blockers concentrated on
  synergy payoffs) is a clean, important engineering finding, and the six payoff-card
  implementations match their claims with documented residuals (lord self-include, forced
  Copter loot, Constrictor player-counter half elided, Revolt accessor faithful).
- **Pre-registration was real and falsifiable**, with a registered failure read — a
  genuine methodological strength even though the scoring drifted (U3).
- **All numeric summaries recomputed check out:** bucket means from every CSV, all four
  N=4 Spearmans (VMC −0.40/−0.20, PIMC −0.20/−0.40), referee-sensitivity ρ=0.884, field
  counts (32 decks / 15 titles; Golgari ×12, Spirits ×8), games arithmetic
  (122 = 61×2, 28 = 7×2×2, 992-game infeasibility estimate), 0 draws in 3,782 games.

---

## Assessment of the thesis

"Jointly limited by referee strength, card fidelity, and field breadth; no single lever
fixes alignment" is **directionally reasonable but established only at reduced
strength**.

Solidly established: VmcMaterial is exploitable and referee-shaped (internal capsule
evidence); fidelity approximations are pervasive and payoff-concentrated; referee choice
moves rankings in a structured way.

Not established: any quantitative statement about alignment *with real Magic strength*,
because the validation target is a censored, near-flat conversion metric with fuzzy joins
(U1, U2, C1), and the referee-arm's "toward real" movement is ~1σ, partly
regression-to-the-mean, with the budget-scaling claim confounded (U5, U6).

The thesis is also missing its fourth limit: **the arc is limited by its validation
target as much as by its three levers.** On the synthesis's own framing question ("is the
pre-registered bucket-movement readout stronger than raw ρ?"): in principle yes — but as
executed it is weaker than presented, because the registered thresholds were partly
missed and the confirmed directions coincide with the regression null.

---

## Recommended next step

**Fix the validation target before spending compute on any lever** — closest to "more
data / real validation," and cheaper than either the engine batch or a distilled referee:

1. **Re-derive and re-label the ground truth** as what it is (top-8 placement /
   event-win conversion), attach event-bootstrap CIs to every real-side number, and
   pre-register *which bucket contrasts are decidable* (currently: Gruul-vs-Spirit
   conversion, little else). If broader real signal is needed, it must come from a source
   with full-field or match-level data; MTGTop8 alone cannot support "alignment with real
   strength."
2. **Repair the alias map** (remove the Devotion-to-Golgari join, drop or justify the
   Rakdos↔Mono-Black proxy, widen the Scales real-side variants), **commit subset
   manifests**, and keep the runner's provenance headers in committed CSVs.
3. **Add the missing control arm:** a fresh VMC re-run on the same 8 decks to calibrate
   the re-measurement (regression-to-mean) null against which any future referee movement
   is judged.

Then the synthesis's own recommendation — the field-breadth engine batch — is the right
lever, since it serves both the field confound and the validation N. A PIMC-distilled
referee remains premature for exactly the reason the synthesis gives. If full-field
external data proves unobtainable, the honest move is the fourth option for the
"vs real" branch specifically: drop alignment-with-reality as the objective and write up
the internally-valid core — the gauntlet harness, the coverage/fidelity audits, and the
Goodhart/referee-ensemble results — which is publishable as-is and does not lean on the
broken axis.

---

## Appendix — new checks run for this review

### A. Event field-size census (basis of U1)

Walked all PI events in `kaggle.zip` with the same parsing as
`placement_vs_gauntlet.real_strength_by_name` (field = max upper rank bound in
`players_info.csv`, events with field < 2 or < 2 parsed rows skipped):

```
PI events used: 528
field=2: 70   field=3: 20   field=4: 79   field=5: 10
field=6: 10   field=7: 6    field=8: 333
events with field <= 8: 528 (100%)
mean reported players/event: 6.27
```

At field ≤ 8, `(lo-1)/field < 0.125` ⇔ `lo == 1`, so `real_top8` = event-win share among
recorded (top-8) appearances.

### B. Event-level bootstrap of real-side bucket strengths (basis of U2)

2,000 bootstrap resamples over *events* (the natural exchangeable unit), same bucket
variants as `PI_BUCKETS`, min_stars=0. Point estimates reproduce the committed artifacts
exactly.

```
bucket               real_str [95% CI]      top_share [95% CI]    n
UW Spirit Aggro      0.594 [0.554, 0.636]   0.192 [0.132, 0.261]  151
Rakdos (=MonoB)      0.587 [0.550, 0.624]   0.145 [0.104, 0.188]  234
Mono-Red (RDW)       0.582 [0.549, 0.619]   0.167 [0.123, 0.211]  234
Golgari/HardScales   0.542 [0.441, 0.641]   0.111 [0.024, 0.229]   36
Mono-Green Aggro     0.535 [0.479, 0.588]   0.178 [0.120, 0.239]  135
Sultai Control       0.512 [0.435, 0.593]   0.186 [0.103, 0.281]   70
Gruul/RG Aggro       0.511 [0.449, 0.570]   0.093 [0.037, 0.160]   86

P(Scales real_str > Gruul real_str)  = 0.71
P(Spirit top_share > Gruul top_share) = 0.99
```

### C. Alias-sensitivity recomputation (basis of C1)

Dropping the mis-joined "Mono-Green Aggro" bucket (whose sole gauntlet member is the
"Devotion to Golgari" deck) from the 62-deck check: Spearman vs real_str 0.000 → −0.143;
vs real_top8 −0.393 → −0.314 (N=6). Additionally dropping the Rakdos↔Mono-Black proxy:
−0.20 / −0.30 (N=5). Direction of the "does not track" conclusion unchanged; exact
headline values not robust to single aliases.

### D. Referee-arm effect sizes (basis of U5)

Per bucket: 2 decks × 28 games = 56 games/arm. Approximate binomial SE of a bucket Δ
≈ 0.09 (ignoring positive duel-block correlation, which reduces effective n further):
Gruul −0.089 ≈ 1.0σ; Scales +0.143 ≈ 1.6σ. Within-bucket per-deck spread in the same CSVs
(Gruul PIMC 0.750 vs 0.571) exceeds both movements.
