//! Punish Ignorance — `{W}{U}{U}{B}` instant. "Counter target spell.
//! Its controller loses 3 life and you gain 3 life."
//!
//! The spell's controller is not readable from the effect surface, so
//! "its controller loses 3 life" is a GAP; counter + you gain 3 life
//! are emitted.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Punish Ignorance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell. Its controller loses 3 life and you gain 3 life.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(ObjectFilter::default()),
                count: TargetCount::Exactly(1),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: spell's controller is not readable here, so "its
    // controller loses 3 life" is omitted.
    vec![
        Effect::Counter { target: *id },
        Effect::GainLife { player: entry.controller, amount: 3 },
    ]
}
