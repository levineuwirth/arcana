//! Knockout Maneuver — `{2}{G}` sorcery. "Put a +1/+1 counter on
//! target creature you control, then it deals damage equal to its
//! power to target creature an opponent controls."

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knockout Maneuver");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on target creature you control, then it deals damage equal to its power to target creature an opponent controls.".into(),
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
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
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

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut ids = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            ids.push(*id);
        }
    }
    if ids.len() < 2 {
        return Vec::new();
    }
    let pumped = ids[0];
    let foe = ids[1];
    // Power after the counter, since the counter is added first.
    let dmg = (script::power_of(state, pumped) + 1).max(0) as u32;
    vec![
        Effect::AddCounters {
            target: pumped,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DealDamage {
            source: pumped,
            target: DamageTarget::Object(foe),
            amount: dmg,
        },
    ]
}
