//! Seeds of Innocence — `{1}{G}{G}` sorcery. "Destroy all artifacts. They
//! can't be regenerated. The controller of each of those artifacts gains life
//! equal to its mana value."
//!
//! GAP: Life gain equal to each artifact's mana value for its controller is
//! not expressible (no per-object CMC query or per-object controller life
//! gain). The "can't be regenerated" clause has no Effect representation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seeds of Innocence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all artifacts. They can't be regenerated. The controller of each of those artifacts gains life equal to its mana value.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let ids = script::ids_matching(state, &filter, entry.controller);
    // GAP: life gain per artifact equal to its mana value for its controller; can't-regenerate clause
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
