# Keyword rules passes (Pass 3.x)

> **STATUS: Passes 3.1–3.8 COMPLETE** (commits 3d32a0a, bae9519,
> da33990, 128fb2b, d66c8f0, 072dbb8, fec53ba, 8acd186). All
> deferred markers resolved → catalog 1010 → **1023**, every landed
> card attested clean by `verify_catalog.py` against committed
> bytes. The **lone remaining `DEFERRED_KEYWORDS` entry is Warp**
> (cast-time alternative cost — a recognized no-op would
> misrepresent the card); its cards stay honestly quarantined until
> a dedicated pass. Keywords whose choice/mana parts can't yet be
> honestly resolved (Banding, Enlist, Provoke, Soulshift, Scavenge,
> Devour, Amplify, Unleash, Sunburst, Cycling-activation,
> Changeling-all-types, Afterlife-Spirit-subtype) use documented
> deterministic Phase-1 policies — DEBT noted at each site.
> Scoped re-lands now use `land_cards.py --only <idx,...>` (no
> staging dir).

Passes 0–2 are landed: prompt/contract fixes, landwalk (real),
evasion keywords (Fear/Intimidate/Shadow/Horsemanship/Skulk real) +
the 31 deferred markers + the L3-stub honesty guard. The markers
*compile and certify structurally* (L2: characteristic is correct)
but their **rules are not implemented**, so cards carrying them are
quarantined as L3 stubs and never landed.

These passes turn each deferred marker into **real enforced engine
rules**, removing it from `semantic.rs::DEFERRED_KEYWORDS` so cards
that use it can finally land. Ordered by ROI: shared engine hook
first, complexity-from-cheap-to-worst, Banding last.

Per-pass loop (the same one Passes 1–2 used):

1. Engine rule in `arcana-core` (enum payload if parametrized,
   combat/trigger/replacement hook) + unit tests.
2. L2 cert: `structural.rs` `evergreen_variant` / `Expected` so the
   harness asserts the characteristic.
3. Remove the keyword(s) from `semantic.rs::DEFERRED_KEYWORDS`;
   update/extend its tests.
4. `prompt.rs`: move the keyword from the "pre-wired marker" list
   into the implemented surface; add constructor/mapping notes.
5. Smoke-regen a few affected cards via the subagent pipeline →
   `verify_dir` → `land_cards.py` → `verify_catalog.py` attest.
6. Commit the pass.

Gotchas (learned the hard way):
- **Rebuild `verify_dir` after any `semantic.rs`/`structural.rs`
  change** (`cargo build -p arcana-gen --bin verify_dir`). The
  prebuilt binary embeds the old L2/L3 rules; a stale one will
  L3-quarantine cards the pass just made real.
- **Land from a minimal staging dir, not the full run dir.**
  `land_cards.py --force --apply` re-lands *every* passed row; on a
  run dir full of stale sources that's 100+ unintended catalog
  edits. Stage only the pass's regenerated `.rs` + `manifest.jsonl`
  + `verify-report.jsonl` into a tmp dir and land that.

Infra already present (per damage/counter pipeline map):
`CounterKind::{PlusOnePlusOne,MinusOneMinusOne,Poison}`,
`PlayerState::poison_counters`, `GameObject::add_counters`,
`poison>=10` SBA, `replace_damage()` replacement pipeline,
`effective_keywords()/has_keyword()`, parametrized-variant pattern
(`Ward(ManaCost)`, `Landwalk(SmallString)`, `Surveil(u32)`).

---

## Pass 3.1 — combat damage as counters / poison  ← START HERE
Keywords: **Wither**, **Infect**, **Toxic(u8)**
Shared hook: combat-damage application (`combat.rs deal_damage` /
`replace_damage`). All three reuse the existing counter + poison
infra — nothing new to store.

- Wither — combat damage to creatures is dealt as −1/−1 counters
  instead of marked damage.
- Infect — combat damage to creatures as −1/−1 counters; to players
  as poison counters.
- Toxic N — in addition to combat damage, deals N poison to the
  player. Parametrize: `Toxic(u8)`.

## Pass 3.2 — death / return triggers
Keywords: **Undying**, **Persist**, **Afterlife(u32)**
Shared hook: dies-trigger → return-to-battlefield / token creation.

- Undying — dies with no +1/+1 counter → return with one.
- Persist — dies with no −1/−1 counter → return with one.
- Afterlife N — dies → N 1/1 W/B Spirit tokens with flying.

## Pass 3.3 — attack triggers (pump / counters)
Keywords: **Exalted**, **BattleCry**, **Mentor**, **Dethrone**,
**Renown(u32)**, **Enlist**
Shared hook: attackers-declared trigger.

- Exalted — a creature attacks alone → that creature +1/+1 EOT.
- Battle cry — attacks → each *other* attacker +1/+0 EOT.
- Mentor — attacks → +1/+1 counter on a lesser-power attacker.
- Dethrone — attacks the player with most life → +1/+1 counter.
- Renown N — deals combat damage to a player → N +1/+1 counters
  (once).
- Enlist — as it attacks, tap a non-attacking creature; add its
  power.

## Pass 3.4 — combat statics (block-time pump / debuff)
Keywords: **Flanking**, **Rampage(u8)**, **Bushido(u8)**,
**Provoke**
Shared hook: blockers-declared.

- Flanking — blocked by a non-flanking creature → that blocker
  −1/−1 EOT.
- Rampage N — each blocker beyond the first → +N/+N EOT.
- Bushido N — blocks or is blocked → +N/+N EOT.
- Provoke — attacks → may untap a defending creature; it must block
  it if able.

## Pass 3.5 — ETB counters / scaling
Keywords: **Modular(u8)**, **Graft(u8)**, **Bloodthirst(u8)**,
**Sunburst**, **Amplify(u8)**, **Devour(u8)**, **Unleash**, **Riot**
Shared hook: ETB replacement / enters-with-counters + a choice.

- Modular N — ETB with N +1/+1; dies → move them to an artifact
  creature.
- Graft N — ETB with N +1/+1; another creature ETB → may move one.
- Bloodthirst N — opponent damaged this turn → ETB with N +1/+1.
- Sunburst — ETB with a +1/+1 (or charge) counter per color of mana
  spent.
- Amplify N — ETB → reveal sharing-type cards; +N/+N each.
- Devour N — ETB → sacrifice creatures; N +1/+1 each.
- Unleash — ETB → may add a +1/+1; can't block while it has one.
- Riot — ETB → choose haste or a +1/+1.

## Pass 3.6 — long tail
Keywords: **Soulshift(u8)**, **Scavenge**, **Evolve**,
**Fading(u8)**, **Vanishing(u8)**, **Changeling**, **Banding**

- Soulshift N — dies → may return a Spirit with mana value ≤ N.
- Scavenge — graveyard activated ability: exile for +1/+1 counters.
- Evolve — a creature ETB with greater power *or* toughness →
  +1/+1 counter.
- Fading N — ETB with N fade counters; upkeep remove one, none →
  sacrifice.
- Vanishing N — ETB with N time counters; upkeep remove one, none →
  sacrifice.
- Changeling — is every creature type (characteristic-defining;
  intersects classifier + structural).
- Banding — full CR 702.22 combat banding. **Worst ROI; do last or
  leave permanently deferred.**

## Pass 3.7 — parametrized activated tail (non-marker backlog)
Keywords: **Cycling(ManaCost)**, **Ward(ManaCost)**, **Warp**
The 3 "unsupported-keyword" cards in the unlanded floor. Already
parametrized in the enum; need real activation / trigger rules.
