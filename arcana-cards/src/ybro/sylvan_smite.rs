//! Sylvan Smite — `{1}{G}` instant. "Put a +1/+1 counter on target
//! creature you control if you weren't the starting player. Then that
//! creature deals damage equal to its power to target creature you
//! don't control."

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
    let name = reg.interner_mut().intern("Sylvan Smite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on target creature you control if you weren't the starting player. Then that creature deals damage equal to its power to target creature you don't control.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
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
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(own)) = targets.next() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(foe)) = targets.next() else {
        return Vec::new();
    };
    // GAP: "if you weren't the starting player" cannot be tested — the
    // +1/+1 counter is omitted; the fight-style damage is unconditional.
    let power = script::power_of(state, *own).max(0) as u32;
    vec![Effect::DealDamage {
        source: *own,
        target: DamageTarget::Object(*foe),
        amount: power,
    }]
}
