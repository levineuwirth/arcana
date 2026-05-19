//! Midnight Tilling — `{1}{G}` instant.
//! "Mill four cards, then you may return a permanent card from among them to your hand."
//!
//! GAP: optional return-a-permanent-card-from-among-milled-cards choice;
//! Mill is expressible but the conditional return from the just-milled subset is not.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Midnight Tilling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mill four cards, then you may return a permanent card from among them to your hand.".into(),
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
    vec![
        Effect::Mill { player: entry.controller, count: 4 },
        // GAP: optional return-a-permanent-card-from-the-just-milled-subset to hand;
        // no Effect variant for scoped return from milled cards.
    ]
}
