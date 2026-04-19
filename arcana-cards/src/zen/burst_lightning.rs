//! Burst Lightning — `{R}`, Kicker `{4}`. "Burst Lightning deals 2
//! damage to any target. If Burst Lightning was kicked, it deals 4
//! damage to that target instead." Zendikar common. The seed's
//! touchstone for CR 702.32 Kicker: an optional mana additional
//! cost paid at cast time, flagged on the stack entry, consulted at
//! resolution to pick a kicked vs unkicked rider.
//!
//! # Rules references
//!
//! * CR 702.32a — Kicker is an optional additional mana cost paid as
//!   the caster casts the spell. It doesn't add a second spell or
//!   ability — the same spell resolves with extra effects.
//! * CR 702.32c — Whether a spell was kicked is a property of the
//!   cast, not of the card; `StackEntry::kicked` records this once
//!   per cast and resolution reads it off the entry.
//! * CR 601.2f — "additional" costs (kicker) are distinct from
//!   "alternative" costs (flashback). Kicker composes with the
//!   printed cost; flashback replaces it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burst Lightning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        // Kicker lives on the card's base characteristics; layer-
        // aware lookups in `legal_actions` and `apply_cast_spell`
        // resolve this via `effective_keywords`, so granted-kicker
        // (if any such mechanic arrives) would compose.
        keywords: vec![KeywordAbility::Kicker(
            ManaCost::parse("{4}").expect("valid kicker cost"),
        )],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Burst Lightning deals 2 damage to any target. If \
                       Burst Lightning was kicked, it deals 4 damage to \
                       that target instead.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let amount = if entry.kicked { 4 } else { 2 };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount,
    }]
}
