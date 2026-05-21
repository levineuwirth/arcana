//! Intruder's Inquisition — `{B}{R}` sorcery. "Target creature you
//! control deals damage equal to its power to target creature an
//! opponent controls. If excess damage was dealt to a creature this
//! way, its controller discards a card with the greatest mana value
//! among cards in their hand."
//!
//! Modeled as the first creature dealing power-equal damage to the
//! second. The excess-damage discard rider is not expressible — GAP.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Intruder's Inquisition");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its power to target creature an opponent controls. If excess damage was dealt to a creature this way, its controller discards a card with the greatest mana value among cards in their hand.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(src)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(victim)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    let amount = script::power_of(state, *src).max(0) as u32;
    // GAP: excess-damage discard rider is not expressible.
    vec![Effect::DealDamage {
        source: *src,
        target: DamageTarget::Object(*victim),
        amount,
    }]
}
