//! Quenchable Fire — `{3}{R}` sorcery. "Quenchable Fire deals 3
//! damage to target player or planeswalker. It deals an additional 3
//! damage to that player or planeswalker at the beginning of your
//! next upkeep step unless that player or that planeswalker's
//! controller pays {U} before that step."
//!
//! Note: planeswalkers aren't modelled; target is a player. The
//! delayed second 3 damage with a {U} escape is not expressible
//! (DelayedAction has no damage action); only the immediate 3 damage
//! is emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quenchable Fire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Quenchable Fire deals 3 damage to target player or planeswalker. It deals an additional 3 damage to that player or planeswalker at the beginning of your next upkeep step unless that player or that planeswalker's controller pays {U} before that step.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: delayed additional 3 damage next upkeep with a {U} escape
    // is not expressible (no delayed-damage action).
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(*p),
        amount: 3,
    }]
}
