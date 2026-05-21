//! Shard Convergence — `{3}{G}` sorcery. Search your library for a
//! Plains, an Island, a Swamp, and a Mountain. Put them into your hand.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shard Convergence");
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a Plains card, an Island card, a Swamp card, and a Mountain card. Reveal those cards, put them into your hand, then shuffle.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::TutorToHand { player: entry.controller, filter: script::subtype_filter(reg, "Plains"), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: script::subtype_filter(reg, "Island"), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: script::subtype_filter(reg, "Swamp"), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: script::subtype_filter(reg, "Mountain"), reveal: true },
    ]
}
