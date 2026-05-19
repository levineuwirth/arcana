//! Coming Attraction — `{2}{G}` sorcery, "Search your library for a basic land card,
//! put it onto the battlefield tapped, then shuffle. Open an Attraction."
//!
//! GAP: "Open an Attraction" (put top card of Attraction deck onto battlefield) is not
//! expressible with available Effect variants.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coming Attraction");
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
                text: "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle. Open an Attraction.".into(),
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
    // GAP: TutorToBattlefield with tapped=true is not available in the catalog (tapped: false only shown)
    // GAP: Open an Attraction (Attraction deck mechanic) not expressible
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: false,
        },
    ]
}
