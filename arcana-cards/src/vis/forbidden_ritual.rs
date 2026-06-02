//! Forbidden Ritual — `{2}{B}{B}` sorcery. "Sacrifice a nontoken
//! permanent. If you do, target opponent loses 2 life unless that
//! player sacrifices a permanent of their choice or discards a card.
//! You may repeat this process any number of times."
//!
//! Only the leading "sacrifice a nontoken permanent" is expressible
//! with the current catalog. The "loses 2 life UNLESS that player
//! sacrifices/discards" is a player-choice menu branch with no
//! matching primitive, and the "repeat any number of times" loop is
//! likewise unmodeled — both are GAP'd below.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forbidden Ritual");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Sacrifice a nontoken permanent. If you do, target opponent loses 2 life unless that player sacrifices a permanent of their choice or discards a card. You may repeat this process any number of times.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "target opponent loses 2 life UNLESS that player sacrifices a
    // permanent of their choice OR discards a card" is a player-choice
    // menu branch (loses-life-unless-alternative) with no Effect variant,
    // and "you may repeat this process any number of times" is an
    // unmodeled repeat loop. Only the leading sacrifice is emitted.
    vec![Effect::Sacrifice {
        player: entry.controller,
        filter: ObjectFilter::permanent().nontoken(),
        count: 1,
    }]
}
