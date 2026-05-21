//! New Way Forward — `{2}{U}{R}{W}` instant. "The next time a source
//! of your choice would deal damage to you this turn, prevent that
//! damage. When damage is prevented this way, New Way Forward deals
//! that much damage to that source's controller and you draw that
//! many cards."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("New Way Forward");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "The next time a source of your choice would deal damage to you this turn, prevent that damage. When damage is prevented this way, New Way Forward deals that much damage to that source's controller and you draw that many cards.".into(),
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
    // Prevent all damage that would be dealt to you this turn (the
    // closest catalog primitive to "the next time a source would deal
    // damage to you").
    vec![Effect::PreventDamage {
        target: DamageTarget::Player(entry.controller),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
    // GAP: "a source of your choice", the one-shot "next time" limit,
    // and the reflected-damage / draw rider keyed to the prevented
    // amount cannot be expressed; only blanket damage prevention to
    // you is applied.
}
