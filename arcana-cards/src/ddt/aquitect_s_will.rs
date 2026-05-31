//! Aquitect's Will — `{U}` Kindred Sorcery — Merfolk. "Put a flood
//! counter on target land. That land is an Island in addition to its
//! other types for as long as it has a flood counter on it. If you
//! control a Merfolk, draw a card."
//!
//! The flood counter and the conditional draw are expressible. The
//! "becomes an Island in addition for as long as it has a flood
//! counter" continuous subtype grant is gated on counter presence and
//! adds a land SUBTYPE (Island), not a card type — neither a
//! counter-tied duration nor a subtype-granting effect is in the
//! catalog, so that clause is GAP'd.

use arcana_core::effects::{Condition, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aquitect's Will");
    let _flood = reg.interner_mut().intern("flood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a flood counter on target land. That land is an Island in addition to its other types for as long as it has a flood counter on it. If you control a Merfolk, draw a card.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let flood = reg
        .interner()
        .lookup("flood")
        .expect("flood interned during register()");
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Named(flood),
            count: 1,
        },
        // GAP: "is an Island in addition for as long as it has a flood
        // counter" — no catalog effect grants a land SUBTYPE, and no
        // duration is gated on a counter's presence. The counter is
        // recorded; the continuous Island grant is dropped.
        Effect::Conditional {
            condition: Condition::ControlPermanentMatching(
                script::subtype_filter(reg, "Merfolk"),
            ),
            then: Box::new(Effect::DrawCards {
                player: entry.controller,
                count: 1,
            }),
            otherwise: None,
        },
    ]
}
