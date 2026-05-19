//! Felling Blow — `{2}{G}` sorcery, "Put a +1/+1 counter on target creature
//! you control. Then that creature deals damage equal to its power to
//! target creature an opponent controls."
//!
//! GAP: power-based damage from one specific creature to another (not
//! Fight) not expressible directly; expressed as AddCounters + Fight.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control. Then that creature deals damage equal to its power to target creature an opponent controls.".into(),
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
    if entry.targets.targets.len() < 2 { return Vec::new(); }
    let id_a = match &entry.targets.targets[0] {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    let id_b = match &entry.targets.targets[1] {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    vec![
        Effect::AddCounters { target: id_a, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::Fight { a: id_a, b: id_b },
    ]
}
