//! Cache Grab — `{1}{G}` instant, "Mill four cards. You may put a permanent
//! card from among the cards milled this way into your hand. If you control
//! a Squirrel or returned a Squirrel card to your hand this way, create a
//! Food token."
//!
//! GAP: 'put a permanent card from among milled cards into your hand' (choice
//! from milled subset) is not in the Effect catalog. Food token creation is
//! also not in the catalog. Best-effort: mill 4.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cache Grab");
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
                text: "Mill four cards. You may put a permanent card from among the cards milled this way into your hand. If you control a Squirrel or returned a Squirrel card to your hand this way, create a Food token.".into(),
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
    // GAP: put a permanent card from milled cards into hand (milled-subset choice) not in catalog
    // GAP: Food token creation not in Effect catalog
    vec![Effect::Mill { player: entry.controller, count: 4 }]
}
