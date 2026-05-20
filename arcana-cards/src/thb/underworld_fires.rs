//! Underworld Fires — `{1}{R}` sorcery. "Underworld Fires deals 1
//! damage to each creature and each planeswalker. If a permanent
//! dealt damage this way would die this turn, exile it instead." The
//! damage to each creature is expressible via ForEach; planeswalkers
//! aren't a script filter and the die-replacement rider has no
//! primitive (GAP-noted, partial).

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
    let name = reg.interner_mut().intern("Underworld Fires");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Underworld Fires deals 1 damage to each creature and each planeswalker. If a permanent dealt damage this way would die this turn, exile it instead.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each planeswalker" is not a script filter, and the
    // "would die -> exile instead" replacement has no primitive; the
    // 1 damage to each creature is modeled.
    let targets = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 1,
        }),
    }]
}
