//! Hunt the Weak — `{3}{G}` sorcery. "Put a +1/+1 counter on target
//! creature you control. Then that creature fights target creature you
//! don't control."
//!
//! Two targets: a creature you control (gets the counter and is the
//! aggressor) and a creature you don't control (the defender). The
//! counter is applied first, then the two fight.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hunt the Weak");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on target creature you control. Then that creature fights target creature you don't control.".into(),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut it = entry.targets.targets.iter();
    let Some(TargetChoice::Object(mine)) = it.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(theirs)) = it.next() else { return Vec::new(); };
    vec![
        Effect::AddCounters {
            target: *mine,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::Fight { a: *mine, b: *theirs },
    ]
}
