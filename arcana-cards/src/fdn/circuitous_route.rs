//! Circuitous Route — `{3}{G}` sorcery. "Search your library for up
//! to two basic land cards and/or Gate cards, put them onto the
//! battlefield tapped, then shuffle." Catalog has a single-tutor
//! variant; we emit two land tutors and GAP the Gate component.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Circuitous Route");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to two basic land cards and/or Gate cards, put them onto the battlefield tapped, then shuffle.".into(),
            target_requirements: vec![] as Vec<TargetRequirement>,
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
    // GAP: "and/or Gate cards" branch + "up to two" optional second tutor not modeled.
    let land = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::TutorToBattlefield { player: entry.controller, filter: land.clone(), tapped: true },
        Effect::TutorToBattlefield { player: entry.controller, filter: land, tapped: true },
    ]
}
