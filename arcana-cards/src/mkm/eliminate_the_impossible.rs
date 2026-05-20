//! Eliminate the Impossible — `{1}{U}` instant. "Investigate.
//! Creatures your opponents control get -2/-0 until end of turn. If
//! any of them are suspected, they're no longer suspected."
//!
//! The Clue token's activated ability and the suspected-clearing
//! rider can't be modeled; a Clue artifact token is created and the
//! mass debuff is applied via ForEach.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eliminate the Impossible");
    let _clue = reg.interner_mut().intern("Clue");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Investigate. Creatures your opponents control get -2/-0 until end of turn. If any of them are suspected, they're no longer suspected.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    let clue = reg.interner().lookup("Clue").expect("Clue interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clue);
    let token = TokenDefinition {
        name: clue,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: Clue token's "{2}, Sacrifice: Draw a card" activated
    // ability and the suspected-clearing rider.
    vec![
        Effect::CreateToken { controller: entry.controller, token },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: -2,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
    ]
}
