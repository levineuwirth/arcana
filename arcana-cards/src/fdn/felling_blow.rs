//! Felling Blow — `{2}{G}` sorcery, "Put a +1/+1 counter on target
//! creature you control. Then that creature deals damage equal to
//! its power to target creature an opponent controls."

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
    let name = reg.interner_mut().intern("Felling Blow");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on target creature you control. Then that creature deals damage equal to its power to target creature an opponent controls.".into(),
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

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut ts = entry.targets.targets.iter();
    let Some(TargetChoice::Object(mine)) = ts.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(theirs)) = ts.next() else { return Vec::new(); };
    // Damage uses power after the +1/+1 counter is added.
    let amount = (script::power_of(state, *mine) + 1).max(0) as u32;
    vec![
        Effect::AddCounters {
            target: *mine,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*theirs),
            amount,
        },
    ]
}
