//! Hunter's Edge — `{3}{G}` sorcery. "Put a +1/+1 counter on target creature
//! you control. Then that creature deals damage equal to its power to target
//! creature you don't control."
//!
//! # GAP: DamageEqualToPower — no Effect variant for one-sided 'deals damage
//!   equal to its own power'; Fight is the closest approximation (symmetric)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hunter's Edge");
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
                text: "Put a +1/+1 counter on target creature you control. Then that creature deals damage equal to its power to target creature you don't control.".into(),
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
    // GAP: DamageEqualToPower — no Effect variant for one-sided 'deals damage equal to its power'
    let mut targets = entry.targets.targets.iter();
    let Some(t1) = targets.next() else { return Vec::new(); };
    let Some(t2) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(id1) = t1 else { return Vec::new(); };
    let TargetChoice::Object(id2) = t2 else { return Vec::new(); };
    vec![
        Effect::AddCounters { target: *id1, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::Fight { a: *id1, b: *id2 },
    ]
}
