//! Wrath of Leknif — `{1}{W}{W}{U}` sorcery. "Destroy all creatures.
//! They can't be regenerated. Untap up to four lands you control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrath of Leknif");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. They can't be regenerated. Untap up to four lands you control.".into(),
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
    let mut effects: Vec<Effect> = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    )
    .into_iter()
    .map(|id| Effect::DestroyPermanent { target: id })
    .collect();
    // "up to four lands you control" — untap the first four your-controlled tapped lands deterministically.
    let lands = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You)
            .tapped_only(),
        entry.controller,
    );
    for id in lands.into_iter().take(4) {
        effects.push(Effect::Untap { target: id });
    }
    effects
}
