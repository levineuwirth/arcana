# Arcana Roadmap

*Snapshot: June 2026. Engine 990 unit + 126 integration tests; catalog 8,416 cards
(26.2% of the 30,881 vintage-legal oracle corpus); behavioral allowlist 274;
random-game harness 1,100 games / 0 failures.*

This document captures the full scoping pass: where the project stands on
catalog coverage, engine completeness, multiplayer, and RL — and the ordered
plan for each track.

---

## 1. Where we stand

### Catalog (8,416 / 30,881 = 26.2%)

| In catalog | Count | | Missing (vintage-legal) | Count |
|---|---|---|---|---|
| Plain creatures | 5,393 | | Creatures | 11,740 |
| Pure instants/sorceries | 2,411 | | Enchantments | 3,065 |
| Transform DFCs | 342 | | Instants | 2,258 |
| Sagas | 211 | | Artifacts | 2,245 |
| Adventures | 134 | | Sorceries | 2,156 |
| MDFCs | 45 | | Lands | 1,053 |
| Battles / Classes | 38 / 37 | | Planeswalkers | 288 |
| **Plain enchantments** | **6** | | | |
| **Non-creature artifacts** | **53** | | | |
| **Lands** | **38** | | | |
| **Planeswalkers** | **15** | | | |

Two structural facts drive the plan:

1. **98.2% of the gap is layout-"normal" cards.** Typed layouts are nearly
   exhausted (4 Sagas and 17 Adventures remain in the entire game).
2. **The pipeline has no PromptShape for enchantments, non-creature artifacts,
   lands, or planeswalkers** — their near-absence is by construction, not
   difficulty. And of the missing creatures, **9,946 (86%) are T4-multiline**
   (2+ ability lines), which the pipeline refuses at the tier gate.

Catalog quality: 55.6% of card files are fully faithful (zero GAP);
~500–550 pure-stub resolvers; top debt family is **transform/MDFC back-face
abilities (~160)**, then missing Effect variants (~64), Battle defeat-cast (28).
Many GAP comments are stale (claim primitives that now exist) — a re-sweep is
cheap yield.

### Engine

47,460 lines, 28 modules. Complete: full turn structure, priority/APNAP, stack
(copies/modal/split + CR 608.2b rechecks), combat (first strike, trample
overflow, deathtouch, PW/battle attacks), **all 11 layers**, 13 replacement
kinds, full SBA list, full mana system (hybrid/Phyrexian/snow, X, two-tier
solver), London + Vancouver mulligans, day/night, dungeons, emblems,
energy/poison, every typed-card shape, ~50 enforced keywords,
flashback/madness/kicker/cycling/cascade/storm/convoke/delve/improvise.

Not implemented: morph/foretell/escape/overload/emerge/prowl/spectacle/
evoke/dash/surge/mutate cast paths; generic cost modifiers (Affinity declared,
unwired); Shroud targeting; attacking-side banding; Specialize face swap;
**aura cast-resolution attach** (is_aura + SBA exist; nothing attaches on
resolution — gates the Aura shape); crew/Vehicles; monarch/initiative;
commander rules (scaffolding only).

### Multiplayer

N-player skeleton is honest (parametric state, APNAP, attack-any-opponent,
each-opponent effects, clean card code). Five blockers:
1. DeclareBlockers decision hardwired to `(active+1) % N` (engine.rs:4173) and
   `apply_declared_blockers` never checks blocker controller == defender.
2. No CR 800.4a elimination — dead players keep taking turns/priority.
3. `GameResult::Eliminated` never constructed.
4. `DiscardChoice::OpponentChooses` picks `(p+1) % N`.
5. Monarch/initiative/commander absent; goad's "attacks if able" half missing.
No integration test plays >2 players.

### RL

2-player is genuinely ready: closed `legal_actions → step` loop (1,100-game
proof), seeded-replayable ChaCha8 RNG with a determinism test, clone-based
rollouts, **arcana-ai** (Policy trait, N-policy `run_episode`, info-set
projection, 99-float v0 encoder, terminal reward) and **arcana-py** (PyO3
`run_episode` + training.py + REINFORCE demo). Gaps: `MtgEnv.step()` stub,
full-GameState serde (14 fn-pointer sites; action trace serializes),
canonical-only enumeration for ordering/distribution choices, unmeasured
throughput (spec target >20k games/s).

---

## 2. Catalog breadth plan (CURRENT TRACK)

Sub-shape census of the missing normal-layout cards:

| Bucket | Count | Engine status | Wave |
|---|---|---|---|
| Triggered enchantments (1-line + multi-trigger) | ~1,050 | ready (TriggeredAbilityDef on enchantment chars) | **1** |
| Activated/mana artifacts (non-Equipment) | ~830 + 140 triggered | ready (mirrors ActivatedAbilityCreature; mana rocks trivial) | **1** |
| Lands (pure mana 349, mana+1-extra 573) | ~920 | ready (ManaRestrictions, EntersWithSpec::Tapped, ActivationZone) | **1** |
| Equipment | 551 | ready (`with_equip` + attached_pt; bonesplitter touchstone) | **1** |
| Static enchantments (anthem-like) | ~280 | ready (glorious_anthem ETB-install pattern) | **1** |
| Auras | 1,201 | **engine task: resolution attach + Enchant targeting** | 2 |
| Planeswalkers | 282 | machinery proven (15 in catalog); needs classifier route before the T4 gate + pack | 2 |
| **T4-multiline creatures** | **9,946** | engine handles multi-ability cards (this session's batches prove it); needs a T4 multi-ability shape + decomposed prompt | **3** |
| T4-multiline spells | 2,228 | same | 3 |
| T2/T3 creature/spell tail (unattempted or failed) | ~2,750 | re-run / failure triage | 3 |
| Modal + X spells | ~850 | ModalSpec + x_value exist; needs prompt support | 3 |
| Vehicles (crew) | 166 | engine task: Crew | 4 |
| Mixed-ability permanents, flip/leveler/mutate/meld/prototype layouts | ~600+ | per-layout work | 4 |

**Wave 1 ≈ 3,950 addressable cards with zero engine work** — new PromptShapes
only: `TriggeredEnchantment`, `ActivatedArtifact`, `UtilityLand`, `Equipment`,
`StaticEnchantment`. Recipe per shape: PromptShape variant + `select_shape`
route (+ classifier tweak where needed) + few-shot pack (2–3 catalog exemplar
cards via `include_str!`, hand-writing seed exemplars where the catalog lacks
them) + semantic-gate entry + pilot (~30 cards) → measure → scale fleet.

Wave 2 unblocks ~1,500 more with two bounded engine tasks (aura attach;
PW routing). Wave 3 is the big prize (~15,800) and is mostly prompt
engineering: a multi-ability T4 shape that emits N ability defs.

At full Wave 1–3 execution the catalog ceiling is roughly 28k of 30.9k
(~90%); the realistic near-term target after Wave 1 + pilots is ~12–13k.

## 3. Multiplayer plan

1. Correctness: per-defender blocking (fix engine.rs:4173 + controller check),
   CR 800.4a elimination (objects leave, spells fizzle, skip turns/priority),
   construct `GameResult::Eliminated`.
2. Gate: extend the random-game harness with 3–4 player profiles.
3. Features: chosen-opponent staging (discard etc.), goad attack-requirement,
   monarch, commander rules (tax, identity, 21-damage SBA — scaffolding
   exists), initiative later.

## 4. RL plan

1. Implement `MtgEnv.step()` (thin — `run_episode` already does the work).
2. Run the bench suite; record games/s, clone, legal-action latency against
   spec §19 targets.
3. Decide canonical-vs-complete enumeration policy for
   OrderCards/PickCards/Distribute choices (document as part of the env
   contract).
4. Encoder v1 (card-specific features, 1500–2500 floats per spec).
5. Longer-term: `ConditionFnId` migration to unblock full-state serde.

## 5. Engine-depth backlog (Phase-3 continuation, all small)

Spell-ordinal accessor (second_guess), cascade grants + next-spell cost
reductions, cast-from-graveyard permissions, self-exclusion ("another X")
filters, for-as-long-as durations, put-with-rider continuations
(hans_eriksson fight / winota indestructible), The Ring tempts you (13 cards),
at-end-of-combat timing (15), doesn't-untap statics (9), stale-GAP re-sweep.
