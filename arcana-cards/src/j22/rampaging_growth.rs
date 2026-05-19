//! Rampaging Growth — `{3}{G}` instant. "Search your library for a basic land
//! card, put it onto the battlefield, then shuffle. Until end of turn, that
//! land becomes a 4/3 Insect creature with reach and haste. It's still a land."
//!
//! # GAP: animating the tutored land (SetBasePT + type change to also be a
//! creature) with keyword grants requires chaining on the tutored object's id,
//! which is not returned by TutorToBattlefield. The tutor is implemented; the
//! animate rider is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rampaging Growth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a basic land card, put it onto the battlefield, then shuffle. Until end of turn, that land becomes a 4/3 Insect creature with reach and haste. It's still a land.".into(),
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
    // GAP: animate the tutored land (type addition + SetBasePT + keywords) not expressible;
    // TutorToBattlefield does not return the new object's id for further effects
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        tapped: false,
    }]
}
