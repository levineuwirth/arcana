//! Druidic Ritual — `{2}{G}` sorcery. "You may mill three cards.
//! Then return up to one creature card and up to one land card from
//! your graveyard to your hand." Emits the mandatory mill; the
//! graveyard returns require two more targets/zones which the spec
//! frames as part of resolution — GAP the targeted returns.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Druidic Ritual");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You may mill three cards. Then return up to one creature card and up to one land card from your graveyard to your hand.".into(),
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
    // GAP: optional resolution-time choose-from-graveyard for one creature and one land — needs in-resolution choice, not target reqs.
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
