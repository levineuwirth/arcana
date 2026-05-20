//! May of the Machine — `{2}{B}` sorcery. "Destroy target creature or
//! planeswalker. If today's date is May 6th, scry 1. If today's date
//! is May 7th, fateseal 1."
//!
//! Destroy is expressible; the calendar-date-conditional scry/fateseal
//! has no catalog primitive (no real-date predicate; fateseal absent).

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
    let name = reg.interner_mut().intern("May of the Machine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature or planeswalker. If today's \
                   date is May 6th, scry 1. If today's date is May 7th, \
                   fateseal 1."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types_any(TypeLine(
                        TypeLine::CREATURE | TypeLine::PLANESWALKER,
                    )),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: calendar-date-conditional scry/fateseal not expressible (no
    // real-date predicate; fateseal absent from the catalog).
    vec![Effect::DestroyPermanent { target: *id }]
}
