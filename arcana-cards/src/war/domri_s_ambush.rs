//! Domri's Ambush — `{R}{G}` sorcery. Put a +1/+1 counter on target
//! creature you control. Then that creature deals damage equal to its
//! power to target creature or planeswalker you don't control.

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
use arcana_core::types::{CardId, CounterKind, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri's Ambush");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control. Then that creature deals damage equal to its power to target creature or planeswalker you don't control.".into(),
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
                            ObjectFilter::permanent()
                                .with_types_any(TypeLine(
                                    TypeLine::CREATURE | TypeLine::PLANESWALKER,
                                ))
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

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    let Some(TargetChoice::Object(my_id)) = targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(their_id)) = targets.get(1) else { return Vec::new(); };
    let my_id = *my_id;
    let their_id = *their_id;
    let power_after = (script::power_of(state, my_id) + 1).max(0) as u32;
    vec![
        Effect::AddCounters {
            target: my_id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DealDamage {
            source: my_id,
            target: DamageTarget::Object(their_id),
            amount: power_after,
        },
    ]
}
