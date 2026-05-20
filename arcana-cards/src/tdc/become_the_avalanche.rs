//! Become the Avalanche — `{4}{G}{G}` sorcery. "Draw a card for each
//! creature you control with power 4 or greater. Then creatures you
//! control get +X/+X until end of turn, where X is the number of cards
//! in your hand."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Become the Avalanche");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw a card for each creature you control with power 4 or greater. Then creatures you control get +X/+X until end of turn, where X is the number of cards in your hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        entry.controller,
    );
    // After the draw, hand size becomes current hand + n drawn cards
    // (minus this spell, already on the stack).
    let x = (script::hand_size(state, entry.controller) + n) as i32;
    vec![
        Effect::DrawCards { player: entry.controller, count: n },
        Effect::ForEach {
            targets: script::ids_matching(
                state,
                &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                entry.controller,
            ),
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: x,
                toughness: x,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
    ]
}
