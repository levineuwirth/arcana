//! Showstopping Surprise — `{3}{R}{R}` instant. "Choose target creature you
//! control. Turn it face up if it's face down. Then it deals damage equal to
//! its power to each other creature."
//!
//! # GAP: Turn face-up effect not in Effect catalog
//! The damage-to-each-other-creature portion is expressible via ForEach + script.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement, TargetChoice};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::events::DamageTarget;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Showstopping Surprise");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control. Turn it face up if it's face down. Then it deals damage equal to its power to each other creature.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::You)),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: turn face-up effect not in Effect catalog
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(source_id) = target else { return Vec::new(); };
    let power = script::power_of(state, *source_id).max(0) as u32;
    if power == 0 {
        return Vec::new();
    }
    // Each other creature on the battlefield
    let others: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    ).into_iter().filter(|id| id != source_id).collect();
    others.into_iter().map(|id| Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: power,
    }).collect()
}
