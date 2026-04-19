# Arcana Engine — Keyword & Keyword-Action Index

A living punch list of every Magic: The Gathering keyword ability (CR 702)
and keyword action (CR 701), with implementation status. The goal is full
parity with paper MTG; this file tracks the distance to that goal.

**Status legend:**

- ✅ **Wired** — enum variant present *and* behaviorally honored in the engine.
- 🟡 **Enum-only** — variant exists in `KeywordAbility` but not consulted by
  the engine. Card effects can reference it; no mechanical consequence.
- 🟠 **Partial** — wired with known Phase 1 simplifications (deterministic
  policy in place of an agent decision, etc.).
- ❌ **Missing** — no representation at all.
- — **Expressible via primitives** — not a dedicated enum/Effect variant,
  but can be composed from existing primitives (`CreateToken`,
  `AddCounters`, etc.).
- **N/A** — niche variant (Un-set, Planechase, digital-only, etc.) we
  don't plan to support in the near term.

Last updated: after Phase 2 priority batch (Protection / Attach / Ward /
Prowess / Regenerate / Manifest). **615 unit tests green.**

---

## Evergreen keyword abilities (CR 702)

| Keyword | CR | Status | Notes |
|---|---|---|---|
| Deathtouch | 702.2 | ✅ | `has_deathtouch_damage` flag; SBA & trample interactions |
| Defender | 702.3 | ✅ | `legal_actions::can_attack` rejects |
| Double strike | 702.4 | ✅ | Two-pass damage in `combat::deal_damage_pass` |
| Enchant | 702.5 | 🟡 | Enum variant; Attach primitive exists but aura-enters-as-attached not auto-wired |
| Equip | 702.6 | 🟡 | Enum variant; Attach primitive exists but activated-ability wiring is per-card |
| First strike | 702.7 | ✅ | Two-pass damage with `has_first_strike` gate |
| Flash | 702.8 | ✅ | `legal_actions` bypass for sorcery-speed gate |
| Flying | 702.9 | ✅ | Block filter in `legal_actions::can_block_attacker` |
| Haste | 702.10 | ✅ | Summoning-sickness override in `can_attack` |
| Hexproof | 702.11 | ✅ | `TargetRequirement::matches_choice` rejects opponent targets |
| Indestructible | 702.12 | ✅ | SBA + `Effect::DestroyPermanent` both honor |
| Lifelink | 702.15 | ✅ | Post-replacement in `deal_damage` |
| Menace | 702.110 | ✅ | Block filter — singleton blocker rejected |
| Protection | 702.16 | 🟠 | `ProtectionQuality` enum wired for color/anycolor/type/everything. Damage, attach, block all honor. Targeting honors `Everything` fully; color-specific targeting needs source-of-spell threading (TODO) |
| Prowess | 702.108 | ✅ | Auto-fires in `collect_pending_triggers` on noncreature cast by controller |
| Reach | 702.17 | ✅ | Combined with Flying in block filter |
| Trample | 702.19 | ✅ | `trample_damage_distribution` + DT interaction + dead-blocker overflow |
| Vigilance | 702.20 | ✅ | Skip tap in `apply_declared_attackers` |
| Ward(cost) | 702.21 | 🟠 | Deterministic policy: opponent spells rejected at target check (same as hexproof). TODO: real trigger + agent decision to pay cost |

---

## Keyword actions (CR 701)

| Action | CR | Status | Notes |
|---|---|---|---|
| Activate | 701.1 | ✅ | `Action::ActivateAbility` pipeline |
| Attach | 701.2 | ✅ | `Effect::Attach`; protection-aware |
| Cast | 701.3 | ✅ | `apply_cast_spell`; `Effect::CastFromHandFree`, `CastFromGraveyard` |
| Counter | 701.4 | ✅ | `Effect::Counter` + stack pop |
| Create | 701.5 | ✅ | `Effect::CreateToken` |
| Destroy | 701.6 | ✅ | `Effect::DestroyPermanent` (respects Indestructible) |
| Discard | 701.7 | ✅ | `Effect::Discard` (Phase 1 first-card policy) |
| Double | 701.8 | ❌ | Expressible via `Effect::DoubleDamage` replacement but no dedicated primitive |
| Exchange | 701.9 | ❌ | |
| Exile | 701.10 | ✅ | `Effect::ExilePermanent`, `ExileFromGraveyard` |
| Fight | 701.11 | ✅ | `Effect::Fight` |
| Mill | 701.12 | ✅ | `Effect::Mill` |
| Play | 701.13 | ✅ | `Action::PlayLand` + Cast pipeline |
| Reveal | 701.14 | 🟠 | `TutorToHand { reveal: true }` marks `known_cards`; no standalone `Effect::Reveal` primitive |
| Sacrifice | 701.15 | ✅ | `Effect::Sacrifice` (Phase 1 first-match policy) |
| Scry | 701.16 | 🟠 | `Effect::Scry` — Phase 1 keep-on-top; TODO agent reorder/bottom decision |
| Search | 701.17 | ✅ | `Effect::Search`, `TutorToHand`, `TutorToBattlefield` |
| Shuffle | 701.18 | ✅ | `Effect::Shuffle` + `GameState::shuffle_library` |
| Tap | 701.19 | ✅ | `Effect::Tap` |
| Untap | 701.20 | ✅ | `Effect::Untap` |
| Fateseal | 701.21 | ❌ | Scry-variant targeting opponent |
| Clash | 701.22 | ❌ | Lorwyn mechanic |
| Transform | 701.23 | ✅ | `Effect::Transform` |
| Vote | 701.24 | ❌ | Council's dilemma — needs agent decision |
| Regenerate | 701.25 | ✅ | `Effect::Regenerate` + `ReplacementKind::RegenerateShield` + SBA wiring |
| Planeswalk | 701.26 | N/A | Planechase only |
| Set in Motion | 701.27 | N/A | Archenemy only |
| Proliferate | 701.28 | 🟠 | `Effect::Proliferate` (greedy max — every eligible permanent/player bumped). TODO agent-choice variant |
| Populate | 701.29 | ❌ | Expressible via `CopyPermanent` on a token |
| Monstrosity | 701.30 | ❌ | Per-card state transition |
| Meld | 701.31 | ❌ | Two cards become one |
| Manifest | 701.32 | ✅ | `Effect::Manifest`; face-down 2/2 |
| Support | 701.33 | ❌ | Multi-target counter distribution |
| Investigate | 701.34 | — | Expressible via `CreateToken` (Clue) |
| Bolster | 701.35 | ❌ | Counter on lowest-toughness creature |
| Amass | 701.36 | ❌ | Zombie Army counter accumulation |
| Explore | 701.37 | ❌ | Ixalan creature mechanic |
| Goad | 701.38 | ✅ | `Effect::Goad`; defender-filter honored, must-attack TODO(agent hint) |
| Assemble | 701.39 | N/A | Un-set (Contraptions) |
| Surveil | 701.40 | 🟠 | `Effect::Surveil` — Phase 1 all-to-graveyard; TODO per-card decision |
| Adapt | 701.41 | ❌ | Per-card state transition |
| Venture into the dungeon | 701.43 | ❌ | Forgotten Realms |
| Reconfigure | 701.44 | ❌ | Kamigawa: Neon Dynasty |
| Learn | 701.45 | ❌ | Strixhaven |
| Connive | 701.49 | ❌ | Streets of New Capenna |
| Incubate | 701.52 | ❌ | March of the Machine |
| Collect Evidence | 701.55 | ❌ | Murders at Karlov Manor |
| Suspect | 701.56 | ❌ | Murders at Karlov Manor |
| Forage | 701.57 | ❌ | Bloomburrow |
| Plot | 701.58 | ❌ | Outlaws of Thunder Junction |
| Saddle | 701.59 | ❌ | Outlaws of Thunder Junction |
| Solve a case | 701.61 | ❌ | Murders at Karlov Manor |
| Discover | 701.62 | ❌ | Cascade variant |
| Time Travel | 701.63 | ❌ | Doctor Who |
| Bargain | 701.64 | ❌ | Wilds of Eldraine |

### Keyword actions not on CR 701 but frequently needed
| | | | |
|---|---|---|---|
| Shuffle-hand-into-library | CR 103.4 | ✅ | Mulligan flow |
| Draw | CR 121 | ✅ | `Effect::DrawCards` |
| Pay life | CR 118.8 | — | `LoseLife` primitive |
| Gain life | CR 119 | ✅ | `Effect::GainLife` |
| Phase out / Phase in | 702.25 | ❌ | `PermanentStatus::phased_out` field exists; no Effect or enforcement |
| Flip coin / Roll die | 705/706 | ❌ | |

---

## Non-evergreen keyword abilities (CR 702.x)

The Wikipedia list enumerates ~120 non-evergreen keywords. Most are
per-set mechanics expected to land with their card sets.

### In enum (`KeywordAbility::*`), not wired
- `Convoke` — wired as `AdditionalCostPayment::Convoke` in cast pipeline
- `Delve` — wired as `AdditionalCostPayment::Delve`
- `Flashback(cost)` — wired as `CastModifier::Flashback` (alt-cost)
- `Kicker(cost)` — wired as `AdditionalCostPayment::Kicker`, kicked flag on StackEntry
- `Affinity(SubtypeFilter)`, `Equip(cost)`, `Enchant(filter)`,
  `Cycling(cost)`, `Madness(cost)`,
  `Morph(cost)`, `Manifest` (as a keyword vs. the action primitive),
  `Surveil(n)`, `Explore`, `Adapt(n)`, `Foretell(cost)`, `Learn`,
  `Connive`, `Discover(n)`, `Bargain`, `Offspring(cost)`,
  `Impending { mana_cost, time_counters }`
- `Custom { name, implementation }` — escape hatch

### Not in enum (carry them on the tail)

**Classic / high-profile:**
Absorb, Affinity (subtype variants), Afflict, Aftermath, Annihilator,
Ascend, Aura swap, Bands with other, Battle cry, Bestow, Bloodthirst,
Buyback, Cascade, Champion, Changeling, Cipher, Crew, Cumulative upkeep,
Dash, Daybound/Nightbound, Devour, Dredge, Echo, Embalm, Emerge, Entwine,
Epic, Evoke, Evolve, Exalted, Exert, Exploit, Extort, Fabricate, Fading,
Flanking, Flip, Forecast, Fortify, Frenzy, Graft, Gravestorm, Haunt,
Hideaway, Horsemanship, Infect, Jump-start, Level up, Living weapon,
Meld, Mentor, Miracle, Modular, Multikicker, Mutate, Ninjutsu, Offering,
Overload, Persist, Poisonous, Populate, Provoke, Prowl, Rampage, Rebound,
Recover, Reinforce, Renown, Replicate, Retrace, Riot, Ripple, Scavenge,
Shadow, Soulbond, Soulshift, Spectacle, Splice, Split second, Storm,
Sunburst, Suspend, Transfigure, Transmute, Typecycling, Umbra armor,
Undying, Unearth, Unleash, Vanishing, Wither

**Recent (2023–2026):**
Plot, Saddle, Solve a case, Disguise, Cloak, Craft, Collect evidence,
Suspect, Forage, Impending

### Strategy for the tail
- Don't preemptively add `KeywordAbility::*` variants for every tail item.
- Add a variant when the first card using that keyword is being
  implemented — the card's effect builder defines the mechanics as a
  composition of existing primitives, and the keyword enum variant
  unlocks cross-card queries (e.g. "creatures with Infect" for a global
  check).
- For one-shot mechanics that don't need cross-card queries,
  `KeywordAbility::Custom` is the escape hatch.

---

## Immediate punch list (prioritized)

High-leverage, unlocks many cards:
1. **Protection — color-specific targeting** — thread the source
   object through the targeting API so `ProtectionQuality::Color`
   matches work on the targeting side (currently only on damage/
   attach/block).
2. **Ward — real trigger + agent decision** — upgrade from "always
   counter" to "may pay cost" once decision yields land.
3. **Reveal** — standalone `Effect::Reveal { player, card }` so
   non-tutor reveals don't require piggybacking on `TutorToHand`.
4. **Phase out / Phase in** — `Effect::PhaseOut`, `PhaseIn` consuming
   the existing `PermanentStatus::phased_out` field.
5. **FlipCoin / RollDie** — yield a random-choice action; deterministic
   seeded result in Phase 1.
6. **Vote** — Council's dilemma; expressible once agent decisions
   generalize.

Medium-leverage, fewer but important cards:
7. **Cascade / Discover** — depend on `CastFromHandFree` plus
   library-exile-until + predicate. The cast primitive is done;
   a helper `Effect::CascadeUntil { ... }` would close this.
8. **Storm** — spell copy count tracked in state.
9. **Delirium / Threshold / Revolt** — graveyard/turn-state
   conditions expressible but no dedicated `Condition` variants.

Low-leverage, niche:
10. Fateseal, Clash, Planeswalk, Set in motion, Venture, Meld,
    Monstrosity, Bolster, Amass, Support, Investigate (macro only).

---

## Known deliberate simplifications

These are Phase 1 compromises to be revisited when the agent-decision
yield framework lands:

| Behavior | Current | Target |
|---|---|---|
| Ward payment | never pays → countered | agent chooses pay-or-counter |
| Scry / Surveil reorder | keep-on-top / mill-all | agent picks per-card disposition |
| Tutor choice | lowest id | agent picks |
| Search in graveyard | lowest id | agent picks |
| Goad "must attack if able" | hint only | requirement enforced at enumeration |
| Protection color targeting | only `Everything` fully rejected | filter by source color |
| Legend rule tiebreak | keep lowest id | agent picks |
| London mulligan bottom picks | lowest ids | agent picks |
