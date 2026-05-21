//! Nissa's Renewal — `{5}{G}` sorcery. "Search your library for up to three
//! basic land cards, put them onto the battlefield tapped, then shuffle. You
//! gain 7 life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa's Renewal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to three basic land cards, put them onto the battlefield tapped, then shuffle. You gain 7 life.".into(),
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
    // Each TutorToBattlefield fetches one land card; repeated for "up to three".
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToBattlefield { player: entry.controller, filter: land_filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: entry.controller, filter: land_filter.clone(), tapped: true },
        Effect::TutorToBattlefield { player: entry.controller, filter: land_filter, tapped: true },
        Effect::GainLife { player: entry.controller, amount: 7 },
    ]
}
