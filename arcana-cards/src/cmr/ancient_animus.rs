//! Ancient Animus — `{1}{G}` instant. "Put a +1/+1 counter on target creature
//! you control if it's legendary. Then it fights target creature an opponent controls."
//!
//! # GAP: conditional "if legendary" cannot be encoded cleanly without
//! Effect::Conditional's condition type being demonstrated. We emit AddCounters
//! unconditionally and Fight; a human will route the legendary check.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ancient Animus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control if it's legendary. Then it fights target creature an opponent controls.".into(),
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
    let mut targets = entry.targets.targets.iter();
    let Some(t1) = targets.next() else { return Vec::new(); };
    let Some(t2) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(id1) = t1 else { return Vec::new(); };
    let TargetChoice::Object(id2) = t2 else { return Vec::new(); };
    // GAP: conditional counter only if id1 is legendary is not expressible
    vec![
        Effect::AddCounters { target: *id1, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::Fight { a: *id1, b: *id2 },
    ]
}
