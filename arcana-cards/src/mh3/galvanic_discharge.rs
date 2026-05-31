//! Galvanic Discharge — `{R}` instant. "Choose target creature or
//! planeswalker. You get {E}{E}{E} (three energy counters), then you
//! may pay any amount of {E}. Galvanic Discharge deals that much
//! damage to that permanent."
//!
//! The energy-gain half is expressible (`Effect::GainEnergy`). The
//! payoff — paying a variable amount of {E} as part of resolution and
//! dealing damage equal to the amount paid — is NOT: there is no
//! optional-energy-spend-during-resolution primitive, and the damage
//! amount is dynamic on that spend. So the targeted variable-damage
//! half is GAP'd rather than emitting a wrong fixed amount.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galvanic Discharge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature or planeswalker. You get {E}{E}{E} \
                   (three energy counters), then you may pay any amount of {E}. \
                   Galvanic Discharge deals that much damage to that permanent."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
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
    // GAP: paying a variable amount of {E} during resolution and dealing
    // damage equal to the amount paid is not expressible — no
    // optional-energy-spend primitive and the damage amount is dynamic
    // on that spend. Only the energy gain is emitted.
    vec![Effect::GainEnergy {
        player: entry.controller,
        amount: 3,
    }]
}
