//! Mycoid Resurrection — `{4}{B}{G}` sorcery.
//! "Fathomless descent — Each creature card in your graveyard perpetually gets
//! +X/+X, where X is the number of permanent cards in your graveyard. Then
//! return a creature card from your graveyard to the battlefield."
//!
//! # GAP: Fathomless descent / perpetual +X/+X to cards in graveyard based on
//! permanent-card count in graveyard — no Effect variant for this Arena mechanic.
//! The reanimate rider is expressible but requires a target; text says "return
//! a creature card" without targeting. Best effort: reanimate via TutorToBattlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mycoid Resurrection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Fathomless descent — Each creature card in your graveyard perpetually gets +X/+X, where X is the number of permanent cards in your graveyard. Then return a creature card from your graveyard to the battlefield.".into(),
                target_requirements: vec![],
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
    // GAP: Fathomless descent — perpetual +X/+X to graveyard creature cards based on permanent-card count
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
