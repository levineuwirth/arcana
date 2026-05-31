//! Reckless Impulse — `{1}{R}` sorcery. "Exile the top two cards of your
//! library. Until the end of your next turn, you may play those cards."
//! Impulse-draw (CR 601.3e): the two cards are exiled and flagged
//! playable.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Impulse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile the top two cards of your library. Until the end of your next turn, you may play those cards.".into(),
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
    // GAP: "until the end of your NEXT turn" — ImpulseExile clears the
    // play-permission at end of THIS turn, not the next. The exile +
    // play-this-turn portion is modeled faithfully.
    vec![Effect::ImpulseExile { player: entry.controller, count: 2 }]
}
