//! Fated Retribution — `{4}{W}{W}{W}` instant. "Destroy all
//! creatures and planeswalkers. If it's your turn, scry 2."
//!
//! The "if it's your turn" condition is not testable via the
//! catalog; we destroy all creatures (planeswalkers not separately
//! filterable) and unconditionally scry 2.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fated Retribution");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures and planeswalkers. If it's your turn, scry 2.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: planeswalkers not separately filterable; "if it's your
    // turn" not testable — scry applied unconditionally.
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent {
                target: NULL_OBJECT_ID,
            }),
        },
        Effect::Scry {
            player: entry.controller,
            count: 2,
        },
    ]
}
