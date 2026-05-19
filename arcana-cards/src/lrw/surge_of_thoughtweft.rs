//! Surge of Thoughtweft — `{1}{W}` kindred instant — Kithkin.
//! "Creatures you control get +1/+1 until end of turn. If you control a
//! Kithkin, draw a card."
//!
//! Note: Kindred instant type and Kithkin subtype not modeled in TypeLine;
//! treated as plain Instant. GAP: Arcane/Kindred supertypes on Instant.
//! GAP: conditional draw based on controlling a specific creature subtype.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surge of Thoughtweft");
    // GAP: Kindred instant type not modeled
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control get +1/+1 until end of turn. If you control a Kithkin, draw a card.".into(),
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
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects: Vec<Effect> = ids.into_iter().map(|id| Effect::Pump {
        target: id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }).collect();
    // Conditional draw if you control a Kithkin
    let kithkin_count = script::count_matching(
        state,
        &script::subtype_filter(reg, "Kithkin").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if kithkin_count > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count: 1 });
    }
    effects
}
