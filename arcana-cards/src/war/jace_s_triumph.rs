//! Jace's Triumph — `{2}{U}` sorcery. "Draw two cards. If you control
//! a Jace planeswalker, draw three cards instead."

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
    let name = reg.interner_mut().intern("Jace's Triumph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. If you control a Jace planeswalker, draw three cards instead.".into(),
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
    // 'Jace planeswalker you control' — a planeswalker permanent with
    // subtype Jace under your control.
    let mut jace_filter = script::subtype_filter(reg, "Jace");
    jace_filter = jace_filter
        .with_types(TypeLine::PLANESWALKER.into())
        .controlled_by(ControllerConstraint::You);
    let has_jace = script::count_matching(state, &jace_filter, entry.controller) > 0;
    let count = if has_jace { 3 } else { 2 };
    vec![Effect::DrawCards { player: entry.controller, count }]
}
