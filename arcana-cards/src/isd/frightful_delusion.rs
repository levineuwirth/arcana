//! Frightful Delusion — `{2}{U}` instant. "Counter target spell
//! unless its controller pays {1}. That player discards a card."
//!
//! The unconditional "that player discards a card" follows the soft
//! counter. We model both: the soft counter on the spell, and a
//! discard by the spell's controller (read as a target player is not
//! available — the discard targets the spell's controller, which the
//! catalog cannot address without a player target). Discard is
//! gapped.

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
    let name = reg.interner_mut().intern("Frightful Delusion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell unless its controller pays {1}. That player discards a card.".into(),
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

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "That player discards a card" — the spell's controller is
    // not addressable as a player target by the catalog.
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{1}").expect("valid cost"),
    }]
}
