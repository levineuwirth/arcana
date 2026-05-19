//! Grave Strength — `{1}{B}` sorcery. "Choose target creature. Mill three
//! cards, then put a +1/+1 counter on that creature for each creature card
//! in your graveyard."
//!
//! GAP: counter amount = number of creature cards in graveyard — variable
//! counter count requiring graveyard state inspection not expressible with
//! AddCounters (count field must be a static u32). Mill 3 is expressed;
//! the counter placement is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grave Strength");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature. Mill three cards, then put a +1/+1 counter on that creature for each creature card in your graveyard.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: AddCounters count = creature cards in graveyard — variable count not expressible
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
