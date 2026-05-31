//! Exploding Borders — `{2}{R}{G}` sorcery. "Domain — Search your
//! library for a basic land card, put that card onto the battlefield
//! tapped, then shuffle. Exploding Borders deals X damage to target
//! player or planeswalker, where X is the number of basic land types
//! among lands you control."
//!
//! The ramp half is expressible: tutor a basic land to the
//! battlefield tapped (shuffle is automatic). The damage half scales
//! with Domain (number of basic land types among lands you control),
//! which has no `script::` helper — GAP'd rather than emitting a wrong
//! fixed literal.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exploding Borders");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle. Exploding Borders deals X damage to target player or planeswalker, where X is the number of basic land types among lands you control.".into(),
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
    // GAP: the X-damage half scales with Domain (number of basic land
    // types among lands you control); no script:: helper exists for
    // counting distinct basic land types, so the dynamic damage cannot
    // be computed. Only the basic-land ramp is emitted.
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(arcana_core::types::SupertypeSet::new().with(arcana_core::types::SupertypeSet::BASIC)),
        tapped: true,
    }]
}
