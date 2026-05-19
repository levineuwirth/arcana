//! Sylvan Tutor — `{G}` sorcery. "Search your library for a creature
//! card, reveal it, then shuffle and put that card on top."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sylvan Tutor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a creature card, reveal it, then shuffle and put that card on top.".into(),
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
    use arcana_core::effects::Effect;
    use arcana_core::targets::ObjectFilter;
    // TutorToHand with reveal:true is the closest available primitive;
    // the spec says "put on top" which is a library-top tutor.
    // GAP: no TutorToTopOfLibrary variant — using TutorToHand as best effort
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        reveal: true,
    }]
}
