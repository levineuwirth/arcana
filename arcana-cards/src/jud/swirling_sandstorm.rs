//! Swirling Sandstorm — `{3}{R}` sorcery. "Threshold — Swirling
//! Sandstorm deals 5 damage to each creature without flying if there
//! are seven or more cards in your graveyard." Conditional on
//! graveyard size >= 7; otherwise the spell does nothing. The
//! "without flying" subset and the threshold gate are computed at
//! resolution.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swirling Sandstorm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Threshold — Swirling Sandstorm deals 5 damage to each creature without flying if there are seven or more cards in your graveyard.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Note: "without flying" has no ObjectFilter refinement, so the
    // sweep hits all creatures when threshold is met.
    if script::graveyard_size(state, entry.controller) < 7 {
        return Vec::new();
    }
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 5,
        }),
    }]
}
