//! The Elderspell — `{B}{B}` sorcery. "Destroy any number of target
//! planeswalkers. Choose a planeswalker you control. Put two loyalty
//! counters on it for each planeswalker destroyed this way."
//!
//! The destroy clause is expressible: "any number of target
//! planeswalkers" is `TargetCount::Any` over a planeswalker filter, and
//! each chosen target is destroyed.
//!
//! GAP: the loyalty-counter rider ("put two loyalty counters on a
//! planeswalker you control for each planeswalker destroyed this way")
//! is not expressible — there is no resolution-time count of
//! "permanents destroyed by this effect", and the recipient is a
//! player-chosen planeswalker rather than a card target read from the
//! stack entry. Emitting a fixed AddCounters would be a materially
//! wrong card, so only the destroy half is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Elderspell");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy any number of target planeswalkers. Choose a planeswalker you control. Put two loyalty counters on it for each planeswalker destroyed this way.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()),
                ),
                count: TargetCount::Any,
                controller: None,
            }],
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
    // Destroy each chosen target planeswalker.
    let ids: Vec<_> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(*id),
            _ => None,
        })
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    // GAP: the "put two loyalty counters on a chosen planeswalker you
    // control for each planeswalker destroyed this way" rider is not
    // expressible — no resolution-time destroyed-count and the
    // recipient is a player choice, not a stack target.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
