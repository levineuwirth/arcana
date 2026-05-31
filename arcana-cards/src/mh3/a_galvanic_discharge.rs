//! A-Galvanic Discharge — `{R}` instant. "Choose target creature or
//! planeswalker. You get {E}{E}, then you may pay any amount of {E}.
//! Galvanic Discharge deals that much damage to that permanent."
//!
//! The energy GAIN ({E}{E}) is expressible via `Effect::GainEnergy`.
//! The follow-up "pay any amount of {E}; deal that much damage" needs an
//! energy-as-cost / variable-X payment that is not yet modeled, so the
//! damage portion is GAP-ed (a literal damage amount would be a wrong
//! card).

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
    let name = reg.interner_mut().intern("A-Galvanic Discharge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature or planeswalker. You get {E}{E}, then you may pay any amount of {E}. Galvanic Discharge deals that much damage to that permanent.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
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
    // GAP: "pay any amount of {E}; deal that much damage" — spending energy
    // as a cost and the resulting variable-X damage are not expressible.
    // The energy gain is emitted faithfully.
    vec![Effect::GainEnergy { player: entry.controller, amount: 2 }]
}
