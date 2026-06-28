//! Traverse Eternity — `{2}{U}{U}` sorcery. "Draw cards equal to the greatest
//! mana value among historic permanents you control. (Artifacts, legendaries,
//! and Sagas are historic.)"

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Traverse Eternity");
    // intern "Saga" so we can look it up at resolve time for the Saga subtype filter
    let _saga = reg.interner_mut().intern("Saga");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw cards equal to the greatest mana value among historic permanents you control. (Artifacts, legendaries, and Sagas are historic.)".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let you = entry.controller;
    let filter_artifact = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let filter_legendary = ObjectFilter::permanent()
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))
        .controlled_by(ControllerConstraint::You);
    let max_art = script::max_cmc_of(state, &filter_artifact, you);
    let max_leg = script::max_cmc_of(state, &filter_legendary, you);
    let max_saga = if let Some(s) = reg.interner().lookup("Saga") {
        let filter = ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_subtypes_any(vec![s]);
        script::max_cmc_of(state, &filter, you)
    } else {
        0
    };
    let n = max_art.max(max_leg).max(max_saga);
    vec![Effect::DrawCards { player: you, count: n }]
}
