//! Entish Restoration — `{2}{G}` instant, "As an additional cost to cast this
//! spell, sacrifice a land. Search your library for up to two basic land cards
//! and put them onto the battlefield tapped. If you controlled seven or more
//! lands as this spell resolved, search for up to three basic land cards
//! instead. Then shuffle."
//!
//! GAP: sacrifice a land as additional cost (no additional-cost payment effect);
//! conditional "up to three vs two" based on land count at resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Entish Restoration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "As an additional cost to cast this spell, sacrifice a land. Search your library for up to two basic land cards and put them onto the battlefield tapped. If you controlled seven or more lands as this spell resolved, search for up to three basic land cards instead.".into(),
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
    // GAP: sacrifice a land as additional cost;
    // conditional up-to-three vs up-to-two based on land count
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
    ]
}
