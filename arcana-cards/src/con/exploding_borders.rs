//! Exploding Borders — `{2}{R}{G}` sorcery, "Domain — Search your library
//! for a basic land card, put that card onto the battlefield tapped, then
//! shuffle. Exploding Borders deals X damage to target player or
//! planeswalker, where X is the number of basic land types among lands you
//! control."
//!
//! # GAP
//! * GAP: TutorToBattlefield has no `tapped: true` support (only `tapped: false`)
//! * GAP: domain X damage (damage proportional to distinct basic land types you control)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
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
                target_requirements: vec![TargetRequirement::target_player()],
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
        // TutorToBattlefield with tapped:false is the closest available; GAP: tapped variant
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: false,
        },
        // GAP: domain X damage (damage proportional to distinct basic land types you control)
    ]
}
