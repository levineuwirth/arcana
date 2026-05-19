//! Oko's Hospitality — `{3}{G}{U}` instant. "Creatures you control have base
//! power and toughness 3/3 until end of turn. You may search your library
//! and/or graveyard for a card named Oko, the Trickster, reveal it, and put
//! it into your hand. If you search your library this way, shuffle."
//!
//! GAP: "creatures you control have base power and toughness 3/3 until end of
//! turn" — SetBasePT targets a single ObjectId; no ForEach+SetBasePT variant
//! shown. Approximated with ForEach.
//! GAP: name-based library/graveyard search not expressible.

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
    let name = reg.interner_mut().intern("Oko's Hospitality");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control have base power and toughness 3/3 until end of turn. You may search your library and/or graveyard for a card named Oko, the Trickster, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
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
    // GAP: name-based library/graveyard search not expressible
    let my_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: my_creatures,
        effect: Box::new(Effect::SetBasePT {
            target: NULL_OBJECT_ID,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        }),
    }]
}
