# Arcana RL — status & findings (for peer review)

*A self-contained write-up of where our game-playing AI stands, the experiments
we ran, and the open questions. Goal: enough context for an outside researcher
to say "here's what I'd do."*

---

## 1. The domain

**Arcana** is a from-scratch Rust implementation of a Magic: the Gathering–style
trading-card game engine plus a catalog of ~20,500 cards. The engine is a
faithful rules simulator (stack, priority, layers/continuous effects,
replacement effects, combat, triggered/activated abilities, ~105 distinct effect
primitives). Games are:

- **Two-player, zero-sum, turn-based.**
- **Imperfect information** — each player's hand and library are hidden; libraries
  are also stochastic (shuffled). So this is an imperfect-information,
  stochastic game (closer to poker/Stratego than chess/Go).
- **Long horizon & high branching** — games run dozens to hundreds of decision
  points; the legal-action set at a decision can be large (every castable card ×
  target/mode/X-cost combination, plus attacks/blocks/activated abilities).
- **Huge, open-ended action/state space** — 20k cards, and the catalog keeps
  growing. Any agent must generalize across cards it has never specifically been
  trained on.

The AI lives in a crate `arcana-ai` (~6.3k LOC). It is a **value-function RL
pipeline**, not a full AlphaZero-style policy+value network. Everything is pure
Rust (no PyTorch/JAX); the "network" is a small in-house logistic / MLP.

---

## 2. The system

### 2.1 State encoding (`observation.rs`)
A `BasicE2Encoder` turns a game state into **123 floats** (2 × 50 per-player +
20 game-level + 3 perspective), perspective-relative (the to-move player's blocks
come first). Features are **identity-free aggregates** — they go beyond raw
material counts (CMC bins, keyword-presence aggregates) but never name a specific
card:

- per player: life, poison/energy/experience counters, lands-played, mana pool,
  zone sizes (library/hand/graveyard/exile), battlefield permanent counts by
  type (creature/land/artifact/enchantment/PW), tapped-by-type, by-color,
  **total power / total toughness / total damage** on board, an **8-bucket CMC
  histogram** of controlled permanents, and **per-keyword presence** (flying,
  deathtouch, …) on controlled creatures (layer-aware — counts granted keywords).
- game level: turn/phase/step, combat state, stack size, etc.

**Important limitation:** the encoder still has **no card identity** — it can see
the *shape* of the board (a CMC-3 creature with flying) but not *which* card it
is: it cannot tell a Llanowar Elves from a Black Lotus, only "1 creature, power
1, CMC 0." So a learned value can at best learn a function of *aggregate
material/tempo/curve*, which is close to what the hand-tuned heuristic already
encodes.

### 2.2 Policies / search (`search.rs`)
A `StatePolicy` picks an action from a state. A `ValueFn` scores a (state,
player) → [-1, 1] (terminal ±1). Implemented:

- **`MaterialValue`** — hand-tuned leaf: `0.9·tanh((my_material − opp_material)/30)`,
  where material is a board+life heuristic. No learning.
- **`GreedyValuePolicy`** — 1-ply: take the action maximizing a `ValueFn`.
- **`ValueMcPolicy`** — short Monte-Carlo rollouts from each action, leaves scored
  by a `ValueFn` (lookahead; not myopic). Budget e.g. (depth 6, 25 rollouts).
- **`PimcPolicy`** — **Perfect-Information Monte-Carlo**: sample determinizations
  of the hidden information, solve each with flat-MC rollouts, vote. Budget e.g.
  (15 determinizations, 150 rollouts). *No learned component.*
- An IS-MCTS exists but **underperforms PIMC** (consistent with the known
  "PIMC beats IS-MCTS for trick/card games" folklore result).

### 2.3 Learner (`learn.rs`, `mlp.rs`)
Monte-Carlo value learning:

1. `collect_value_data`: play self-play games under a chosen *referee* policy; at
   every visited state, record the encoded features from **both** perspectives,
   labeled with that player's **eventual game outcome** (win=1 / loss=0 /
   draw=0.5). I.e. Monte-Carlo value targets.
2. Standardize features (per-feature mean/var).
3. `train_logistic`: full-batch gradient descent + L2 → win-probability.
   (`LinearValue::value = 2·σ(w·x+b) − 1`, terminal overridden to ±1.)
4. An MLP variant exists (`train_mlp_value`) but isn't the focus below.

### 2.4 Evaluation — the "yardstick"
A **round-robin tournament**: each policy pair plays N games (seats swapped to
cancel first-player bias); report a win matrix + totals. Plus a calibration
module that fits value→win-probability and reports log-loss/Brier/reliability.

All of the above is **mirror-matchup** on a single fixed sampled deck (both
players play the same deck), so results are deck-specific.

---

## 3. Experiments & results (this round)

All runs are `--release`, on the mirror deck. Tournament cells are row-vs-column
wins; totals are out of 3 opponents × N games.

### 3.1 Greedy + value-learned-from-greedy-material self-play
Learned from **24** greedy-material self-play games; round-robin 12 games/pair:

```
ranking: pimc(32) > random(25) > g-learned(15) > g-material(0)   [/36]
```
- **`g-material` = 0/36 — loses to *random* 0–12.** A 1-ply material maximizer
  never attacks (attacking spends material for no immediate gain), so it durdles
  to a deck-out/timeout loss. → MaterialValue is a useful *search leaf* but a
  pathological *greedy policy*.
- `g-learned` (15) beats its teacher 12–0 but loses to random — it learned from
  degenerate data.

### 3.2 Value as a *search leaf* (ValueMc), learned from random self-play
Learned from **30** random self-play games; value deployed inside `ValueMcPolicy`;
10 games/pair:

```
ranking: pimc(20) > vmc-material(17) > random(12) > vmc-learned(11)   [/30]
```
- **`vmc-material` ≈ `pimc`** (17 vs 20; tied 5–5 head-to-head) — a material leaf
  inside lookahead is already near top. *Search does the work.*
- **`vmc-learned` is worst** — the learned leaf is *worse* than the hand-tuned one.

### 3.3 Diagnostic: is the learned value a good static predictor?
On 3,264 mid-game self-play states, learned value (from 30 games) vs material:

```
learned : std 0.568  range [-1.000, +0.999]
material: std 0.427  range [-0.897, +0.868]
corr(learned, material) = 0.630
log-loss  learned = 0.7165   material = 0.5408   (ln2 = 0.693 = chance)
```
- The learned value is **NOT flat** — it's *more* spread than material (confident
  to ±1) **but predicts the outcome WORSE THAN CHANCE out-of-sample** (0.72 >
  0.69), while material is genuinely informative (0.54). → **overfit /
  overconfident.**
- **Root cause:** ~30 games, but states within a game share one outcome label →
  *effective* independent labels ≈ 60, against ~120 features → underdetermined.

### 3.4 Learning curve: does more data fix the predictor?
Out-of-sample log-loss on 3,476 held-out fresh-game states:

```
baseline (always 0.5)  0.6930
material               0.5419
learned n= 50          0.6015
learned n=150          0.5133   ← beats material
learned n=300          0.5269   ← beats material
```
- **Yes** — by ~150 games the learned value is a *better outcome predictor* than
  the hand-tuned heuristic. Crossover ~150; diminishing past it (300 ≈ 150,
  possibly wanting LR/epochs scaled with n).

### 3.5 Confirmation: does the better predictor play better?
Re-ran 3.2's tournament with the leaf trained on **150** games (not 30):

```
ranking: pimc(20) > vmc-material(18) > vmc-learned(11)   [/30]
  (vmc-material beats vmc-learned head-to-head 6–4)
```
- **No.** More data fixed the *predictor* (log-loss 0.51 < 0.54) but **NOT the
  *player*** — `vmc-learned` is essentially unchanged (11 vs 18), still loses to
  the material leaf.

---

## 4. What we concluded

1. **"Good predictor ≠ good search leaf."** Lower aggregate log-loss did not
   translate into stronger play. Play strength depends on the leaf's *action-
   ordering* at the states actually reached during search, not its average
   accuracy on random-play states.
2. **Search is the lever, not the learned leaf.** PIMC dominates everything (20);
   a material leaf in lookahead ≈ PIMC (18); the learned value adds nothing for
   play. The pragmatic deployed AI is PIMC (or ValueMc-material) — *zero learning
   required.*
3. The learned-value-as-leaf branch is **parked** as the path to a stronger
   agent. (The pipeline is now guarded by a fast smoke test + two diagnostics so
   it doesn't bit-rot.)

---

## 5. Caveats / threats to validity (please poke holes)

- **Small samples:** 10–12 games/pair. Differences are *directional*, not firm;
  several head-to-heads (e.g., 4–6) are within noise of 5–5.
- **Single mirror deck.** No deck/matchup diversity; conclusions may be
  deck-specific. (We do have a `deckeval` gauntlet but didn't use it here.)
- **Self-referee win-probability.** MC targets are "P(win | *this referee* plays
  on from here)", not P(win | optimal). Value learned under random/greedy
  self-play is calibrated to a weak continuation.
- **Train/deploy distribution mismatch.** The value is trained on random-play (or
  greedy-material) states but deployed on ValueMc-rollout states.
- **Identity-free features.** The encoder sees structural aggregates (counts,
  CMC bins, keyword presence) but no card identity — a structural ceiling on
  what any value head can learn beyond aggregate material/tempo/curve.
- **Linear model.** Logistic regression; an MLP variant exists but wasn't pushed
  (and on ~60 effective samples, more capacity would overfit harder).
- **PIMC's known weakness ("strategy fusion"/non-locality)** — PIMC assumes
  perfect information per determinization, so it can misplay genuine
  information-hiding/bluff lines. It still wins here, but that's a theoretical
  ceiling.

---

## 6. Open questions — where we want your opinion

1. **Is the learned value worth pursuing at all**, given material-in-search ≈
   PIMC for free? Or is "make the search better/faster" the whole game?
2. **If we pursue learning, how do we optimize for *play* not log-loss?**
   Candidates we're weighing: (a) train the value on *deployment-distribution*
   states (states reached by the search, à la DAgger/expert-iteration), (b) a
   real **policy network + PUCT (AlphaZero-style)** rather than value-as-leaf,
   (c) policy-gradient / regret-based methods suited to imperfect info
   (DREAM/ReBeL/Player-of-Games lineage), (d) just distill PIMC's action choices
   into a fast policy net (imitation) to remove PIMC's per-move search cost.
3. **Features:** is adding **card-identity embeddings** (so the value can see
   *what's* on board, not just counts) the highest-leverage change — or a
   distraction given (1)?
4. **Imperfect info:** PIMC vs IS-MCTS vs an explicit imperfect-info solver — for
   a game this large/long, what's the realistic target?
5. **Eval rigor:** what sample sizes / deck diversity / opponent pool would you
   require before trusting a "policy A > policy B" claim here?
6. **Compute reality check:** this is pure-Rust, single-machine, no GPU, no
   external ML stack. Given that constraint, which direction has the best
   effort-to-payoff?

---

## 7. TL;DR

We have a working imperfect-information card-game engine and a value-function RL
stack. **PIMC (search, no learning) is by far the strongest agent; a hand-tuned
material heuristic inside lookahead nearly matches it.** A self-play-learned value
function is currently *worse* as a search leaf than the hand-tuned heuristic.
We diagnosed the obvious cause (overfit from too few games) and *fixed the
predictor* (it now beats the heuristic on held-out log-loss) — **but that did not
make it play better.** So our current read is: **search is the lever; the learned
value is a research rabbit hole** unless we change the objective to optimize play
directly (expert-iteration / policy net / imitating PIMC). We'd love opinions on
whether that read is right and which direction you'd take.
