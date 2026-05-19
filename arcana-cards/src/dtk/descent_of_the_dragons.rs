//! Descent of the Dragons — `{4}{R}{R}` sorcery. "Destroy any number
//! of target creatures. For each creature destroyed this way, its
//! controller creates a 4/4 red Dragon creature token with flying."
//!
//! # GAP
//! "For each creature destroyed this way, its controller creates a
//! token" requires knowing the controller of each destroyed creature.
//! The resolver can destroy all targeted creatures, but cannot create
//! tokens controlled by each destroyed creature's original controller.
//! Token creation is noted as a gap; the ForEach destroy is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Descent of the Dragons");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy any number of target creatures. For each creature destroyed this way, its controller creates a 4/4 red Dragon creature token with flying.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Creature,
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
    // GAP: per-destroyed-creature-controller token creation not expressible
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
