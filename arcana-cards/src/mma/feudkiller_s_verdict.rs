//! Feudkiller's Verdict — `{4}{W}{W}` Kindred Sorcery—Giant, "You gain 10 life. Then if you have
//! more life than an opponent, create a 5/5 white Giant Warrior creature token."
//!
//! GAP: Kindred type not a supported TypeLine const — using SORCERY; comparative life-total
//! conditional for token creation not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feudkiller's Verdict");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain 10 life. Then if you have more life than an opponent, create a 5/5 white Giant Warrior creature token.".into(),
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
    // GAP: comparative life-total conditional for token creation not expressible
    vec![Effect::GainLife { player: entry.controller, amount: 10 }]
}
