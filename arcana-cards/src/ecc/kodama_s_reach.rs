//! Kodama's Reach — `{2}{G}` sorcery — Arcane. "Search your library for up
//! to two basic land cards, reveal those cards, put one onto the battlefield
//! tapped and the other into your hand, then shuffle." Catalog has tutor-to-
//! battlefield(tapped) and tutor-to-hand on separate basic-land filters; we
//! emit one of each.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kodama's Reach");
    let arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(arcane);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to two basic land cards, reveal those cards, put one onto the battlefield tapped and the other into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no "basic-only" supertype refinement on ObjectFilter; using plain Land.
    let land = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land.clone(),
            tapped: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: land,
            reveal: true,
        },
    ]
}
