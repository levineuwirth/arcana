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

- **Referee strength.** Everything in §§3–4 uses the cheap VmcMaterial referee. A
  stronger referee (PIMC, or a value that can pilot synergy) might reorder decks
  substantially. **Answered in §6.5:** the ranking IS strongly referee-dependent
  at the extremes (aggro/synergy), and optimizing against VmcMaterial reward-hacks.
- **No ground truth.** The §§3–4 rankings are not validated against real tournament
  results — "Gruul Aggro #1 in our Pioneer gauntlet" is a statement about our
  referee + our catalog, not the real metagame. **Answered in §6.5 (4):** now
  checked — VmcMaterial has ρ≈0 vs real MTGTop8 finishes (it *fails* this check);
  a direct PIMC-vs-real check is still outstanding.
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

> **These were the refresh-1 framing.** Questions 1, 2, and 4 are largely
> **answered in §6.5** (referee sensitivity + ground-truth check ran; deckbuilding
> optimization *was* premature against the biased objective) — kept here for
> provenance. Questions 3, 5, 6 are still live.

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

1. **Referee sensitivity — first result (done).** Two *separate* rank-correlation
   checks (full numbers in `docs/gauntlet-results/referee-sensitivity.txt`):
   - vmc ↔ **Random**, on a **64-deck** Pioneer field (122 vs 122 games/deck):
     Spearman **ρ = 0.71**.
   - vmc ↔ **small-PIMC**, on a **12-deck** spanning subset (44 vs 22 games/deck):
     Spearman **ρ = 0.88**.
   *Provenance caveat:* the Random arm's 64-deck playable snapshot predates the
   headline `pi_gauntlet_62.csv` (62-deck) snapshot — the two coverage scans
   differ by a couple of decks, and the 64-deck arm's per-deck CSVs are only
   summarized in the `.txt`, not checked in (a reproducibility gap to close).
   The cheap referee tracks the *stronger* small-PIMC (0.88) more closely than it
   tracks *random noise* (0.71), and the residual movement is structured as
   predicted — vmc mildly **over-rates aggro** (Red Deck Wins falls moving off
   material; aggro falls under Random) and **under-rates +1/+1 synergy** (Golgari
   Scales / Hardened Scales rise under **both non-material referees**, Random *and*
   PIMC). The effect looked modest here (~0.10 mean point-rate Δ, noisy at 22
   games/deck), so the tentative read ("directionally safe, mildly aggro-
   exploiting") was **too generous** — the higher-budget-PIMC arm (3) and the
   ground-truth check (4) found the misalignment is severe, not mild.
2. **Coverage capsules, not broad coverage** — deliberately implement the
   blockers for 4–6 *chosen* archetypes per format, to get diverse experimental
   domains instead of the current Jund/aggro skew. (Built one honest Pioneer
   capsule — 5 archetypes, 57-card pool — `docs/capsule-pioneer/`.)
3. **Constrained deckbuilding inside a capsule — done; the objective is
   MISALIGNED.** A (1+1) hill-climb over the capsule maximized the VmcMaterial
   gauntlet score (fitness 0.04 → 0.52) and converged to a *recognizable* deck
   (23 lands, avg MV 2.24, a Gruul-ish "good-stuff" pile: the individually
   strongest bodies + planeswalkers from across all 5 archetypes). But the
   reviewer's validation arm — score that deck + each seed under a higher-budget
   PIMC (samples 16, cap 160) — shows the VMC gain doesn't just evaporate, it
   **inverts**: the optimized deck is the *single worst* deck in the field under
   PIMC (point-rate 0.133, below every seed ≥0.21), while the synergy seed the
   material referee buried (Hardened Scales) *rises* (0.29 → 0.50). Textbook
   reward-hacking: the pile maxes raw material but a referee that sequences games
   punishes its incoherence + 3-color mana base. Full table:
   `docs/capsule-pioneer/results.txt`.
   - **Referee v2 A/B (the "hand-tune a cheap value" fix) — a split verdict.** A
     card-advantage-rebalanced material leaf (creatures P+T not 2P+T; hand cards
     ×2; PW 2+loyalty; `MaterialValueV2`) is a **much better JUDGE**: mean-abs-error
     vs PIMC on the 5 seeds *halves* (0.175 → 0.092), fixing the two worst v1 biases
     (UW Spirit over-rating 0.75→0.46 = PIMC exactly; Hardened Scales under-rating
     0.29→0.42 toward PIMC 0.50). But it is **still exploitable as a TARGET**: a
     hill-climb against v2 scores 0.43 under v2 yet only 0.20 under PIMC (still
     near-bottom), and the exploit merely *relocated* — v2's hand-card weight pushed
     the optimizer to cut lands (23→19) and jam a 4-color pile. Textbook Goodhart:
     improving the cheap referee helps *evaluation* but a single static cheap target
     stays gameable. **Take:** adopt v2 as the gauntlet judge; for optimization use
     PIMC-in-the-loop / PIMC-distilled, or a referee *ensemble* + PIMC hold-out —
     not any single cheap proxy.
4. **MTGTop8 placement as *validation* — done; VmcMaterial fails it.** Per-archetype
   real strength from the dump's tournament finishes (`player_result` over 528
   Pioneer events) vs our gauntlet point-rate, collapsed into 7 confidently-matched
   archetype buckets: **Spearman ρ = 0.00** (mean finish) / **−0.39** (top-8 share,
   sharper). The extremes invert — our #1 Gruul/RG Aggro is real-world *weakest*
   (0.000 top-8 share in ≥2-star events), our worst Scales-synergy cluster is
   real-world mid-to-strong — and our referee manufactures a 0.30–0.82 spread where
   reality compresses to ~0.51–0.60. Caveats (N=7 fuzzy buckets, narrow real band,
   metagame confound) in `docs/gauntlet-results/gauntlet-vs-real-PI.txt`.
   Together (3) and (4) establish that **VmcMaterial fails the real-world check**
   (ρ≈0) and that **PIMC disagrees with VmcMaterial in the more plausible
   direction** on the key Scales/aggro cases (synergy up, the aggro pile down).
   They do **not** show PIMC *itself* correlating with real tournament buckets —
   and the direct check (6a) confirms **PIMC also fails it** (ρ ≈ −0.09 / −0.49 on
   the matched subset): less aggro-biased than vmc, but not aligned with real. So
   the binding constraints are *referee quality **and** card fidelity **and** field
   representativeness* together — not optimizer sophistication, and not the referee
   alone.
5. **Defer the generic learned value-leaf** — deckbuilding needs a stable
   objective more than another value head; better ML later is
   action-ranking/distillation from search or a deck-level surrogate.
6. **Objective validation.**
   (a) **Direct PIMC-vs-real check — done (first cut); PIMC ALSO fails.** On the
   *same* 12-deck Pioneer subset scored under both referees, Spearman vs real
   MTGTop8 strength: vmc **+0.26 / −0.03** (mean-finish / top-8); small-PIMC
   **−0.09 / −0.49** — no better than vmc, arguably worse. small-PIMC is *less
   aggro-biased* than vmc (it nudges synergy up: Golgari/Scales 0.30→0.36) but its
   absolute ranking still over-rates aggro (its #1 Gruul/RG Aggro is real-world
   *weakest*). So **"PIMC tracks reality" is false even directly** — the residual
   bias implicates **card fidelity** (approximated non-aggro payoffs weaken
   synergy/control *regardless* of referee) + **field confound**, not only referee
   strength; referee quality is *necessary but not sufficient*. Caveat: small-PIMC
   (8/80) is under-powered vs the capsule's 16/160.
   `docs/gauntlet-results/pimc-vs-real-PI.txt`.
   - **Fidelity-controlled control set — attempted; the field COLLAPSES.** To
     separate referee-bias from card-fidelity-bias, restrict to Pioneer decks with
     *no* GAP-approximated cards. Of 800 dumped / 65 fully-covered decks: **0 are
     zero-GAP (strict)**, and only **4** survive even a relaxed tier that allows the
     low-impact shocklands (3 archetypes: Gruul/Azorius Aggro + Hardened Scales). So
     **100% of the covered field is approximated** — fidelity-bias contaminates every
     gauntlet result, and no clean control set exists on this corpus. The blocker
     histogram is the key: after shocklands (highest count, lowest impact), the
     load-bearing approximations are **synergy payoffs** — Hangarback Walker (19
     decks), Supreme Phantom / Empyrean Eagle anthems (9/8), Soul-Scar Mage, Torbran
     — i.e. exactly the non-aggro payoffs whose elision weakens synergy decks under
     *any* referee. This is the plausible **mechanism** behind the direct-check
     failure: even PIMC over-rates aggro vs real partly *because the synergy payoffs
     are GAP'd*. `docs/gauntlet-results/fidelity-subset-PI.txt`.
   - **Fidelity grind + payoff check — fidelity helps where it *mechanically can*,
     but VMC still fails.** A targeted grind recovered the synergy payoffs (Hangarback
     death-Thopters + counter-pump, Supreme Phantom / Empyrean Eagle anthems, Winding
     Constrictor, Smuggler's Copter loot, Fatal Push Revolt + a new
     `permanent_left_battlefield_this_turn` accessor) — un-collapsing the relaxed field
     **4 → 32 decks / 3 → 15 archetypes**. Re-running VMC-vs-real on that fidelity-fixed
     field: **ρ = −0.40 / −0.20** (mean/top-8, N=4) — no global improvement over the
     0.00–0.26 baselines. But the *decomposition* is the result: the fix brought the
     synergy whose payoff is **immediate board material** into alignment (UW Spirit
     anthems: vmc 0.65 ≈ real 0.59), while **grindy** synergy stayed under-rated
     (Golgari Scales 0.37 vs real 0.54 — executes now, but 4-rollout material can't
     pilot it) and **aggro over-rating** was untouched (Gruul 0.78 vs 0.51). So the two
     confounds separate by mechanism: **fidelity** governs material-visible synergy;
     **referee strength** governs grindy synergy + aggro calibration. Caveat: N=4,
     synergy-heavy field. `docs/gauntlet-results/fidelity-vmc-vs-real-PI.txt`.
   (b) **Pragmatic deckbuilding loop = cheap candidate generation + PIMC
   selection/hold-out**, not optimization against any single cheap scalar (the v2
   A/B showed even an improved cheap referee stays gameable). **First result
   (`deckbuild_capsule_pimc_select`, `docs/capsule-pioneer/pimc-select.txt`):
   selection helps, with a ceiling.** Cheap-gen (6 candidates, v1 & v2 leaves ×3
   seeds) yields *high-variance-under-PIMC* decks (PIMC 0.12–0.52); the cheap v2
   score does not rank them by PIMC (two v2=0.48 decks differ by 0.16 under PIMC),
   so a PIMC finalist pass recovers a coherent, mid-field deck (0.52) the
   single-shot optimizer missed (A/B decks were 0.13/0.20) — an aggressive Boros
   midrange that reads like real Magic, *not* an exploit pile. But even the
   PIMC-selected best (0.52) sits below the top seeds (RDW 0.75); the loop buys
   "mid-field competitive," not "beats the best real decks." Next lever: widen the
   candidate pool (selection value scales with it) and/or guide generation. Caveat:
   5 games/opp is noisy and the headline +0.16 is partly PIMC breaking a cheap-judge
   *tie* (expected ~+0.08).

---

## 7. TL;DR

We acted on the deckbuilding pivot and built a **real-deck gauntlet** with proper
paired-seed/bootstrap statistics, then fed it real MTGTop8 decklists. The
unexpected lesson: the hard part was **catalog coverage** (0% of real decks were
playable until we implemented ~62 targeted cards + a Crew engine feature), and
once decks *were* playable, **two confounds dominate the rankings** — the cheap
material referee flatters aggro and penalizes synergy (Pioneer: Gruul Aggro 89% →
Temur Midrange 9%), and coverage selection bias can collapse a "format" to one
archetype (Modern became a 42/43 Jund mirror).

We then built one honest Pioneer **capsule** and closed the deckbuilding loop —
and the loop *diagnosed its own objective*. Three results converge: the
optimized deck (good under VmcMaterial) is the **worst** deck in its field under
a higher-budget **PIMC** (point-rate 0.13 vs every seed ≥0.21); the VmcMaterial
gauntlet has **zero-to-negative rank correlation with real MTGTop8 finishes**
(ρ = 0.00 mean-finish, −0.39 top-8); and while **PIMC is less aggro-biased than
VmcMaterial** (it nudges the buried synergy decks up), a *direct* PIMC-vs-real
check shows **PIMC also fails the real-world correlation** (ρ ≈ −0.09 / −0.49) —
so referee quality alone is not the fix.

The takeaway sharpened: it is **not** that VmcMaterial is "directionally right
but noisy" — optimizing against it is reward-hacking, and the rankings are
*"strength under a referee that does not track reality."* But the direct
PIMC-vs-real check reframes the fix: **referee quality is a binding constraint,
not the only one** — **card fidelity** (approximated non-aggro payoffs) and
**field representativeness** bind too, since even PIMC over-rates aggro vs real.
The concrete next step is **objective validation, not more optimization**: a
*higher-budget* PIMC-vs-real check + a fidelity-controlled deck set (small and
expensive is fine) — because the central claim above depends on it. After that,
the pragmatic deckbuilding loop is **cheap candidate generation + PIMC
selection/hold-out**, not optimization against any single cheap scalar — the v2
A/B (§6.5 (3)) showed that even an *improved* cheap referee stays gameable as a
target. The `deckbuild_capsule_pimc_select` experiment (§6.5 (6b)) tested exactly that:
**selection helps, with a ceiling** — a PIMC finalist pass recovers a coherent
mid-field deck the single-shot optimizer missed, but not one that beats the top
seed decks. Widening the candidate pool is the lever.
