//! Grapple with the Past — `{1}{G}` instant. "Mill three cards, then
//! you may return a creature or land card from your graveyard to your
//! hand." Returning a creature OR land from grave is the
//! ReturnFromGraveyardToHand target choice; the may + creature-or-land
//! union isn't directly modelable. Mill 3 emitted; GAP the optional
//! union retrieval.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grapple with the Past");
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
                text: "Mill three cards, then you may return a creature or land card from your graveyard to your hand.".into(),
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
    // GAP: optional "you may return a creature OR land card from your graveyard" not modeled (no in-resolution may + union TargetFilter).
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
