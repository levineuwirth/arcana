//! Chandra's Ignition — `{3}{R}{R}` sorcery. "Target creature you
//! control deals damage equal to its power to each other creature and
//! each opponent."

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
    let name = reg.interner_mut().intern("Chandra's Ignition");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its power to each other creature and each opponent.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(src) = target else { return Vec::new(); };
    let amount = script::power_of(state, *src).max(0) as u32;
    let mut effects = Vec::new();
    for id in script::ids_matching(state, &ObjectFilter::creature(), entry.controller) {
        if id == *src {
            continue;
        }
        effects.push(Effect::DealDamage {
            source: *src,
            target: DamageTarget::Object(id),
            amount,
        });
    }
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::DealDamage {
            source: *src,
            target: DamageTarget::Player(opp),
            amount,
        });
    }
    effects
}
