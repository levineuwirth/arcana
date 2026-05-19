//! Anticognition — `{1}{U}` instant. "Counter target creature or planeswalker
//! spell unless its controller pays {2}. If an opponent has eight or more
//! cards in their graveyard, instead counter that spell, then scry 2."
//!
//! # GAP: conditional counter-or-scry based on opponent graveyard size
//! The engine has no `Effect::Conditional` condition variant that checks an
//! opponent's graveyard size.  Best-effort: emit the unconditional soft-counter
//! only; the scry-2 bonus branch is dropped.

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
    let name = reg.interner_mut().intern("Anticognition");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target creature or planeswalker spell unless its controller pays {2}. If an opponent has eight or more cards in their graveyard, instead counter that spell, then scry 2.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::SORCERY))
                ),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: conditional branch (scry 2 when opponent has 8+ cards in graveyard)
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{2}").expect("valid cost"),
    }]
}
