//! Ray of Ruin — `{4}{B}` sorcery. "Exile target creature, Vehicle,
//! or nonbasic land. Scry 1."
//!
//! Target is a creature or land (Vehicle is an artifact subtype not
//! modeled; the nonbasic restriction on lands is also not a filter
//! predicate). Exile the target, then Scry 1.
//!
//! GAP: "Vehicle" subtype and "nonbasic" land restriction not
//! expressible in the target filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ray of Ruin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature, Vehicle, or nonbasic land. Scry 1.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "Vehicle" subtype / "nonbasic" land restriction not expressible.
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::Scry { player: entry.controller, count: 1 },
    ]
}
