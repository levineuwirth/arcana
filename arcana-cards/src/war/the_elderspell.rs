//! The Elderspell — `{B}{B}` sorcery. "Destroy any number of target
//! planeswalkers. Choose a planeswalker you control. Put two loyalty
//! counters on it for each planeswalker destroyed this way."
//!
//! # GAP
//! - Planeswalker type line constant not in the expressible TypeLine
//!   surface for `with_types`.
//! - "Put two loyalty counters per destroyed planeswalker" requires
//!   counting destroyed permanents at resolution; CounterKind::Loyalty
//!   not listed in catalog.
//! - "Choose a planeswalker you control" as a second target selection
//!   at resolution is inexpressible.
//! Best effort: destroy all targeted permanents via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy any number of target planeswalkers. Choose a planeswalker you control. Put two loyalty counters on it for each planeswalker destroyed this way.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Permanent(
                        arcana_core::targets::ObjectFilter::new()
                            .with_types(TypeLine::PLANESWALKER.into()),
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
    // GAP: loyalty counter kind not in catalog; per-destroyed-planeswalker counter scaling not expressible; choose-a-planeswalker second effect inexpressible
    let ids: Vec<_> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| if let TargetChoice::Object(id) = t { Some(*id) } else { None })
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
