//! Hunt the Weak — `{3}{G}` sorcery. "Put a +1/+1 counter on target
//! creature you control. Then that creature fights target creature you
//! don't control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control. Then that creature fights target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
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
    let Some(TargetChoice::Object(id0)) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(id1)) = entry.targets.targets.get(1) else { return Vec::new(); };
    vec![
        Effect::AddCounters {
            target: *id0,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::Fight { a: *id0, b: *id1 },
    ]
}
