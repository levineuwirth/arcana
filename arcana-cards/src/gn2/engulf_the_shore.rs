//! Engulf the Shore — `{3}{U}` instant. Return to their owners' hands
//! all creatures with toughness less than or equal to the number of
//! Islands you control.

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
    let name = reg.interner_mut().intern("Engulf the Shore");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return to their owners' hands all creatures with toughness less than or equal to the number of Islands you control.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let islands = script::count_matching(
        state,
        &script::subtype_filter(reg, "Island").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let filter = ObjectFilter::creature().with_max_toughness(islands as i32);
    let ids = script::ids_matching(state, &filter, entry.controller);
    ids.into_iter()
        .map(|id| Effect::ReturnToHand { target: id })
        .collect()
}
