//! Roil's Retribution — `{3}{W}{W}` instant. 5 damage divided as you
//! choose among any number of target attacking or blocking creatures.
//!
//! The combat-state restriction is enforced via
//! `ObjectFilter::creature().attacking_or_blocking_only()`, and the
//! division uses `Effect::DealDamageDivided`. `UpTo(5)` matches the
//! real ceiling (each chosen target must be assigned at least 1 of
//! the 5 damage). (The player's exact division is a documented
//! fidelity gap; the engine spreads `total` across the chosen
//! targets.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roil's Retribution");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Roil's Retribution deals 5 damage divided as you choose among any number of target attacking or blocking creatures.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().attacking_or_blocking_only(),
                ),
                count: TargetCount::UpTo(5),
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
    let targets: Vec<DamageTarget> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::DealDamageDivided {
        source: entry.source,
        targets,
        total: 5,
    }]
}
