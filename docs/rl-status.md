# Arcana RL — status & findings, refresh 2 (for peer review)

*A self-contained write-up of where our game-playing AI + deck-evaluation work
stands, the experiments we ran, and the open questions. Goal: enough context for
an outside researcher to say "here's what I'd do." This refresh adds the major
arc since the last write-up: acting on the prior feedback to pivot toward
**deckbuilding-as-evaluation**, building a real-deck gauntlet harness, and what
we learned running it on real tournament decklists.*

---

## 0. What changed since refresh 1 (TL;DR of the delta)

Last time the headline was about **play strength**: PIMC (search, no learning)
dominates; a self-play-learned value function is a *worse* search leaf than a
hand-tuned material heuristic; "good predictor ≠ good search leaf." That still
stands and is summarized in §2.

The peer feedback was: *park the learned-value-leaf, pivot to deckbuilding as a
black-box optimization problem using the cheap material-in-search policy as a
referee, turn the deck evaluator into a real experiment runner with proper
statistics, and source real decklists.* We did the **infrastructure half** of
that, and it surfaced a result we didn't expect: the binding constraint isn't
the RL/optimization at all — it's **catalog coverage** (an engineering problem),
and the **evaluation referee's bias** dominates any ranking we produce. Details
in §3–§6.

---

## 1. The domain (unchanged)

**Arcana** is a from-scratch Rust implementation of a Magic: the Gathering–style
engine plus a catalog of **~20,600 implemented cards** (out of ~31k printed). The
engine is a faithful rules simulator (stack, priority, layers/continuous effects,
replacement effects, combat, triggered/activated abilities, ~105 effect
primitives). Games are two-player, zero-sum, turn-based, **imperfect-information**
(hidden hands/libraries; shuffled = stochastic), **long-horizon / high-branching**,
over a **huge open-ended action+card space**. Pure Rust, single machine, no GPU,
no PyTorch/JAX.

---

## 2. Track A — play-strength RL (recap; conclusions unchanged)

A value-function pipeline (`arcana-ai`), not AlphaZero. State → 123 identity-free
float features (counts, CMC bins, keyword presence — no card identity). Policies:
`MaterialValue` (hand-tuned leaf), `ValueMcPolicy` (short MC rollouts with a
value leaf), `PimcPolicy` (perfect-information Monte-Carlo), and a learned
`LinearValue`/MLP. Yardstick = round-robin on a mirror deck.

**Findings (still current):**
- **PIMC dominates**; a **material leaf inside lookahead ≈ PIMC** (e.g. 18 vs 20
  / 30) with *zero learning*.
- The **self-play-learned value is a worse leaf** than the hand-tuned material
  one, even after we fixed its predictor accuracy (held-out log-loss 0.51 < the
  heuristic's 0.54 by ~150 games) — **"good predictor ≠ good search leaf."**
- The learned-value-leaf branch is **parked**; the deployable AI is PIMC /
  ValueMc-material. The pipeline is guarded by a smoke test + diagnostics so it
  doesn't bit-rot.

This matters here because the cheap, no-learning **ValueMc(Material)** policy is
exactly the "referee" we reuse below to evaluate *decks*.

---

## 3. Track B — deckbuilding as evaluation (the new work)

The pivot's premise: if we can't easily learn a better *player*, can we use the
players we have to evaluate *decks* — rank real decklists, measure card value,
eventually optimize decklists? Step one is a trustworthy **deck gauntlet**.

### 3.1 The gauntlet harness (`deckeval_runner.rs`)
A reproducible round-robin over a set of decks with the statistics needed to make
a *claim*, not just a point estimate:

- **Seat-swapped, shared-seed duels.** Each deck pair plays *duels* of two games
  that share one top-level engine seed and per-deck policy seeds but swap seats.
  Precisely: the engine keys each library shuffle by *physical seat*, so a deck
  does **not** literally redraw the same library after the swap — the shared seed
  instead balances seat/stream luck at the **duel-block** level (the unit the
  bootstrap resamples). Per-deck policy RNG does cancel.
- **Block bootstrap CIs.** The experimental unit is the duel (its two games share
  seeds and are correlated), so per-deck 95% CIs resample *duel blocks*, not iid
  games — resampling games would make intervals too optimistic.
- **Referee knob.** The policy *both* seats play (we measure decks, not policies):
  `Random` (screening), **`VmcMaterial`** (the cheap material-in-search policy
  from Track A — our standard referee), or **small-budget `Pimc`** (slowest;
  note this is a small PIMC budget, not the full-budget Track-A PIMC that
  dominated §2).
- Output: win matrix + per-deck point-rate (draws = ½) with CIs + CSV.
- Sizing rule of thumb (from your feedback): ~100 games/deck = a screen, 400+ for
  a firm claim.

### 3.2 The deck space + the binding constraint
We sourced **real decklists** from the Kaggle MTGTop8 dataset (~125k tournament
decks across Standard/Modern/Legacy/Vintage/Pioneer/EDH), de-duplicated and
tagged by archetype. A loader normalizes each list against the catalog and
reports which cards don't resolve.

**The surprise:** a decklist is only playable if **every** card is implemented,
and our catalog — though ~20.6k cards — was generated by breadth, not by
competitive relevance. Initial result: **0% of real competitive decks were fully
playable in any format** (mean per-deck coverage 56–67%; a single missing staple
disqualifies a 60-card deck). The bottleneck was not RL, optimization, or eval —
it was **card implementation**.

The miss list is concentrated, though: only ~300–370 distinct cards were missing
per format, and they cluster (dual/shock/fetch lands, a handful of ubiquitous
spells, format-defining creatures). So it's a tractable, frequency-ranked
worklist, not a long tail.

### 3.3 The card-grind (what it took to get real formats playable)
We implemented the highest-leverage missing cards — **62 new cards + 2 engine
features** across ~14 batches — driven by a "which cards block the most
*almost-playable* decks" analysis:

- Lands: 10 Ravnica shocklands (+ a small engine feature: "enters tapped unless
  you pay N life"), 10 original dual lands, 5 ELD Castles, 5 utility lands, 5
  Worldwake **manlands**.
- A real engine feature: **Crew / Vehicles** (a Vehicle is an artifact that
  becomes a creature when crewed — tap creatures with total power ≥ N), reused by
  the manlands' "becomes a creature" animation.
- Spells: burn, board wipes, counters, library-dig, **modal "Commands"/Charms**
  (Kolaghan's/Izzet/Boros), delve (Treasure Cruise, Dig Through Time).
- Creatures incl. **Tarmogoyf** (modeled as a fixed 4/5 — its dynamic
  graveyard-scaled P/T is a known, deferred engine task).

Several "engine levers" we'd budgeted turned out to already exist (**delve** and
**fetchlands** were fully wired — fetchlands now fetch the subtype-bearing lands
we added). Net effect on playable real decks (of 800 distinct/format sampled):

```
                Pioneer    Modern    Legacy    Vintage
fully playable    0→64      0→43      0→1        0
mean coverage   ~67→88%   ~73→77%   ~67→69%   ~62→64%
```

This is itself a finding: **a modest, well-targeted card-implementation push
moves a real-deck eval from "impossible" to "runnable."**

**Reproducibility.** The exact conversion command, dataset source + sha256, the
coverage snapshot, both gauntlet result CSVs, and the missing-card worklist are
checked in under [`docs/gauntlet-results/`](gauntlet-results/) so the tables
below can be rerun and audited from the repo (the 90 MB Kaggle zip itself is
gitignored; its checksum is recorded there).

### 3.4 Gauntlet results — Pioneer (62 decks, 122 games/deck)
Referee = VmcMaterial, block-bootstrap 95% CIs. Point-rate (draws ½):

```
Gruul Aggro        89.3%  [83.6, 94.3]   ┐
Red Deck Wins  73–84% (×5)               │ aggro tier
Rg / Rakdos Aggro  70–80%                ┘
Rakdos PW Control  67%
Azorius Aggro      63%
Sultai Control     ~50%
Golgari / Hardened-Scales (×7)  27–35%   ← +1/+1-counter synergy decks
Temur Midrange      9.0%  [4.1, 14.8]    ← worst
```

Well-separated and **coherent — but it's an artifact of the referee**: aggro
decks (curve out creatures, attack) are exactly what a shallow material heuristic
plays well; the +1/+1-counter synergy and grindy midrange decks rely on lines the
referee can't pilot and on payoffs that are partly GAP'd in our catalog. So those
decks are almost certainly **underrated, not bad.** (0 draws across all 3,782
physical games — 1,891 deck pairs × one seat-swapped duel × 2 games.)

### 3.5 Gauntlet results — Modern (43 decks, 84 games/deck)
Same harness. The result is instructive in a different way:

```
playable Modern field: 43 decks — of which 42 are "Jund" + 1 "4/5c Good Stuff"
ranking: a tight band of Jund builds, 34%–63%, CIs heavily overlapping
```

The Modern field **collapsed to a single archetype.** Why: our card-grind for
Modern targeted Jund's exact cards (Tarmogoyf, Kolaghan's Command, shock/fetch
lands, manlands), so the only fully-covered decks *are* Jund. The gauntlet then
ranks **Jund builds against each other** — a near-mirror — where the material
referee barely separates them (overlapping CIs). This is **selection bias in its
starkest form: the field equals the cards you've implemented.** (Pioneer looked
diverse only because its coverage happened to span several archetypes' staples.)

---

## 4. What we concluded

1. **The harness works.** Paired seeds + block-bootstrap give well-separated,
   CI-bounded rankings on real decklists, reproducibly. The plumbing is done.
2. **Two confounds dominate any ranking it produces**, and both are bigger than
   the statistics:
   - **Referee bias.** Under VmcMaterial, "deck strength" ≈ "how well a shallow
     material heuristic can pilot it." Aggro is flattered; synergy/control/value
     is penalized. The ranking is *internally consistent*, not ground truth.
   - **Coverage selection bias.** The playable field = the decks whose every card
     we've implemented. A narrow card-grind yields a narrow (even single-archetype)
     field — Modern became a Jund mirror.
3. **The bottleneck for "evaluate real decks" was engineering, not ML.** Most of
   the work that moved the needle was implementing cards and one engine feature,
   not anything RL-shaped.
4. **Tarmogoyf-as-keystone effect.** Coverage is super-additive: clearing a deck's
   *other* missing cards first means one final card (Tarmogoyf) flips dozens of
   decks from unplayable to playable at once. The order you implement in matters.

---

## 5. Caveats / threats to validity (please poke holes)

- **Referee strength.** Everything above uses the cheap VmcMaterial referee. A
  stronger referee (PIMC, or a value that can pilot synergy) might reorder decks
  substantially — we don't yet know how *referee-dependent* the ranking is.
- **No ground truth.** We have not validated any ranking against real tournament
  win-rates. "Gruul Aggro #1 in our Pioneer gauntlet" is a statement about our
  referee + our catalog, not the real metagame.
- **Coverage selection bias** (see §3.5) — the field is not the real metagame; it
  is a non-random subset determined by what we've implemented.
- **Coverage ≠ fidelity.** A deck counted "playable" has every maindeck card
  *registered*, but some are **approximated** (Tarmogoyf = fixed 4/5; several
  modal/triggered riders elided), which systematically weakens the affected
  (often non-aggro) decks. Today "playable" means "resolves", not "faithful" —
  the reports don't yet surface a per-deck fidelity/GAP-card score (a planned
  addition; the approximations are listed in `docs/gauntlet-results/`).
- **Maindeck-only input.** The MTGTop8 converter emits maindecks only, and the
  loader treats unresolved cards over the whole list — fine here, but a future
  source that includes sideboards would need unresolved tracked *per section* so
  a sideboard miss can't disqualify an otherwise-playable maindeck.
- **Sample size.** 84–122 games/deck = screening tier (~±10% CIs); fine for the
  clear top/bottom splits, not for close pairs.
- **Track A caveats still apply** (small mirror-deck samples, identity-free
  features, PIMC's strategy-fusion weakness).

---

## 6. Open questions — where we want your opinion

1. **What makes a deck-strength eval trustworthy?** Given the referee bias, is the
   right move (a) invest in a stronger/faster referee (PIMC at scale; or finally a
   learned value that *can* pilot synergy), (b) validate against real win-rates
   (treat MTGTop8 placements / external meta win% as labels and check rank
   correlation), or (c) accept "strength under referee X" as the definition and
   move on to optimization?
2. **Referee sensitivity.** Cheapest informative experiment: re-run the same field
   under Random / VmcMaterial / PIMC and measure how much the ranking moves. If
   it's stable, the cheap referee is fine; if it reorders, referee quality is the
   whole game. Worth doing first?
3. **Beating selection bias.** Do we (a) keep grinding cards to broaden the field
   toward the real metagame (expensive, open-ended), (b) deliberately implement
   *across* archetypes to get a diverse small field, or (c) restrict claims to
   "best build within a covered archetype" (which the Modern Jund result actually
   does well)?
4. **Is deckbuilding-optimization premature** until (1)–(3) are settled? The
   black-box-optimization loop (CEM/bandits/successive-halving over decklists with
   the cheap referee) is ready to build, but optimizing against a biased objective
   just finds decks that flatter the referee.
5. **Card-identity features**, again: a value head that sees *which* cards are on
   board is the plausible route to a referee that can pilot synergy — still the
   highest-leverage ML change, or still a distraction?
6. **Compute reality check** (unchanged): pure-Rust, single-machine, no GPU. Which
   direction has the best effort-to-payoff?

---

## 6.5 Current plan (the order we're taking it)

1. **Referee sensitivity — first result (done).** Same Pioneer field + seeds
   under different referees (full numbers in
   `docs/gauntlet-results/referee-sensitivity.txt`):
   - vmc ↔ **Random** (64-deck field): Spearman **ρ = 0.71**.
   - vmc ↔ **small-PIMC** (12-deck spanning subset): Spearman **ρ = 0.88**.
   The cheap referee tracks a *stronger* one (0.88) better than it tracks noise
   (0.71), and the residual movement is structured exactly as predicted — vmc
   mildly **over-rates aggro** (Red Deck Wins falls under PIMC; aggro falls under
   random) and **under-rates +1/+1 synergy** (Golgari Scales / Hardened Scales
   rise under both stronger-than-material refs). Effect is modest (~0.10 mean
   point-rate Δ) and partly noisy at 22 games/deck. **Tentative read:**
   optimizing decks against VmcMaterial is *directionally* safe but will mildly
   exploit aggro; confirm with a higher-budget PIMC and larger samples before any
   firm claim. Next still: a larger-budget-PIMC arm + ground-truth check.
2. **Coverage capsules, not broad coverage** — deliberately implement the
   blockers for 4–6 *chosen* archetypes per format, to get diverse experimental
   domains instead of the current Jund/aggro skew.
3. **Constrained deckbuilding inside a capsule** — "optimize under referee X
   within this covered pool/archetype." Narrow, honest, and a useful diagnostic
   (what degenerate decks does the referee reward?).
4. **MTGTop8 placement as *validation*, not training truth** — noisy and
   metagame-confounded; use for rank-correlation sanity checks after (1).
5. **Defer the generic learned value-leaf** — deckbuilding needs a stable
   objective more than another value head; better ML later is
   action-ranking/distillation from search or a deck-level surrogate.

---

## 7. TL;DR

We acted on the deckbuilding pivot and built a **real-deck gauntlet** with proper
paired-seed/bootstrap statistics, then fed it real MTGTop8 decklists. The
unexpected lesson: the hard part was **catalog coverage** (0% of real decks were
playable until we implemented ~62 targeted cards + a Crew engine feature), and
once decks *were* playable, **two confounds dominate the rankings** — the cheap
material referee flatters aggro and penalizes synergy (Pioneer: Gruul Aggro 89% →
Temur Midrange 9%), and coverage selection bias can collapse a "format" to one
archetype (Modern became a 42/43 Jund mirror). The harness is sound and the
rankings are internally consistent, but they are *"strength under our referee,
among the decks we can represent,"* not the real metagame. Our open question for
you: before we build the deckbuilding *optimizer*, how much should we invest in
**referee quality** and **ground-truth validation** so the objective is worth
optimizing — and is referee-sensitivity the first experiment to run?
